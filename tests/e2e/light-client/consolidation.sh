#!/usr/bin/env bash

# Light client shielded note consolidation smoke test.
#
# This validates the z_mergenotes RPC path used by BTX Wallet Light's
# consolidation flow. It does not create fragmentation; use a funded wallet with
# multiple shielded notes.
#
# Example:
#   BTX_NETWORK_ARGS="-regtest" \
#   WALLET_NAME="light-e2e-sender" \
#   tests/e2e/light-client/consolidation.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

WALLET_NAME="${WALLET_NAME:-light-e2e-sender}"

log "BTX Wallet Light E2E: shielded note consolidation"
info "Network args: ${BTX_NETWORK_ARGS}"
info "Wallet: ${WALLET_NAME}"

ensure_node
require_wallet_ready "$WALLET_NAME"

info "Before consolidation: shielded balance and note health"
wallet_cli "$WALLET_NAME" z_getbalance || {
  fail "z_getbalance failed. Confirm the wallet is unlocked, synced, and has shielded funds."
}

info "Running z_mergenotes..."
MERGE_RESULT="$(wallet_cli "$WALLET_NAME" z_mergenotes)"
info "$MERGE_RESULT"

info "After consolidation command: refresh balance and guidance"
wallet_cli "$WALLET_NAME" z_getbalance || true

info "If running regtest, mine a block and run z_getbalance again to confirm note health improves."
info "PASS: consolidation command completed."
