# BTX Wallet Phase 0 Release Checklist

Release tag: `v0.1.0-qt`

Release URL: <https://github.com/jarekpiot/btx-wallet/releases/tag/v0.1.0-qt>

Successful workflow run: <https://github.com/jarekpiot/btx-wallet/actions/runs/26012356964>

BTX core source:

- Repository: `https://github.com/btxchain/btx.git`
- Tag: `v0.29.7`
- Commit: `2d983afab1338762b43d2614cb1104ac8a1520ec`
- Local compatibility patch: `patches/btx-v0.29.7/appleclang-shielded-wallet-structured-binding.patch`
  - Scope: AppleClang build compatibility only, replacing one structured binding with an explicit reference alias in `src/wallet/shielded_wallet.cpp`.
  - Crypto impact: none; no primitive, consensus, serialization, signing, or shielded algorithm change.

## Required Gates

- [x] Linux clean build completed.
- [x] Linux rebuild completed with identical artifact SHA256.
- [x] Windows clean build completed.
- [x] Windows rebuild completed with identical artifact SHA256.
- [x] macOS clean build completed.
- [x] macOS rebuild completed with identical artifact SHA256.
- [x] `SHA256SUMS` generated.
- [x] `SHA256SUMS.asc` generated with the release GPG key.
- [x] Each binary/archive has a detached `.asc` signature.
- [x] Release public key in `docs/release-signing-key.asc` matches signing key fingerprint.
- [x] `scripts/verify-release-artifacts.sh artifacts` passed in Actions.
- [x] `scripts/verify-no-new-crypto.sh` passed on all build jobs.
- [x] Test wallet created.
- [x] Test wallet encrypted and reopened.
- [x] Transparent test transaction sent.
- [x] Shielded SMILE v2 test transaction sent.
- [x] Pruned first-run launchers confirmed with `-prune=4096`.
- [x] Pruned first-run configuration confirmed with `prune=4096`.
- [x] Runtime pruned regtest node confirmed `pruned: true`.
- [x] GitHub Actions `.github/workflows/release.yml` completed successfully.
- [x] GitHub Release contains artifacts, SHA256 sums, signatures, `SIGNED-ARTIFACTS.txt`, reproducibility proofs, and this checklist.

## Build Evidence

| Platform | Clean build 1 SHA256 | Clean build 2 SHA256 | Match | Runner / OS | Notes |
|---|---|---|---|---|---|
| Linux x86_64 | `8a85d9f06f728179e1cbcce8383a054d9e458284e3a062ef3d29343649ad10d6` | `8a85d9f06f728179e1cbcce8383a054d9e458284e3a062ef3d29343649ad10d6` | Yes | Linux / `ubuntu-24.04` | `x86_64-pc-linux-gnu` |
| Windows x86_64 | `3ad5a553707d1641d242d99ce42b1356f46207fde7fb2580abf40570523c343a` | `3ad5a553707d1641d242d99ce42b1356f46207fde7fb2580abf40570523c343a` | Yes | Linux / `ubuntu-24.04` | MinGW cross toolchain `x86_64-w64-mingw32` |
| macOS arm64 | `cfde40d009660769b2b1b263c02eaf03f9ed4ec232163413bc275c6925d6cba1` | `cfde40d009660769b2b1b263c02eaf03f9ed4ec232163413bc275c6925d6cba1` | Yes | macOS / `macos-14` | `arm64-apple-darwin` |

## Signature Evidence

Release GPG fingerprint: `599F9E7A4192E1BD7CEBA82ABB9A6F689BB11C30`

Published `SHA256SUMS`:

```text
8a85d9f06f728179e1cbcce8383a054d9e458284e3a062ef3d29343649ad10d6  btx-wallet-v0.1.0-qt-linux-x86_64.tar.gz
cfde40d009660769b2b1b263c02eaf03f9ed4ec232163413bc275c6925d6cba1  btx-wallet-v0.1.0-qt-macos-arm64.tar.gz
3ad5a553707d1641d242d99ce42b1356f46207fde7fb2580abf40570523c343a  btx-wallet-v0.1.0-qt-windows-x86_64.zip
```

Actions verification proof:

```text
gpg: Good signature from "BTX Wallet Release <security@btx.dev>" [unknown]
Primary key fingerprint: 599F 9E7A 4192 E1BD 7CEB  A82A BB9A 6F68 9BB1 1C30
btx-wallet-v0.1.0-qt-linux-x86_64.tar.gz: OK
btx-wallet-v0.1.0-qt-macos-arm64.tar.gz: OK
btx-wallet-v0.1.0-qt-windows-x86_64.zip: OK
Release artifact verification passed for artifacts
```

Local Windows SHA256 proof:

```text
btx-wallet-v0.1.0-qt-windows-x86_64.zip
SHA256: 3AD5A553707D1641D242D99CE42B1356F46207FDE7FB2580ABF40570523C343A
```

Note: local GPG verification was not rerun on this Windows host because `gpg` is not installed on PATH. The release workflow imported the committed public key, verified `SHA256SUMS.asc`, and verified detached artifact signatures before upload.

## Wallet Smoke Evidence

Windows artifact used: `btx-wallet-v0.1.0-qt-windows-x86_64.zip`

```text
btxd --version: BTX daemon version v0.29.7
btx-cli --version: BTX RPC client version v0.29.7
Regtest node: pruned=true, automatic_pruning=true, prune_target_size=576716800
Wallet: smoke2
Wallet encryption: created with passphrase smoke-pass; after daemon restart, loadwallet smoke2 succeeded and walletpassphrase smoke-pass unlocked it.
Transparent txid: b046d06ad0a6c7cd88246ff7d8a8d864a4c818d2a57f85d892620bf080cad5cf
Transparent tx confirmations: 1
Shielded SMILE v2 txid: f28fb5395569492738c307f5d9320752f531c4425eac6ae298432414f67c6c9c
Shielded tx confirmations: 1
Shielded balance observed after shielded RPC activity: 54.99915000 BTX
```

## Security Review

- [x] No wallet-layer implementation code was added.
- [x] No new cryptographic primitive was added.
- [x] All cryptographic operations remain in official `btxchain/btx`.
- [x] `-DENABLE_HARDENING=ON` was used.
- [x] `-DREDUCE_EXPORTS=ON` was used.
- [x] Release hardening was used; the only pinned BTX source patch is the AppleClang build-compatibility alias described above.
- [x] `-DWERROR` policy recorded in `BTX-Wallet-Phase0/SOURCE-MANIFEST.txt`.
- [x] Upstream `check-security` target passed where generated.
- [x] No `-march=native` or machine-local optimization flag was used.
- [x] Release artifacts include `BTX-Wallet-Phase0/btx-pruned.conf`.
- [x] Release artifacts include pruned `btx-qt` launchers for Windows and Linux/macOS.

## Release Approval

Approver: automated Phase 0 release gate

Date: 2026-05-18

Decision: GO

```text
All automated build, reproducibility, no-new-crypto, signing, checksum, release upload, and Windows smoke-test gates passed.
Residual risk: macOS and Linux GUI launch was not manually opened in this environment; binaries were built, hashed, signed, and included in the release.
```
