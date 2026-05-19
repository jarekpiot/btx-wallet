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

## Lightweight Security Review

Review date: 2026-05-19

Decision: safe to include in the next Phase 0 build after normal CI/release
build verification.

Findings:

- [x] No new cryptographic code, key handling, note selection rules, proving,
  signing, encryption, or transaction construction paths were added.
- [x] Shielded sends, consolidation, fee calculation, signing, and broadcast
  remain delegated to the existing BTX core wallet implementation.
- [x] "What to try" guidance is appended to backend/RPC errors as plain text.
  It does not parse user input as markup, execute commands, or alter error
  control flow.
- [x] Qt empty states use static translated labels only. They do not render
  wallet data, addresses, labels, transaction IDs, or other user-provided text.
- [x] New Qt model-signal connections only toggle static empty-state label
  visibility based on row counts.
- [x] Complex-send warnings are advisory only and do not block, bypass, or
  alter transaction building.
- [x] Privacy finding fixed: complex-send warnings are now suppressed when the
  existing shielded RPC privacy path redacts input/output counts. This avoids
  leaking a coarse note-count or change-output hint through warning text.

Review proof:

```text
git apply --check appleclang-shielded-wallet-structured-binding.patch: passed
git apply --check qt-shielded-usability-backport.patch: passed
git apply both patches in a clean worktree + git diff --check: passed
scripts/verify-no-new-crypto.sh: No wallet-layer cryptographic implementation found
```

Residual recommendations:

- Run the full Phase 0 deterministic build workflow before publishing updated
  binaries.
- Add a small upstream wallet/RPC test for `warnings` behavior if this patch is
  later moved from repository patch form into the core tree.
- Keep future Qt full-node backports similarly limited to display text,
  documentation, and existing RPC/core behavior unless a separate full security
  review is planned.
