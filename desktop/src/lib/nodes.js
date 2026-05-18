export const LOCAL_NODE_PROFILE = {
  id: "local-default",
  label: "Local BTX node",
  url: "http://127.0.0.1:18443",
  wallet: "main",
  allowRemote: false
};

const STORAGE_KEY = "btx-wallet-light.nodes.v1";
const MAX_PROFILES = 8;
const MAX_LABEL = 48;
const MAX_URL = 512;
const MAX_WALLET = 64;

function trimBounded(value, max) {
  return String(value ?? "").trim().slice(0, max);
}

export function sanitizeNodeProfile(input) {
  const label = trimBounded(input?.label, MAX_LABEL) || "BTX node";
  const url = trimBounded(input?.url, MAX_URL);
  const wallet = trimBounded(input?.wallet, MAX_WALLET) || "main";
  const allowRemote = Boolean(input?.allowRemote);

  if (!url) return undefined;

  return {
    id: trimBounded(input?.id, 80) || `${Date.now()}`,
    label,
    url,
    wallet,
    allowRemote
  };
}

export function loadSavedNodes(storage = globalThis.localStorage) {
  try {
    const raw = storage?.getItem(STORAGE_KEY);
    const parsed = raw ? JSON.parse(raw) : [];
    if (!Array.isArray(parsed)) return [];
    return parsed.map(sanitizeNodeProfile).filter(Boolean).slice(0, MAX_PROFILES);
  } catch {
    return [];
  }
}

export function saveNodeProfile(profile, existing = [], storage = globalThis.localStorage) {
  const sanitized = sanitizeNodeProfile(profile);
  if (!sanitized) return existing;

  const profiles = [
    sanitized,
    ...existing.filter((item) => item.id !== sanitized.id && item.url !== sanitized.url)
  ].slice(0, MAX_PROFILES);

  storage?.setItem(STORAGE_KEY, JSON.stringify(profiles));
  return profiles;
}

export function deleteNodeProfile(id, existing = [], storage = globalThis.localStorage) {
  const profiles = existing.filter((item) => item.id !== id);
  storage?.setItem(STORAGE_KEY, JSON.stringify(profiles));
  return profiles;
}

export function isLocalNodeUrl(url) {
  try {
    const parsed = new URL(url);
    return ["127.0.0.1", "localhost", "::1"].includes(parsed.hostname);
  } catch {
    return false;
  }
}

export function connectionModeLabel(url, allowRemote = false) {
  if (isLocalNodeUrl(url)) return "Local node";
  return allowRemote ? "Trusted remote node" : "Remote node blocked";
}

export function syncStatusLabel(node) {
  if (!node) return "Not connected";
  const blocks = Number(node.blocks ?? 0);
  const headers = Number(node.headers ?? 0);
  if (node.initialblockdownload) return "Initial sync";
  if (headers > blocks) return `Syncing ${blocks} / ${headers}`;
  return `Synced at ${blocks}`;
}
