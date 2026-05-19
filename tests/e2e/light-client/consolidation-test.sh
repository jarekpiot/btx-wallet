#!/usr/bin/env bash
set -euo pipefail

# Basic shielded note consolidation smoke test for a local BTX node.
#
# Prerequisites:
# - btxd is running and reachable by btx-cli.
# - The wallet exists, is unlocked if encrypted, and has multiple shielded notes.
# - This script calls z_mergenotes only; it does not create fragmentation.
#
# Example:
#   BTX_NETWORK_ARGS="-regtest" \
#   WALLET_NAME="light-e2e-sender" \
#   tests/e2e/consolidation-test.sh

BTX_CLI="${BTX_CLI:-btx-cli}"
BTX_NETWORK_ARGS="${BTX_NETWORK_ARGS:--regtest}"
WALLET_NAME="${WALLET_NAME:-light-e2e-sender}"
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
      echo "Wallet '${WALLET_NAME}' does not exist. Create and fund it before running consolidation." >&2
      exit 1
    fi
  fi
}

echo "== BTX Wallet Light E2E: shielded note consolidation =="
echo "Network args: ${BTX_NETWORK_ARGS}"
echo "Wallet: ${WALLET_NAME}"

cli getblockchaininfo >/dev/null
ensure_wallet_loaded

echo "Before consolidation: shielded balance and note health"
wallet_cli z_getbalance || {
  echo "z_getbalance failed. Confirm the wallet is unlocked, synced, and has shielded funds." >&2
  exit 1
}

echo "Running z_mergenotes..."
MERGE_RESULT="$(wallet_cli z_mergenotes)"
echo "$MERGE_RESULT"

echo "After consolidation command: refresh balance and guidance"
wallet_cli z_getbalance || true

echo "If running regtest, mine a block and run z_getbalance again to confirm note health improves."
echo "PASS: consolidation command completed."
