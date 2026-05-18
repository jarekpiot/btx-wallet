const UNAVAILABLE_GUIDANCE =
  "Shielded note detail is unavailable. Large sends may still work, but a local synced node gives better reliability checks.";

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
    guidance.push("High note count: large shielded sends may fail or take longer to build.");
  } else if (summary.complexity === "medium") {
    guidance.push("Moderate note fragmentation: consider consolidating before a large payment.");
  }

  if (Number.isFinite(requested) && requested > 0) {
    if (requested > balance) {
      guidance.push("The requested amount is larger than the current shielded balance.");
    } else if (summary.largestNote > 0 && requested > summary.largestNote * 4) {
      guidance.push(
        "This amount is much larger than the largest shielded note, so the node may need many notes to build it."
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
