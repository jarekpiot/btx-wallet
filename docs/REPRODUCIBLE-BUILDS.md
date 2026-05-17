# Reproducible Builds

Phase 0 releases are built from the official `btxchain/btx` source only. The
wallet repository provides orchestration, documentation, release signing, and
starter configuration. It does not introduce wallet-layer cryptography.

## Pinned Source

- Repository: `https://github.com/btxchain/btx.git`
- Tag: `v0.29.7`
- Commit: `2d983afab1338762b43d2614cb1104ac8a1520ec`

The build script checks that the resolved tag matches the pinned commit before
configuring CMake.

## Determinism Rules

- `SOURCE_DATE_EPOCH` is derived from the pinned BTX core commit timestamp.
- Locale and timezone are fixed to `C` and `UTC`.
- The official BTX `depends` tree is used by default.
- CPU-local flags such as `-march=native` are not used.
- CMake hardening is enabled through the upstream `ENABLE_HARDENING=ON` path.
- The upstream `check-security` target is run when the build configuration
  generates it.
- Release archives include a source manifest beside the binaries.

## Local Build

```bash
scripts/build-qt-wallet.sh
scripts/sign-release-artifacts.sh artifacts
scripts/verify-release-artifacts.sh artifacts
```

For Windows release artifacts, build with the official MinGW cross toolchain
from Linux or WSL:

```bash
BTX_TARGET_OS=windows \
BTX_TARGET_ARCH=x86_64 \
BTX_DEPENDS_HOST=x86_64-w64-mingw32 \
scripts/build-qt-wallet.sh
```

For macOS and Linux, run the same script on clean hosts for each target.

## Reproducibility Proof

The GitHub Actions release workflow performs two independent builds for each
platform target using separate source, depends, build, and staging directories.
The workflow fails unless the same-platform artifact hashes are identical.

For local verification, perform two clean builds from the same commit and
compare the artifact hashes:

```bash
sha256sum artifacts/btx-wallet-v0.1.0-qt-*.tar.gz artifacts/btx-wallet-v0.1.0-qt-*.zip
```

The release checklist must record:

- runner or machine identity
- operating system image
- compiler and CMake versions
- BTX core commit
- artifact SHA256
- signature fingerprint
- verification command output

Do not publish a release if any same-platform rebuild produces a different
artifact hash.
