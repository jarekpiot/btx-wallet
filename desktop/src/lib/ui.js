export function sendBlockReason({ connected, walletReady, locked, address, amount }) {
  if (!connected) return "Connect to a node before sending.";
  if (!walletReady) return "Create, restore, or load a wallet before sending.";
  if (locked) return "Unlock the wallet temporarily before sending.";
  if (!String(address ?? "").trim()) return "Enter a recipient address.";
  if (!String(amount ?? "").trim()) return "Enter an amount.";
  return "";
}

export function sendButtonLabel({ busy, mode, blockReason }) {
  if (busy) return "Submitting...";
  if (blockReason) return "Complete send details";
  return `Send ${mode === "shielded" ? "shielded" : "transparent"}`;
}

export function receiveEmptyCopy(mode, walletReady) {
  if (!walletReady) {
    return {
      title: "No wallet loaded",
      body: "Connect to a node and create or restore a wallet before generating receive addresses."
    };
  }
  if (mode === "shielded") {
    return {
      title: "No shielded address yet",
      body: "Generate a fresh shielded address for private receives. For large transfers, receive a small test amount first."
    };
  }
  return {
    title: "No transparent address yet",
    body: "Generate a transparent address for public on-chain receives, exchange-style flows, or testing."
  };
}

export function historyEmptyCopy(walletReady) {
  if (!walletReady) {
    return {
      title: "No wallet activity yet",
      body: "Load a wallet to view transparent and shielded activity from BTX core."
    };
  }
  return {
    title: "No transactions yet",
    body: "Receive BTX or send a small test transaction, then refresh once the node reports activity."
  };
}
