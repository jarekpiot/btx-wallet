# BTX Wallet - Simple, Secure, Post-Quantum

Official lightweight desktop wallet release repo for **BTX** (`btx.dev` /
`btxprice.com`).

Phase 0 ships the audited `btx-qt` wallet from the official
[`btxchain/btx`](https://github.com/btxchain/btx) core, pinned to `v0.29.7`
commit `2d983afab1338762b43d2614cb1104ac8a1520ec`.

- Full SMILE v2 shielded transactions: sender, receiver, and amount hidden,
  with selective disclosure support in the core.
- Native ML-DSA-44 plus SLH-DSA-128s post-quantum signatures from genesis.
- No new wallet-layer cryptography in this repo.
- Zero telemetry, reproducible builds, GPG-signed releases, air-gap friendly.
- Pruned first-run config included for normal gaming PCs.

Security is non-negotiable. Every production release must include signed
artifacts, verifiable SHA256 sums, and a completed release checklist.

## Download

Open the GitHub Release for `v0.1.0-qt` and download the archive for your
operating system:

- Windows: `btx-wallet-v0.1.0-qt-windows-x86_64.zip`
- macOS: `btx-wallet-v0.1.0-qt-macos-arm64.tar.gz`
- Linux: `btx-wallet-v0.1.0-qt-linux-x86_64.tar.gz`

Also download:

- `SHA256SUMS`
- `SHA256SUMS.asc`
- the matching artifact `.asc` signature

Import the Phase 0 release public key first:

```bash
gpg --import docs/release-signing-key.asc
```

Verify before opening the wallet:

```bash
gpg --verify SHA256SUMS.asc SHA256SUMS
sha256sum -c SHA256SUMS
gpg --verify btx-wallet-v0.1.0-qt-linux-x86_64.tar.gz.asc btx-wallet-v0.1.0-qt-linux-x86_64.tar.gz
```

On macOS, use `shasum -a 256 -c SHA256SUMS` if `sha256sum` is unavailable.
On Windows, use Git Bash, WSL, or PowerShell with GPG installed.

## First Run

Phase 0 is built for the BTC, NEAR, and ZEC crowd: a familiar desktop wallet,
private sends, post-quantum keys, and no telemetry.

1. Extract the release archive.
2. Start with `BTX-Wallet-Phase0/launch-btx-qt-pruned.sh` on Linux/macOS or
   `BTX-Wallet-Phase0\launch-btx-qt-pruned.cmd` on Windows.
3. For a permanent config, copy `BTX-Wallet-Phase0/btx-pruned.conf` to your BTX data directory as
   `btx.conf`.
4. Create a new wallet.
5. Encrypt the wallet immediately.
6. Back up the wallet using the official wallet backup flow.
7. Send a small transparent test transaction first.
8. Send a small shielded SMILE v2 test transaction to a `btxs1...` address.

BTX data directories:

| OS | Config path |
|---|---|
| Linux | `~/.btx/btx.conf` |
| macOS | `~/Library/Application Support/BTX/btx.conf` |
| Windows | `%APPDATA%\BTX\btx.conf` |

The included launchers and starter config use `prune=4096`,
`pruneduringinit=4096`, and `retainshieldedcommitmentindex=1` so first sync is
practical while shielded wallet restarts remain fast. They also set
`listen=0`, `natpmp=0`, and `upnp=0` so inbound P2P and router port mapping
are opt-in, not default.

See [docs/FIRST-RUN.md](docs/FIRST-RUN.md) for the full beginner flow.

## Reproducible Builds

Build locally:

```bash
scripts/build-qt-wallet.sh
scripts/sign-release-artifacts.sh artifacts
scripts/verify-release-artifacts.sh artifacts
```

The build script:

- clones the official `btxchain/btx` core only;
- verifies the pinned `v0.29.7` commit;
- derives `SOURCE_DATE_EPOCH` from the core commit;
- uses the official BTX `depends` tree by default;
- enables upstream hardening with `ENABLE_HARDENING=ON`;
- runs the upstream `check-security` target when the core generates it;
- avoids `-march=native` and other host-specific optimization flags;
- packages a source manifest with every artifact.

See [docs/REPRODUCIBLE-BUILDS.md](docs/REPRODUCIBLE-BUILDS.md).

## Release Process

Production releases are built by
[.github/workflows/release.yml](.github/workflows/release.yml).

Required GitHub secrets:

- `BTX_RELEASE_GPG_PRIVATE_KEY`
- `BTX_RELEASE_GPG_KEY_ID`
- `BTX_RELEASE_GPG_PASSPHRASE` if the key is passphrase-protected

The Phase 0 public release key is committed at
[docs/release-signing-key.asc](docs/release-signing-key.asc).

The release is not complete until [RELEASE-CHECKLIST.md](RELEASE-CHECKLIST.md)
contains clean build hashes, GPG verification output, and wallet smoke-test
evidence.

## Security

See [SECURITY.md](SECURITY.md).

The short version:

- no telemetry;
- no phone-home services;
- no new cryptographic primitives;
- no wallet-layer signing or proving code;
- all cryptographic operations remain inside official `btxchain/btx`;
- release artifacts must be reproducible, hashed, signed, and verified.
