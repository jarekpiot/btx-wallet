# Phase 0 Qt Usability Backport

This note documents the light, targeted usability backport applied to the
official `btx-qt` full node wallet through
`patches/btx-v0.29.7/qt-shielded-usability-backport.patch`.

## Scope

- Adds clearer shielded RPC error guidance with a short "What to try" section.
- Adds optional warnings in verbose shielded send results when a send uses
  multiple notes or approaches the live shielded note-spend limit.
- Improves discoverability of `z_mergenotes` as the supported shielded
  consolidation path for fragmented wallets.
- Adds simple empty-state labels to the Qt overview, transaction history, and
  receive-request history screens.

## Security Boundary

This backport does not add wallet-layer cryptography, note selection rules,
signing logic, proving logic, or new transaction-building paths. Shielded
transactions still use the audited BTX core wallet code. The patch only changes
user-facing guidance, RPC response text, and Qt display labels.

## Verification

On 2026-05-19:

```text
git apply --check appleclang-shielded-wallet-structured-binding.patch: passed
git apply --check qt-shielded-usability-backport.patch: passed
git diff --check: passed
scripts/verify-no-new-crypto.sh: No wallet-layer cryptographic implementation found
```

Full deterministic release builds remain covered by the Phase 0 release
workflow before publishing updated binaries.
