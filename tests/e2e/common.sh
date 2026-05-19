#!/usr/bin/env bash

# Shared helpers for BTX Wallet E2E smoke tests.
#
# Source this file from individual scripts. The helpers assume `btxd` or
# `btx-qt` is already running and reachable through `btx-cli`.

set -euo pipefail

BTX_CLI="${BTX_CLI:-btx-cli}"
BTX_NETWORK_ARGS="${BTX_NETWORK_ARGS:--regtest}"
WALLET_PASSPHRASE="${WALLET_PASSPHRASE:-}"
UNLOCK_SECONDS="${UNLOCK_SECONDS:-300}"
E2E_CLEANUP="${E2E_CLEANUP:-0}"
read -r -a NETWORK_ARGS <<< "$BTX_NETWORK_ARGS"

TEMP_WALLETS=()

log() {
  printf '\n== %s ==\n' "$*"
}

info() {
  printf '%s\n' "$*"
}

fail() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 1
}

require_command() {
  command -v "$1" >/dev/null 2>&1 || fail "Required command not found: $1"
}

cli() {
  "$BTX_CLI" "${NETWORK_ARGS[@]}" "$@"
}

wallet_cli() {
  local wallet="$1"
  shift
  "$BTX_CLI" "${NETWORK_ARGS[@]}" "-rpcwallet=${wallet}" "$@"
}

ensure_node() {
  require_command "$BTX_CLI"
  cli getblockchaininfo >/dev/null || fail "btx-cli cannot reach the node. Start btxd/btx-qt or adjust BTX_NETWORK_ARGS."
}

wallet_is_loaded() {
  local wallet="$1"
  cli listwallets | grep -F "\"${wallet}\"" >/dev/null
}

wallet_exists_on_disk() {
  local wallet="$1"
  cli listwalletdir | grep -F "\"${wallet}\"" >/dev/null
}

ensure_wallet() {
  local wallet="$1"
  if wallet_is_loaded "$wallet"; then
    return
  fi

  if wallet_exists_on_disk "$wallet"; then
    cli loadwallet "$wallet" >/dev/null
  else
    cli createwallet "$wallet" >/dev/null
    TEMP_WALLETS+=("$wallet")
  fi
}

ensure_existing_wallet_loaded() {
  local wallet="$1"
  if wallet_is_loaded "$wallet"; then
    return
  fi

  if wallet_exists_on_disk "$wallet"; then
    cli loadwallet "$wallet" >/dev/null
  else
    fail "Wallet '$wallet' does not exist. Create and fund it before running this test."
  fi
}

unlock_wallet_if_needed() {
  local wallet="$1"
  if [[ -n "$WALLET_PASSPHRASE" ]]; then
    wallet_cli "$wallet" walletpassphrase "$WALLET_PASSPHRASE" "$UNLOCK_SECONDS" >/dev/null || \
      fail "Unable to unlock wallet '$wallet'. Check WALLET_PASSPHRASE."
  fi
}

require_wallet_ready() {
  local wallet="$1"
  ensure_existing_wallet_loaded "$wallet"
  unlock_wallet_if_needed "$wallet"
  wallet_cli "$wallet" getwalletinfo >/dev/null || fail "Wallet '$wallet' is not ready."
}

cleanup_temp_wallets() {
  if [[ "$E2E_CLEANUP" != "1" ]]; then
    return
  fi

  for wallet in "${TEMP_WALLETS[@]}"; do
    if wallet_is_loaded "$wallet"; then
      cli unloadwallet "$wallet" >/dev/null || true
    fi
  done
}

trap cleanup_temp_wallets EXIT
