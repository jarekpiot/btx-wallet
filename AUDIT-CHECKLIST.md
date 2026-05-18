# BTX Wallet - Adversarial Security & Hardening Audit (Phase 0)

Date: 2026-05-18

Version under review: `v0.1.0-qt` full-node `btx-qt`

Release URL: <https://github.com/jarekpiot/btx-wallet/releases/tag/v0.1.0-qt>

Audit rebuild workflow: <https://github.com/jarekpiot/btx-wallet/actions/runs/26017508589>

Security posture: security is the utmost priority; Phase 0 remains a packaged,
hardened build of the audited official `btxchain/btx` core with no wallet-layer
implementation code.

## Audit Finding Corrected During Review

- ✅ Least-privilege runtime default hardened after audit discovery.
  - Finding: a direct pruned regtest launch without explicit network hardening
    allowed NAT-PMP port mapping.
  - Fix: default pruned launcher and starter config now set `listen=0`,
    `natpmp=0`, and `upnp=0`.
  - Proof: hardened run log showed command-line args `listen="0"`,
    `natpmp="0"`, `upnp="0"`, pruned mode active, and no `portmap`/NAT-PMP
    add event.

## 1. Reproducible & Deterministic Builds

- ✅ Run `scripts/build-qt-wallet.sh` on clean Ubuntu, macOS, and Windows targets.
  - Proof: GitHub Actions run `26017508589` completed successfully.
- ✅ All three platform artifacts reproduce identically across two clean builds
  of the same platform.
  - Audit note: cross-platform byte-identical binaries are not a valid target;
    Linux, Windows, and macOS artifacts differ by operating-system format and
    toolchain. The enforced security target is identical SHA256 across clean
    rebuilds for each same-platform artifact.
- ✅ Deterministic build inputs enforced.
  - `BTX_CORE_REF=v0.29.7`
  - `BTX_CORE_COMMIT=2d983afab1338762b43d2614cb1104ac8a1520ec`
  - `SOURCE_DATE_EPOCH` derived from the pinned BTX core commit.
  - `ZERO_AR_DATE=1`, `TZ=UTC`, `LC_ALL=C`, `LANG=C`.
  - Deterministic archive writer fixes timestamps, ordering, UID/GID, and gzip
    mtime.
- ✅ Security compiler/linker hardening is enforced through upstream BTX
  `ENABLE_HARDENING=ON` and `REDUCE_EXPORTS=ON`.
  - Security policy documents upstream hardening coverage: FORTIFY, stack
    protection, stack-clash protection, control-flow protection, PIE, relro/now,
    separate-code, and Windows ASLR/NX where supported by the target toolchain.
  - `-march=native` is intentionally absent because CPU-local optimization breaks
    deterministic release builds.
- ✅ Pruning enabled by default for first-run release path.
  - Launcher/config proof: `-prune=4096`, `-pruneduringinit=4096`,
    `retainshieldedcommitmentindex=1`.
  - Runtime proof: local hardened regtest returned `pruned: true`,
    `automatic_pruning: true`, `prune_target_size: 576716800`.

## 2. Binary Signing & Verification

- ✅ Release assets include signed archives, per-artifact SHA256 files,
  `SHA256SUMS`, `SHA256SUMS.asc`, and detached `.asc` signatures.
- ✅ Local GPG verification passed using committed release public key.
  - Key: `BTX Wallet Release <security@btx.dev>`
  - Fingerprint: `599F9E7A4192E1BD7CEBA82ABB9A6F689BB11C30`
  - Verified:
    - `SHA256SUMS.asc` over `SHA256SUMS`
    - Linux detached signature
    - macOS detached signature
    - Windows detached signature
- ✅ Local SHA256 verification passed for downloaded release artifacts.
  - Linux: `f9f2c5583c3db7a533c399cd93bf380111a24b10219366893f10a45dfe067229`
  - macOS: `0cc7b708dc7f2e68dc75672286da9650edfce199e8ef4a9671afa85d0f201c3b`
  - Windows: `3488dc2252c4b1e06390e5a27de6fac4c616cc5484a34068f5487ef92abd5e4b`

