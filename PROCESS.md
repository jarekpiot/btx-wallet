# BTX Wallet Process

This document explains how development, reviews, and releases are handled in
the BTX Wallet repository. It is intentionally lightweight: the goal is to make
the work repeatable without slowing down focused security and usability
improvements.

## Development Approach

BTX Wallet is developed iteratively, with Codex-assisted implementation,
manual review, and targeted verification after each meaningful change.

Preferred changes are:

- focused and easy to review;
- testable with the existing scripts and checks;
- aligned with the current wallet architecture;
- documented when they affect users, security, releases, or operations.

The repository contains two wallet tracks:

- **BTX Wallet Light** in `desktop/`, built with Tauri, Rust, and Svelte.
- **BTX Wallet (Full Node)** through the Phase 0 `btx-qt` release path and
  targeted patches under `patches/`.

The desktop app must remain a control surface over official BTX core. It must
not add wallet-layer signing, proving, key generation, note selection,
encryption, or new cryptographic primitives.

## Security Review Process

Security and user fund safety take priority over convenience, feature speed, or
visual polish.

A focused security review is required after changes involving:

- wallet creation, restore, unlock, backup, or passphrase handling;
- RPC connection handling, credentials, cookie paths, or saved nodes;
- transparent or shielded send flows;
- shielded note health, consolidation, or send-readiness logic;
- amount, address, wallet name, path, txid, or user-input validation;
- Tauri permissions, IPC commands, CSP, local storage, or filesystem access;
- release signing, hashing, installers, or artifact verification.

Stronger adversarial review is required before public releases, after broad
changes to sensitive flows, or whenever a change could plausibly cause loss of
funds, privacy loss, or unsafe recovery behavior.

Reviews must be documented in the relevant checklist:

- `AUDIT-CHECKLIST.md` for Phase 0 full-node wallet audits.
- `RELEASE-CHECKLIST.md` for Phase 0 release proof.
- `DESKTOP-RELEASE-CHECKLIST.md` for Phase 1 light-client reviews and release
  proof.
- specific docs under `docs/` when reviewing a targeted patch or backport.

Findings should be severity-rated when the review is security-sensitive:
Critical, High, Medium, or Low. Critical and High issues must be fixed before a
release is considered safe.

## Release Process

Releases must be reproducible where practical, signed, and documented.

For every public release:

- run the relevant build and test checks;
- produce SHA256 sums for release artifacts;
- GPG-sign artifacts and `SHA256SUMS`;
- verify signatures and hashes from a clean download location when possible;
- update the relevant release checklist with proof;
- keep release notes clear about limitations and residual risks.

### Light Client Releases

BTX Wallet Light releases are built from `desktop/`.

Expected checks include:

- frontend tests with `npm test`;
- production frontend build with `npm run build`;
- Rust formatting and tests with `cargo fmt --check` and `cargo test`;
- Tauri production bundle with `npm run tauri:build`;
- dependency review such as `npm audit --audit-level=moderate`;
- review of Tauri permissions and app security settings;
- confirmation that wallet-sensitive operations still delegate to BTX core.

### Full Node Releases

BTX Wallet full-node releases follow the Phase 0 `btx-qt` path.

Expected checks include:

- reproducible build scripts;
- security hardening flags;
- pruning defaults;
- artifact hashing and GPG signing;
- release checklist proof;
- wallet smoke tests through official BTX core where available.

## Backport Process

Backports from the light client to the full-node Qt wallet should be light and
targeted. Prefer practical improvements such as clearer errors, better
shielded-send guidance, and improved empty states.

Avoid broad UI rewrites or attempts at feature parity. The full-node client
should remain close to the official `btx-qt` experience, with small usability
patches that are easy to audit and maintain.

Backports should be documented under `docs/` and, when delivered as patch
files, kept under `patches/`.

## Documentation Updates

Update documentation when changes affect:

- installation, first-run, or wallet setup;
- release verification;
- security posture or residual risks;
- RPC configuration or node connection behavior;
- shielded send behavior, consolidation, limitations, or recovery guidance;
- contributor workflows or build commands.

User-facing changes should usually update `README.md` or a focused document in
`docs/`. Security-relevant changes should update the relevant checklist.

## General Guidelines

- Keep commits clean and scoped.
- Prefer small, reviewable changes over large mixed refactors.
- Preserve the inclusive BTC, NEAR, and ZEC-friendly tone in public docs.
- Do not add dependencies unless they are clearly justified and reviewed.
- Do not add telemetry, analytics, updater phone-home, or unnecessary
  permissions.
- Keep sensitive data out of local storage and logs.
- Validate user and RPC inputs at the backend boundary.
- Treat frontend checks as UX guidance, not as the security boundary.
- Run the existing verification scripts before release-sensitive commits.
- Document residual risks plainly instead of hiding them.

