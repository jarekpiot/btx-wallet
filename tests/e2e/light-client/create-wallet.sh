#!/usr/bin/env bash
set -euo pipefail

# Light client wallet create/restore-adjacent smoke test.
#
# BTX Wallet Light delegates wallet creation to the connected node. This script
# verifies the basic create/load/encrypt-ready RPC path that the app depends on.
#
# Example:
#   BTX_NETWORK_ARGS="-regtest" \
#   WALLET_NAME="light-e2e-create" \
#   tests/e2e/light-client/create-wallet.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

WALLET_NAME="${WALLET_NAME:-light-e2e-create}"

log "BTX Wallet Light E2E: create wallet"
info "Network args: ${BTX_NETWORK_ARGS}"
info "Wallet: ${WALLET_NAME}"

ensure_node

if wallet_is_loaded "$WALLET_NAME"; then
  info "Wallet is already loaded: ${WALLET_NAME}"
elif wallet_exists_on_disk "$WALLET_NAME"; then
  info "Loading existing wallet: ${WALLET_NAME}"
  cli loadwallet "$WALLET_NAME" >/dev/null
else
  info "Creating wallet: ${WALLET_NAME}"
  cli createwallet "$WALLET_NAME" >/dev/null
  TEMP_WALLETS+=("$WALLET_NAME")
fi

info "Wallet info:"
wallet_cli "$WALLET_NAME" getwalletinfo

info "Creating a shielded receive address to verify shielded wallet RPC readiness..."
wallet_cli "$WALLET_NAME" z_getnewaddress

info "PASS: create/load wallet flow completed."