## Fresh Reproducibility Proof

| Platform | Pass 1 SHA256 | Pass 2 SHA256 | Match | Runner |
|---|---|---|---|---|
| Linux x86_64 | `f9f2c5583c3db7a533c399cd93bf380111a24b10219366893f10a45dfe067229` | `f9f2c5583c3db7a533c399cd93bf380111a24b10219366893f10a45dfe067229` | Yes | `ubuntu-24.04` |
| macOS arm64 | `0cc7b708dc7f2e68dc75672286da9650edfce199e8ef4a9671afa85d0f201c3b` | `0cc7b708dc7f2e68dc75672286da9650edfce199e8ef4a9671afa85d0f201c3b` | Yes | `macos-14` |
| Windows x86_64 | `3488dc2252c4b1e06390e5a27de6fac4c616cc5484a34068f5487ef92abd5e4b` | `3488dc2252c4b1e06390e5a27de6fac4c616cc5484a34068f5487ef92abd5e4b` | Yes | `ubuntu-24.04` MinGW cross |

Signing proof:

```text
Generated: 2026-05-18T07:38:05Z
GPG fingerprint: 599F9E7A4192E1BD7CEBA82ABB9A6F689BB11C30
SHA256SUMS.asc: Good signature
Linux detached signature: Good signature
macOS detached signature: Good signature
Windows detached signature: Good signature
final_hardened_release_verification=PASS
```

## 3. Wallet Core Security (Functional)

- ✅ Created new encrypted descriptor wallet.
  - Wallet: `audit`
  - `descriptors: true`
  - `private_keys_enabled: true`
  - `unlocked_until: 0` after creation.
- ✅ Backup bundle created and verified.
  - `backupwallet` output: `audit-wallet-backup.dat`, 102400 bytes.
  - `restorewallet` output: restored wallet `audit_restore`.
  - `backupwalletbundle`: 10 files, `integrity_ok=True`.
  - `backupwalletbundlearchive`: SHA256
    `d9db8beceb81361ede43ab5bd89f392a367d846e3a52e85a430aa1a38ee659d4`,
    10 files, `integrity_ok=True`.
- ✅ Auto-shield/coinbase shield path tested with supported RPC.
  - `z_shieldcoinbase` produced 16 confirmed shielded notes to satisfy the
    post-fork anonymity-pool minimum.
  - Shielded wallet total after seeding: `1039.97638000 BTX`.
- ✅ Transparent transaction sent and confirmed.
  - Txid: `b83a7bc31b3eb91a2703d820548a9e9dbf86d35f17eff638881a94a3de7f8def`
  - Confirmations: `1`
- ✅ Shielded SMILE v2 transaction sent and confirmed.
  - Txid: `851f3254f9ebd00e28e2e338ad90b4ca450690cb5d913be7780604d69ece3358`
  - Confirmations: `1`
  - `z_viewtransaction include_sensitive=false`: `family=shielded_v2`,
    `value_balance_redacted=True`, `output_chunks_redacted=True`.
  - `z_viewtransaction include_sensitive=true`: `family=v2_send`,
    `value_balance=0.00320000`, `spends=1`, `outputs=2`.
- ✅ Selective disclosure behavior verified.
  - Raw `z_exportviewingkey` is disabled after block 0 by the official core.
  - Supported disclosure path is redacted/non-redacted `z_viewtransaction` plus
    structured backup/audit bundle metadata.
- ✅ Wallet auto-lock verified.
  - Probe: `walletpassphrase audit-pass 5` set `unlocked_until=1779085929`;
    after 7 seconds, `unlocked_until=0`.

## 4. Adversarial / Attack Surface Review

- ✅ No new crypto primitives added.
  - `scripts/verify-no-new-crypto.sh` passed.
  - `git ls-files` found no tracked implementation files matching C/C++/Rust/Go/
    JS/TS/Python extensions in the wallet repo.
  - All ML-DSA, SLH-DSA, SMILE v2, hashing, signing, and encryption behavior
    remains in official `btxchain/btx`.
