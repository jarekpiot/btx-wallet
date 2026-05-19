#!/usr/bin/env bash

# Light client shielded send smoke test.
#
# This validates the critical RPC path used by BTX Wallet Light for SMILE v2
# shielded sends. It assumes the sender wallet already has shielded funds.
#
# Example:
#   BTX_NETWORK_ARGS="-regtest" \
#   SENDER_WALLET="light-e2e-sender" \
#   RECEIVER_WALLET="light-e2e-receiver" \
#   AMOUNT="0.01" \
#   tests/e2e/light-client/shielded-send.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

SENDER_WALLET="${SENDER_WALLET:-light-e2e-sender}"
RECEIVER_WALLET="${RECEIVER_WALLET:-light-e2e-receiver}"
AMOUNT="${AMOUNT:-0.01}"
CONF_TARGET="${CONF_TARGET:-6}"
ESTIMATE_MODE="${ESTIMATE_MODE:-economical}"

log "BTX Wallet Light E2E: shielded send"
info "Network args: ${BTX_NETWORK_ARGS}"
info "Sender wallet: ${SENDER_WALLET}"
info "Receiver wallet: ${RECEIVER_WALLET}"
info "Amount: ${AMOUNT} BTX"

ensure_node
require_wallet_ready "$SENDER_WALLET"
ensure_wallet "$RECEIVER_WALLET"

info "Checking sender shielded balance and note guidance..."
wallet_cli "$SENDER_WALLET" z_getbalance || {
  fail "z_getbalance failed. Confirm the sender wallet is unlocked, synced, and has shielded funds."
}

info "Creating receiver shielded address..."
RECEIVER_ADDRESS="$(wallet_cli "$RECEIVER_WALLET" z_getnewaddress)"
info "Receiver address: ${RECEIVER_ADDRESS}"

info "Sending shielded transaction..."
TXID="$(wallet_cli "$SENDER_WALLET" z_sendtoaddress "$RECEIVER_ADDRESS" "$AMOUNT" "" "" true 0 true "$CONF_TARGET" "$ESTIMATE_MODE")"
info "Shielded txid: ${TXID}"

info "If running regtest, mine a block externally or with your local miner, then refresh balances."
info "Receiver shielded balance:"
wallet_cli "$RECEIVER_WALLET" z_getbalance || true

info "PASS: shielded send command completed."
