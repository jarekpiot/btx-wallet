const DEFAULT_HINT =
  "Refresh the wallet, check the node connection, and try again. If this is a shielded send, review note health first.";

const OPERATION_COPY = {
  connect: {
    title: "Could not connect to BTX node.",
    hint: "Check that btxd is running, the RPC URL is correct, and the credentials or cookie path match your node."
  },
  shieldedSend: {
    title: "Shielded send was not submitted.",
    hint: "Check send readiness, leave room for fees, and consolidate or split the payment if the wallet has many notes."
  },
  transparentSend: {
    title: "Transparent send was not submitted.",
    hint: "Check the recipient address, available balance, and node connection."
  },
  consolidate: {
    title: "Shielded consolidation was not submitted.",
    hint: "Try a smaller consolidation amount, leave room for fees, and wait for confirmations before retrying."
  },
  receive: {
    title: "Could not create a receive address.",
    hint: "Check that a wallet is loaded and the node is reachable."
  },
  disclosure: {
    title: "Could not view the shielded transaction.",
    hint: "Check the txid, refresh the wallet, and make sure the connected node has the wallet loaded."
  },
  backup: {
    title: "Backup did not complete.",
    hint: "Check the destination path, passphrases, and node wallet access."
  },
  wallet: {
    title: "Wallet operation did not complete.",
    hint: "Check the wallet name, passphrase, and node connection."
  }
};

function cleanMessage(raw) {
  const text = String(raw ?? "").trim();
  if (!text || text === "[object Object]") return "The app did not receive a readable error from the node.";
  return text.replace(/^Error:\s*/i, "").trim();
}

export function friendlyError(raw, operation = "default") {
  const message = cleanMessage(raw);
  const copy = OPERATION_COPY[operation];
  const title = copy?.title ?? "Operation failed.";
  const hint = copy?.hint ?? DEFAULT_HINT;

  if (/what to try:/i.test(message)) {
    return `${title}\n\n${message}`;
  }

  return `${title}\n\n${message}\n\nWhat to try: ${hint}`;
}

export function successMessage(operation, txid) {
  if (operation === "shieldedSend") {
    return `Shielded transaction submitted.\n\nTxid: ${txid}\n\nNext: wait for confirmations, then refresh the wallet.`;
  }
  if (operation === "consolidate") {
    return `Shielded consolidation submitted.\n\nTxid: ${txid}\n\nNext: wait for confirmations, refresh note health, then retry the larger send.`;
  }
  if (operation === "transparentSend") {
    return `Transparent transaction submitted.\n\nTxid: ${txid}`;
  }
  return txid ? `Operation submitted.\n\nTxid: ${txid}` : "Operation completed.";
}
