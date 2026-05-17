# BTX Wallet Phase 0 Release Checklist

Release tag: `v0.1.0-qt`

BTX core source:

- Repository: `https://github.com/btxchain/btx.git`
- Tag: `v0.29.7`
- Commit: `2d983afab1338762b43d2614cb1104ac8a1520ec`

## Required Gates

- [ ] Linux clean build completed.
- [ ] Linux rebuild completed with identical artifact SHA256.
- [ ] Windows clean build completed.
- [ ] Windows rebuild completed with identical artifact SHA256.
- [ ] macOS clean build completed.
- [ ] macOS rebuild completed with identical artifact SHA256.
- [ ] `SHA256SUMS` generated.
- [ ] `SHA256SUMS.asc` generated with the release GPG key.
- [ ] Each binary/archive has a detached `.asc` signature.
- [ ] `scripts/verify-release-artifacts.sh artifacts` passed.
- [ ] `scripts/verify-no-new-crypto.sh` passed.
- [ ] Test wallet created.
- [ ] Test wallet encrypted and reopened.
- [ ] Transparent test transaction sent.
- [ ] Shielded SMILE v2 test transaction sent.
- [ ] Pruned first-run launchers confirmed with `-prune=4096`.
- [ ] Pruned first-run configuration confirmed with `prune=4096`.
- [ ] GitHub Actions `.github/workflows/release.yml` completed successfully.
- [ ] GitHub Release contains artifacts, SHA256 sums, signatures, `SIGNED-ARTIFACTS.txt`, reproducibility proofs, and this checklist.

## Build Evidence

| Platform | Clean build 1 SHA256 | Clean build 2 SHA256 | Match | Runner / OS | Notes |
|---|---|---|---|---|---|
| Linux x86_64 | TODO | TODO | TODO | TODO | TODO |
| Windows x86_64 | TODO | TODO | TODO | TODO | Built via MinGW cross toolchain unless otherwise noted |
| macOS arm64 | TODO | TODO | TODO | TODO | TODO |

## Signature Evidence

Release GPG fingerprint: `TODO`

```text
TODO: paste gpg --verify SHA256SUMS.asc SHA256SUMS output
TODO: paste detached artifact signature verification output
```

## Wallet Smoke Evidence

```text
TODO: paste wallet creation/encryption result
TODO: paste transparent txid
TODO: paste shielded SMILE v2 txid or operation id plus final txid
```

## Security Review

- [ ] No wallet-layer implementation code was added.
- [ ] No new cryptographic primitive was added.
- [ ] All cryptographic operations remain in official `btxchain/btx`.
- [ ] `-DENABLE_HARDENING=ON` was used.
- [ ] `-DREDUCE_EXPORTS=ON` was used.
- [ ] `-DWERROR=ON` was used.
- [ ] Upstream `check-security` target passed where generated.
- [ ] No `-march=native` or machine-local optimization flag was used.
- [ ] Release artifacts include `BTX-Wallet-Phase0/btx-pruned.conf`.
- [ ] Release artifacts include pruned `btx-qt` launchers for Windows and Linux/macOS.

## Release Approval

Approver:

Date:

Decision:

```text
TODO: record final go/no-go decision and any residual risk.
```