- ✅ AppleClang compatibility patch reviewed and constrained.
  - File: `patches/btx-v0.29.7/appleclang-shielded-wallet-structured-binding.patch`
  - Change: replace one structured binding with an explicit reference alias in
    `src/wallet/shielded_wallet.cpp`.
  - Impact: build compatibility only; no primitive, consensus, serialization,
    signing, shielded proving, shielded verification, key derivation, note
    selection semantics, or wallet behavior change.
- ✅ No telemetry / phone-home / analytics code added by wallet repo.
  - Search review found no telemetry, analytics, updater phone-home, or block
    explorer integration in Phase 0 wallet packaging.
- ✅ Network access minimized for default wallet launch.
  - P2P outgoing remains available for full-node sync.
  - Inbound P2P listening and router port mapping are opt-in: `listen=0`,
    `natpmp=0`, `upnp=0`.
  - ZMQ, miniupnpc, and natpmp build integrations remain disabled in the build
    script: `WITH_ZMQ=OFF`, `WITH_MINIUPNPC=OFF`, `WITH_NATPMP=OFF`.
- ✅ Filesystem access reviewed.
  - Phase 0 uses official `btx-qt`/`btxd` datadir behavior, not a custom wallet
    sandbox.
  - Wallet files live under the official data directory wallet area; the wallet
    repo adds no code that broadens filesystem access.
- ✅ Clipboard / seed phrase handling reviewed.
  - Phase 0 adds no wallet-layer clipboard, seed phrase, key-entry, or address
    copy code.
  - Exposure remains inherited from official Qt wallet and the operating system.
- ✅ Binary tampering controls verified.
  - GPG detached signatures and `SHA256SUMS.asc` verified locally.
  - Authenticode/codesign/notarization are not present in Phase 0; GPG plus
    reproducible-build verification is the release trust path.
- ✅ Supply-chain check passed.
  - Only source repository cloned by build script is official `btxchain/btx`.
  - Pinned core tag/commit enforced before build.
  - No wallet-layer dependencies were added.

## 5. Runtime Hardening & Threat Model

- ✅ ASLR / DEP / NX / stack protections reviewed.
  - Upstream hardening is enabled with `ENABLE_HARDENING=ON`; upstream
    `check-security` target runs when generated.
  - Windows artifact is built by the official depends/MinGW path with release
    hardening and no host-local CPU flags.
- ✅ Runs as non-root / standard user.
  - Local smoke test used a standard user datadir under the workspace.
  - No admin service, installer service, or privileged background agent is added
    by this repo.
- ✅ Air-gap friendliness reviewed.
  - Artifacts can be verified offline using the committed release public key,
    `SHA256SUMS`, and detached signatures.
  - No telemetry/updater service is added.
  - Backup bundle and encrypted backup archive were created.
- ✅ Clean/offline VM posture reviewed.
  - Full three-platform clean rebuild is performed by GitHub-hosted clean
    runners.
  - Local smoke test used regtest after artifacts were downloaded; no external
    service is needed for wallet creation, backup, transparent regtest send, or
    shielded regtest send.
- ✅ Threat model reviewed.
  - Clipboard: no new wallet-layer clipboard handling.
  - Supply chain: pinned official BTX core, no added dependencies, GPG-signed
    artifacts, reproducible release workflow.
  - Node attack: official full node validation path, pruned mode, shielded
    commitment index retained, no external node default.
  - Binary tampering: local GPG and SHA verification passed; release workflow
    verifies signatures and checksums before upload.
  - Runtime exposure: inbound listener and router port mapping now disabled by
    default.
  - Backup/recovery: `backupwallet`, `restorewallet`,
    `backupwalletbundle`, and encrypted `backupwalletbundlearchive` passed.

## Sign-Off

- ✅ Fresh audit rebuild workflow `26017508589` passed.
- ✅ No unresolved local wallet smoke-test or hardening findings remain after the
  pruned-launch least-privilege fix.
- ✅ Decision: PASSED.
