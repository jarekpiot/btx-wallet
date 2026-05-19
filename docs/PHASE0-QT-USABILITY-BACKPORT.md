# Phase 0 Qt Usability Backport

This note documents the light, targeted usability backport applied to the
official `btx-qt` full node wallet through
`patches/btx-v0.29.7/qt-shielded-usability-backport.patch`.

## Scope

- Adds clearer shielded RPC error guidance with a short "What to try" section.
- Expands shielded failure guidance for locked wallets, invalid destinations,
  fee issues, amount/fee mistakes, sync/anchor issues, and fragmented notes.
- Adds shielded note-health guidance to `z_getbalance` so users can see whether
  note count looks normal, moderately fragmented, highly fragmented, or limited
  by scan/locked-state visibility.
- Adds optional warnings in verbose shielded send results when a send uses
  multiple notes or approaches the live shielded note-spend limit.
- Adds `next_steps` to complex verbose shielded send results so users know to
  wait for confirmation, refresh note health, consolidate, or split a payment.
- Adds `next_steps` to `z_mergenotes` results so consolidation feels less
  mysterious and users understand whether another merge may be useful.
- Improves discoverability of `z_mergenotes` as the supported shielded
  consolidation path for fragmented wallets.
- Adds simple empty-state labels to the Qt overview, transaction history, and
  receive-request history screens.
- Adds clearer send-confirmation guidance that reminds users transactions are
  irreversible after broadcast and recommends a small test amount for important
  payments.
- Adds temporary "Preparing..." and "Broadcasting..." send-button feedback with
  a wait cursor during synchronous send preparation and broadcast handoff.
- Expands standard Qt send errors with concise "What to try" guidance for
  invalid addresses, invalid amounts, insufficient balance, fee issues,
  duplicate recipients, and transaction creation failures.
- Clarifies wallet unlock copy so users understand the wallet should relock
  automatically after the requested operation when possible.
- Cleans up BTX-specific navigation, address-book, receive, and send labels.

## Security Boundary

This backport does not add wallet-layer cryptography, note selection rules,
signing logic, proving logic, or new transaction-building paths. Shielded
transactions still use the audited BTX core wallet code. The patch only changes
user-facing guidance, RPC response text, and Qt display labels.

The deeper Phase 0 backport intentionally remains advisory. It does not block
or bypass a send, change note selection, alter fees, change nullifier
reservation behavior, or add a new shielded transaction path.

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

Additional deeper-backport verification on 2026-05-19:

```text
git apply --check qt-shielded-usability-backport.patch in fresh local BTX worktree: passed
git apply qt-shielded-usability-backport.patch + git diff --check: passed
manual source review: changes are limited to Qt empty-state display logic and shielded RPC guidance/result fields
repo diff --check: passed
```

Additional Qt polish-backport verification on 2026-05-19:

```text
fresh local BTX worktree: git apply --check qt-shielded-usability-backport.patch passed
fresh local BTX worktree: git apply qt-shielded-usability-backport.patch + git diff --check passed
fresh local BTX worktree: git apply appleclang patch + qt usability patch + git diff --check passed
repository diff --check: passed
targeted no-new-crypto source scan: only user-facing encrypted-wallet guidance and existing SMILE note-limit constants matched
manual source review: new Qt changes are limited to static labels, status tips, wait-cursor/button feedback, and guidance strings
```

Note: the repository `scripts/verify-no-new-crypto.sh` helper is Bash-based.
In this Windows review environment, `bash` resolves to WSL and no Linux
distribution is installed. Equivalent source/pattern review was performed
against the patch and applied BTX worktree.

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
- [x] Deeper note-health guidance is based on aggregate spendable note count
  already returned by `z_getbalance`; it does not expose nullifiers,
  commitments, addresses, note values, or tree positions.
- [x] `z_mergenotes` next-step text is derived from merge counts and aggregate
  remaining spendable note count only. It does not change merge note selection,
  fee calculation, signing, or broadcast behavior.
- [x] New `warnings`, `guidance`, and `next_steps` fields are static/advisory
  strings built from existing core state. They are not parsed as commands and
  do not execute user input.
- [x] Send-dialog progress feedback is RAII-scoped UI state only. It does not
  change transaction preparation, fee selection, signing, broadcast behavior, or
  error control flow.
- [x] Expanded Qt send errors and confirmation text are advisory only and reuse
  existing Qt translation/message paths.
- [x] Unlock-dialog wording is static guidance only. It does not change
  passphrase handling, wallet encryption, or relock behavior.

Review proof:

```text
git apply --check appleclang-shielded-wallet-structured-binding.patch: passed
git apply --check qt-shielded-usability-backport.patch: passed
git apply both patches in a clean worktree + git diff --check: passed
scripts/verify-no-new-crypto.sh: No wallet-layer cryptographic implementation found
fresh local worktree apply check for deeper patch: passed
fresh local worktree apply check for polish patch: passed
fresh local worktree git diff --check after polish patch: passed
fresh local worktree git diff --check after AppleClang + Qt usability patches: passed
targeted no-new-crypto source scan: no new cryptographic implementation or transaction-building path found
```

Residual recommendations:

- Run the full Phase 0 deterministic build workflow before publishing updated
  binaries.
- Add a small upstream wallet/RPC test for `warnings` behavior if this patch is
  later moved from repository patch form into the core tree.
- Add RPC tests for `z_getbalance.note_health`, `z_getbalance.guidance`, and
  `z_mergenotes.next_steps` when this patch is promoted from repository patch
  form into the core tree.
- Keep future Qt full-node backports similarly limited to display text,
  documentation, and existing RPC/core behavior unless a separate full security
  review is planned.
