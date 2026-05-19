#!/usr/bin/env bash
set -euo pipefail

# Basic shielded send smoke test for a local BTX node.
#
# Prerequisites:
# - btxd is running and reachable by btx-cli.
# - The sender wallet exists, is unlocked if encrypted, and has shielded funds.
# - The receiver wallet exists or can be created by this script.
#
# Example:
#   BTX_NETWORK_ARGS="-regtest" \
#   SENDER_WALLET="light-e2e-sender" \
#   RECEIVER_WALLET="light-e2e-receiver" \
#   AMOUNT="0.01" \
#   tests/e2e/shielded-send-test.sh

BTX_CLI="${BTX_CLI:-btx-cli}"
BTX_NETWORK_ARGS="${BTX_NETWORK_ARGS:--regtest}"
SENDER_WALLET="${SENDER_WALLET:-light-e2e-sender}"
RECEIVER_WALLET="${RECEIVER_WALLET:-light-e2e-receiver}"
AMOUNT="${AMOUNT:-0.01}"
CONF_TARGET="${CONF_TARGET:-6}"
ESTIMATE_MODE="${ESTIMATE_MODE:-economical}"
read -r -a NETWORK_ARGS <<< "$BTX_NETWORK_ARGS"

cli() {
  "$BTX_CLI" "${NETWORK_ARGS[@]}" "$@"
}

wallet_cli() {
  local wallet="$1"
  shift
  "$BTX_CLI" "${NETWORK_ARGS[@]}" "-rpcwallet=${wallet}" "$@"
}

ensure_wallet() {
  local wallet="$1"
  if ! cli listwallets | grep -F "\"${wallet}\"" >/dev/null; then
    if cli listwalletdir | grep -F "\"${wallet}\"" >/dev/null; then
      cli loadwallet "$wallet" >/dev/null
    else
      cli createwallet "$wallet" >/dev/null
    fi
  fi
}

echo "== BTX Wallet Light E2E: shielded send =="
echo "Network args: ${BTX_NETWORK_ARGS}"
echo "Sender wallet: ${SENDER_WALLET}"
echo "Receiver wallet: ${RECEIVER_WALLET}"
echo "Amount: ${AMOUNT} BTX"

cli getblockchaininfo >/dev/null
ensure_wallet "$SENDER_WALLET"
ensure_wallet "$RECEIVER_WALLET"

echo "Checking sender shielded balance and note guidance..."
wallet_cli "$SENDER_WALLET" z_getbalance || {
  echo "z_getbalance failed. Confirm the sender wallet is unlocked, synced, and has shielded funds." >&2
  exit 1
}

echo "Creating receiver shielded address..."
RECEIVER_ADDRESS="$(wallet_cli "$RECEIVER_WALLET" z_getnewaddress)"
echo "Receiver address: ${RECEIVER_ADDRESS}"

echo "Sending shielded transaction..."
TXID="$(wallet_cli "$SENDER_WALLET" z_sendtoaddress "$RECEIVER_ADDRESS" "$AMOUNT" "" "" true 0 true "$CONF_TARGET" "$ESTIMATE_MODE")"
echo "Shielded txid: ${TXID}"

echo "If running regtest, mine a block externally or with your local miner, then refresh balances."
echo "Receiver shielded balance:"
wallet_cli "$RECEIVER_WALLET" z_getbalance || true

echo "PASS: shielded send command completed."
