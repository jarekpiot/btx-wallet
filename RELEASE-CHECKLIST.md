# BTX Wallet Phase 0 Release Checklist

Release tag: `v0.1.0-qt`

Release URL: <https://github.com/jarekpiot/btx-wallet/releases/tag/v0.1.0-qt>

Successful workflow run: <https://github.com/jarekpiot/btx-wallet/actions/runs/26017508589>

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
| Linux x86_64 | `f9f2c5583c3db7a533c399cd93bf380111a24b10219366893f10a45dfe067229` | `f9f2c5583c3db7a533c399cd93bf380111a24b10219366893f10a45dfe067229` | Yes | Linux / `ubuntu-24.04` | `x86_64-pc-linux-gnu` |
| Windows x86_64 | `3488dc2252c4b1e06390e5a27de6fac4c616cc5484a34068f5487ef92abd5e4b` | `3488dc2252c4b1e06390e5a27de6fac4c616cc5484a34068f5487ef92abd5e4b` | Yes | Linux / `ubuntu-24.04` | MinGW cross toolchain `x86_64-w64-mingw32` |
| macOS arm64 | `0cc7b708dc7f2e68dc75672286da9650edfce199e8ef4a9671afa85d0f201c3b` | `0cc7b708dc7f2e68dc75672286da9650edfce199e8ef4a9671afa85d0f201c3b` | Yes | macOS / `macos-14` | `arm64-apple-darwin` |

## Signature Evidence

Release GPG fingerprint: `599F9E7A4192E1BD7CEBA82ABB9A6F689BB11C30`

Published `SHA256SUMS`:

```text
f9f2c5583c3db7a533c399cd93bf380111a24b10219366893f10a45dfe067229  btx-wallet-v0.1.0-qt-linux-x86_64.tar.gz
0cc7b708dc7f2e68dc75672286da9650edfce199e8ef4a9671afa85d0f201c3b  btx-wallet-v0.1.0-qt-macos-arm64.tar.gz
3488dc2252c4b1e06390e5a27de6fac4c616cc5484a34068f5487ef92abd5e4b  btx-wallet-v0.1.0-qt-windows-x86_64.zip
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
SHA256: 3488DC2252C4B1E06390E5A27DE6FAC4C616CC5484A34068F5487EF92ABD5E4B
```

Local GPG verification was rerun on this Windows host with Git for Windows `gpg.exe` and a temporary audit keyring. `SHA256SUMS.asc` and all three detached artifact signatures verified successfully against fingerprint `599F9E7A4192E1BD7CEBA82ABB9A6F689BB11C30`.

## Wallet Smoke Evidence

Windows artifact used: `btx-wallet-v0.1.0-qt-windows-x86_64.zip`

```text
btxd --version: BTX daemon version v0.29.7
btx-cli --version: BTX RPC client version v0.29.7
Regtest node: pruned=true, automatic_pruning=true, prune_target_size=576716800
Wallet: audit
Wallet encryption: created with passphrase audit-pass; descriptor wallet opened, unlocked, backed up, restored, and auto-lock verified.
Transparent txid: b83a7bc31b3eb91a2703d820548a9e9dbf86d35f17eff638881a94a3de7f8def
Transparent tx confirmations: 1
Shielded SMILE v2 txid: 851f3254f9ebd00e28e2e338ad90b4ca450690cb5d913be7780604d69ece3358
Shielded tx confirmations: 1
Shielded balance observed after shielded RPC activity: 1039.97638000 BTX
Audit wallet bundle: backupwallet, restorewallet, backupwalletbundle, and backupwalletbundlearchive passed with integrity_ok=true.
Runtime hardening: hardened pruned launch used listen=0, natpmp=0, upnp=0 and log review found no portmap/NAT-PMP add event.
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
- [x] Pruned launchers and starter config disable inbound listening and router port mapping by default with `listen=0`, `natpmp=0`, and `upnp=0`.

## Release Approval

Approver: automated Phase 0 release gate

Date: 2026-05-18

Decision: GO

```text
All automated build, reproducibility, no-new-crypto, signing, checksum, release upload, hardened runtime, and Windows smoke-test gates passed.
Residual risk: macOS and Linux GUI launch was not manually opened in this environment; binaries were built, hashed, signed, and included in the release.
```
