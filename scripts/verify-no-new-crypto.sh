#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

tracked_code="$(
  git -C "${repo_root}" ls-files \
    '*.c' '*.cc' '*.cpp' '*.cxx' '*.h' '*.hpp' '*.hh' \
    '*.rs' '*.go' '*.js' '*.ts' '*.jsx' '*.tsx' '*.py' 2>/dev/null \
    | grep -v '^desktop/' || true
)"

if [ -n "${tracked_code}" ]; then
  printf 'error: wallet repo contains non-desktop implementation code outside the official btx core:\n' >&2
  printf '%s\n' "${tracked_code}" >&2
  exit 1
fi

desktop_code="$(
  git -C "${repo_root}" ls-files 'desktop/**/*' 2>/dev/null | grep -E '\.(rs|js|ts|svelte)$' || true
)"

if [ -n "${desktop_code}" ] && (
  cd "${repo_root}"
  grep -InE 'ML-DSA|SLH-DSA|SMILE|cipher|signature|signing key|private key|random|rng|encrypt|decrypt|hash' ${desktop_code}
) >/tmp/btx-wallet-desktop-crypto-refs 2>/dev/null; then
  printf 'Desktop source references security-sensitive terms for review:\n'
  cat /tmp/btx-wallet-desktop-crypto-refs
fi

if grep -RInE 'ML-DSA|SLH-DSA|SMILE|crypto|cipher|hash|signature|random|rng|encrypt|decrypt' \
  -- README.md SECURITY.md RELEASE-CHECKLIST.md DESKTOP-RELEASE-CHECKLIST.md docs scripts .github desktop/README.md >/tmp/btx-wallet-crypto-refs 2>/dev/null; then
  printf 'Documentation/script references to crypto were found for review:\n'
  cat /tmp/btx-wallet-crypto-refs
fi

printf 'No wallet-layer cryptographic implementation found. Crypto remains delegated to official btxchain/btx.\n'
