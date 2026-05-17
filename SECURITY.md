# Security Policy - BTX Wallet

Security is the absolute priority for this repository.

## Reporting a Vulnerability

- Email: `security@btx.dev`
- Or open a private GitHub security advisory if repository permissions allow it.

We follow responsible disclosure with a 90-day target window unless active
exploitation requires faster coordination.

## Scope

In scope:

- release scripts and GitHub Actions workflow;
- deterministic build configuration;
- artifact hashing, signing, and verification;
- packaged first-run configuration;
- documentation that could cause unsafe wallet operation;
- accidental introduction of wallet-layer cryptography or telemetry.

Out of scope for this repo, but still critical for BTX:

- consensus code;
- SMILE v2 proving and verification internals;
- ML-DSA / SLH-DSA implementations;
- wallet key generation and signing internals.

Those operations must remain in the official audited `btxchain/btx` C++ core.
This repository must not add alternative cryptographic primitives or duplicate
wallet signing/proving logic.

## Release Guarantees

Every production release must provide:

- pinned BTX core source commit;
- public release key in `docs/release-signing-key.asc`;
- reproducible build logs for Linux, Windows, and macOS artifacts;
- SHA256 sums;
- detached GPG signatures for artifacts;
- detached GPG signature for `SHA256SUMS`;
- completed `RELEASE-CHECKLIST.md` with hashes, signatures, and smoke-test
  evidence.

## Hardening Requirements

The Phase 0 build must use upstream BTX CMake hardening:

- `-DENABLE_HARDENING=ON`
- `-DREDUCE_EXPORTS=ON`
- `-DWERROR=ON`
- no `-march=native`
- no host-specific CPU tuning in release artifacts

The official core hardening path probes and enables supported platform flags,
including FORTIFY, stack protection, stack-clash protection, control-flow
protection, PIE support, relro/now, separate-code, and Windows ASLR/NX flags
where the toolchain supports them.

## Privacy Requirements

The wallet release must remain:

- zero telemetry;
- zero analytics;
- no bundled remote logging;
- no updater phone-home;
- air-gap friendly;
- least privilege.

Network connectivity is limited to normal BTX peer discovery and user-requested
wallet/node operation from the official core.

## Required Verification

Before declaring a release complete:

1. Build each platform artifact twice on clean machines and confirm identical
   same-platform SHA256 hashes.
2. Sign artifacts and `SHA256SUMS` with the release GPG key.
3. Verify every signature and hash.
4. Create and encrypt a test wallet.
5. Send a transparent test transaction.
6. Send a shielded SMILE v2 test transaction.
7. Run `scripts/verify-no-new-crypto.sh`.
8. Attach evidence in `RELEASE-CHECKLIST.md`.

Do not publish or promote a release while any item is incomplete.
