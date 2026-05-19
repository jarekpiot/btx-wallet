#!/usr/bin/env bash

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
#   tests/e2e/full-node/rpc-console.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

WALLET_NAME="${WALLET_NAME:-qt-e2e-sender}"

log "BTX Full Node E2E: RPC console guidance surfaces"
info "Network args: ${BTX_NETWORK_ARGS}"
info "Wallet: ${WALLET_NAME}"

ensure_node
ensure_wallet "$WALLET_NAME"
unlock_wallet_if_needed "$WALLET_NAME"

info "Checking shielded RPC help includes consolidation guidance..."
HELP_OUTPUT="$(wallet_cli "$WALLET_NAME" help z_sendmany)"
echo "$HELP_OUTPUT" | grep -F "z_mergenotes" >/dev/null || {
  fail "Expected z_sendmany help to mention z_mergenotes guidance."
}

info "Checking z_getbalance exposes note-health guidance when available..."
BALANCE_OUTPUT="$(wallet_cli "$WALLET_NAME" z_getbalance || true)"
info "$BALANCE_OUTPUT"
if ! echo "$BALANCE_OUTPUT" | grep -E "note_health|guidance|recovery_only" >/dev/null; then
  info "z_getbalance did not show note-health fields. This can be normal for an empty wallet; verify manually in Qt with a funded shielded wallet."
fi

info "Manual follow-up required:"
info "- Open btx-qt RPC console."
info "- Run help-console and confirm shielded workflow hints are visible."
info "- Run a shielded command and confirm guidance appears before execution."
info "- Confirm shielded-sensitive commands are not retained in command history."

info "PASS: full-node RPC guidance smoke checks completed."
