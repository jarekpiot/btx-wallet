#!/usr/bin/env bash
set -euo pipefail

# Light client RPC error-handling smoke test.
#
# This intentionally triggers safe failures that should produce actionable
# guidance in the app: invalid shielded address, insufficient funds, and locked
# wallet behavior when an encrypted wallet/passphrase is configured.
#
# Example:
#   BTX_NETWORK_ARGS="-regtest" \
#   WALLET_NAME="light-e2e-sender" \
#   tests/e2e/light-client/error-handling.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

WALLET_NAME="${WALLET_NAME:-light-e2e-sender}"
BAD_ADDRESS="${BAD_ADDRESS:-not-a-btx-address}"
TOO_LARGE_AMOUNT="${TOO_LARGE_AMOUNT:-999999999}"

expect_failure() {
  local name="$1"
  shift

  set +e
  OUTPUT="$("$@" 2>&1)"
  STATUS=$?
  set -e

  if [[ "$STATUS" -eq 0 ]]; then
    info "$OUTPUT"
    fail "Expected failure did not occur: ${name}"
  fi

  info "Expected failure observed: ${name}"
  info "$OUTPUT"
}

log "BTX Wallet Light E2E: error handling"
info "Network args: ${BTX_NETWORK_ARGS}"
info "Wallet: ${WALLET_NAME}"

ensure_node
require_wallet_ready "$WALLET_NAME"

expect_failure "invalid shielded recipient" \
  wallet_cli "$WALLET_NAME" z_sendtoaddress "$BAD_ADDRESS" "0.01"

RECEIVER_ADDRESS="$(wallet_cli "$WALLET_NAME" z_getnewaddress)"
expect_failure "shielded amount exceeds balance" \
  wallet_cli "$WALLET_NAME" z_sendtoaddress "$RECEIVER_ADDRESS" "$TOO_LARGE_AMOUNT"

if [[ -n "$WALLET_PASSPHRASE" ]]; then
  info "Testing locked wallet behavior..."
  wallet_cli "$WALLET_NAME" walletlock >/dev/null || true
  expect_failure "locked wallet shielded send" \
    wallet_cli "$WALLET_NAME" z_sendtoaddress "$RECEIVER_ADDRESS" "0.01"
else
  info "Skipping locked-wallet case because WALLET_PASSPHRASE is not set."
fi

info "PASS: expected error cases completed."
