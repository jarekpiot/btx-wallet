const UNAVAILABLE_GUIDANCE =
  "Shielded note detail is unavailable. Large sends may still work, but a local synced node gives better reliability checks.";
const MAX_SUGGESTED_CONSOLIDATION_RATIO = 0.9;

function unique(items) {
  return [...new Set(items.filter(Boolean))];
}

export function shieldedComplexityLabel(summary, walletReady = false) {
  if (!walletReady) return "Unavailable";
  if (!summary?.available) return "Unknown";
  if (summary.complexity === "high") return "High complexity";
  if (summary.complexity === "medium") return "Moderate complexity";
  return "Low complexity";
}

export function buildShieldedSendGuidance(amount, balance, summary) {
  const guidance = [];
  const requested = Number(amount);

  if (!summary?.available) {
    return [UNAVAILABLE_GUIDANCE];
  }

  guidance.push(...(summary.guidance ?? []));

  if (summary.complexity === "high") {
    guidance.push(
      "High note count: this send may take longer, fail at construction, or need to be split."
    );
  } else if (summary.complexity === "medium") {
    guidance.push("Moderate fragmentation: consolidate first if this is an important or large payment.");
  }

  if (Number.isFinite(requested) && requested > 0) {
    if (requested > balance) {
      guidance.push("The requested amount is larger than the current shielded balance.");
    } else if (summary.largestNote > 0 && requested > summary.largestNote * 4) {
      const estimatedNotes = Math.ceil(requested / summary.largestNote);
      guidance.push(
        `This amount is much larger than the largest shielded note, so the node may need roughly ${estimatedNotes} or more notes to build it.`
      );
    }

    if (balance > 0 && requested > balance * 0.9) {
      guidance.push("Sending almost the full shielded balance can leave too little room for fees.");
    }
  }

  if (summary.immatureCount > 0) {
    guidance.push("Some shielded notes are still waiting for confirmations.");
  }

  return unique(guidance);
}

export function shieldedRiskLevel(amount, balance, summary) {
  const requested = Number(amount);
  if (!summary?.available) return "unknown";
  if (summary.complexity === "high") return "high";
  if (Number.isFinite(requested) && requested > 0) {
    if (requested > balance) return "high";
    if (balance > 0 && requested > balance * 0.9) return "medium";
    if (summary.largestNote > 0 && requested > summary.largestNote * 4) return "medium";
  }
  if (summary.complexity === "medium" || summary.immatureCount > 0) return "medium";
  return "low";
}

export function shieldedRiskLabel(level) {
  if (level === "high") return "High attention";
  if (level === "medium") return "Worth checking";
  if (level === "unknown") return "Limited visibility";
  return "Looks normal";
}

export function consolidationCopy(summary) {
  if (!summary?.available) {
    return "Consolidation sends BTX to a fresh shielded address in this wallet. Use it when a synced node reports many small notes.";
  }
  if (summary.complexity === "high") {
    return "Recommended before large sends. This creates a normal shielded self-send, pays a network fee, and needs confirmations.";
  }
  if (summary.complexity === "medium") {
    return "Useful before larger payments. This creates a normal shielded self-send and may pay a network fee.";
  }
  return "Optional. Your shielded note count looks manageable for normal sends.";
}

export function suggestedConsolidationAmount(balance, summary) {
  if (!summary?.available || summary.complexity === "low" || balance <= 0) return "";
  const suggested = Math.max(0, balance * MAX_SUGGESTED_CONSOLIDATION_RATIO);
  if (suggested <= 0) return "";
  return suggested.toFixed(8);
}

export function consolidationExpectations(summary) {
  const base = [
    "The wallet asks BTX core for a fresh shielded address in this wallet.",
    "BTX core builds and broadcasts a normal shielded self-send.",
    "Wait for confirmations, then refresh before retrying a large payment."
  ];
  if (summary?.complexity === "high") {
    return ["Use a smaller consolidation amount first if the wallet has many tiny notes.", ...base];
  }
  return base;
}
