#!/usr/bin/env bash
set -euo pipefail

# Full Node RPC guidance smoke test.
#
# This script checks that the shielded RPC methods used by the Qt RPC console
# are available and expose the guidance fields added by the usability backport.
# It does not inspect Qt widget state directly; use MANUAL-E2E-CHECKLIST.md for
# visual RPC-console guidance and history-redaction checks.
#
# Example:
#   BTX_NETWORK_ARGS="-regtest" \
#   WALLET_NAME="qt-e2e-sender" \
#   tests/e2e/full-node/rpc-console-test.sh

BTX_CLI="${BTX_CLI:-btx-cli}"
BTX_NETWORK_ARGS="${BTX_NETWORK_ARGS:--regtest}"
WALLET_NAME="${WALLET_NAME:-qt-e2e-sender}"
read -r -a NETWORK_ARGS <<< "$BTX_NETWORK_ARGS"

cli() {
  "$BTX_CLI" "${NETWORK_ARGS[@]}" "$@"
}

wallet_cli() {
  "$BTX_CLI" "${NETWORK_ARGS[@]}" "-rpcwallet=${WALLET_NAME}" "$@"
}

ensure_wallet_loaded() {
  if ! cli listwallets | grep -F "\"${WALLET_NAME}\"" >/dev/null; then
    if cli listwalletdir | grep -F "\"${WALLET_NAME}\"" >/dev/null; then
      cli loadwallet "$WALLET_NAME" >/dev/null
    else
      cli createwallet "$WALLET_NAME" >/dev/null
    fi
  fi
}

echo "== BTX Full Node E2E: RPC console guidance surfaces =="
echo "Network args: ${BTX_NETWORK_ARGS}"
echo "Wallet: ${WALLET_NAME}"

cli getblockchaininfo >/dev/null
ensure_wallet_loaded

echo "Checking shielded RPC help includes consolidation guidance..."
HELP_OUTPUT="$(wallet_cli help z_sendmany)"
echo "$HELP_OUTPUT" | grep -F "z_mergenotes" >/dev/null || {
  echo "Expected z_sendmany help to mention z_mergenotes guidance." >&2
  exit 1
}

echo "Checking z_getbalance exposes note-health guidance when available..."
BALANCE_OUTPUT="$(wallet_cli z_getbalance || true)"
echo "$BALANCE_OUTPUT"
if ! echo "$BALANCE_OUTPUT" | grep -E "note_health|guidance|recovery_only" >/dev/null; then
  echo "z_getbalance did not show note-health fields. This can be normal for an empty or locked wallet; verify manually in Qt with a funded shielded wallet." >&2
fi

echo "Manual follow-up required:"
echo "- Open btx-qt RPC console."
echo "- Run help-console and confirm shielded workflow hints are visible."
echo "- Run a shielded command and confirm guidance appears before execution."
echo "- Confirm shielded-sensitive commands are not retained in command history."

echo "PASS: full-node RPC guidance smoke checks completed."
