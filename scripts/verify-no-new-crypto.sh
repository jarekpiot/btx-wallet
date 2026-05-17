#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

tracked_code="$(
  git -C "${repo_root}" ls-files \
    '*.c' '*.cc' '*.cpp' '*.cxx' '*.h' '*.hpp' '*.hh' \
    '*.rs' '*.go' '*.js' '*.ts' '*.jsx' '*.tsx' '*.py' 2>/dev/null || true
)"

if [ -n "${tracked_code}" ]; then
  printf 'error: wallet repo contains implementation code outside the official btx core:\n' >&2
  printf '%s\n' "${tracked_code}" >&2
  exit 1
fi

if grep -RInE 'ML-DSA|SLH-DSA|SMILE|crypto|cipher|hash|signature|random|rng|encrypt|decrypt' \
  -- README.md SECURITY.md RELEASE-CHECKLIST.md docs scripts .github >/tmp/btx-wallet-crypto-refs 2>/dev/null; then
  printf 'Documentation/script references to crypto were found for review:\n'
  cat /tmp/btx-wallet-crypto-refs
fi

printf 'No wallet-layer implementation code found. Crypto remains delegated to official btxchain/btx.\n'
