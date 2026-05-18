# BTX Wallet

Official desktop wallets for **BTX** (the post-quantum blockchain with native
shielded transactions).

This repository contains two desktop wallet options:

- **BTX Wallet (Full Node)** - Phase 0
- **BTX Wallet Light** - Phase 1, recommended for most users

Security remains the default: no telemetry, signed releases, verifiable
SHA256 sums, and no wallet-layer cryptographic code in this repository.

## Two Desktop Wallets

| Wallet | Type | GUI | Runs Full Node | Recommended For | Release |
|---|---|---|---|---|---|
| **BTX Wallet** | Full Node Client | Qt | Yes | Users who want to run and validate their own node | [v0.1.0-qt](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.1.0-qt) |
| **BTX Wallet Light** | Light Client | Tauri | No | Most users who want a fast, simple desktop wallet | [v0.2.0-light](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.2.0-light) |

### BTX Wallet (Full Node) - Phase 0

- Official `btx-qt` GUI from the audited `btxchain/btx` core.
- Runs a full BTX node and wallet together.
- Maximum trustlessness.
- Includes pruning defaults for normal gaming PCs.
- Best suited for users who want to validate the full chain themselves.

### BTX Wallet Light - Phase 1 (Recommended)

- Modern Tauri desktop application.
- Lightweight and fast to start.
- Connects to a local or trusted remote `btxd` node.
- Designed to be simple, secure, and reliable for everyday use.
- Supports transparent and native **SMILE v2** shielded transactions.
- Includes shielded note health warnings, send-readiness guidance, and basic
  consolidation support for larger or more complex shielded sends.

## Quick Start (Light Client)

1. Download **BTX Wallet Light** from the
   [v0.2.0-light release](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.2.0-light).
2. Install and launch the app.
3. Go to **Settings**.
4. Connect to your node. Recommended: run your own local `btxd`.
5. Create or restore an encrypted descriptor wallet.
6. Start with a small receive and send test.
7. Use shielded mode for private SMILE v2 transactions.

Tip: for the best experience, run your own `btxd` node locally and connect the
light client to `http://127.0.0.1:18443`.

## Verify Downloads

For every release, download the wallet artifact plus:

- `SHA256SUMS`
- `SHA256SUMS.asc`
- the matching artifact `.asc` signature

Import the release public key:

```bash
gpg --import docs/release-signing-key.asc
```

Verify before opening the wallet:

```bash
gpg --verify SHA256SUMS.asc SHA256SUMS
sha256sum -c SHA256SUMS
gpg --verify <artifact>.asc <artifact>
```

On macOS, use `shasum -a 256 -c SHA256SUMS` if `sha256sum` is unavailable.
On Windows, use Git Bash, WSL, or PowerShell with GPG installed.

## Security

Both wallets have gone through security reviews:

- **Phase 0 (`btx-qt`)**: full adversarial security and hardening audit
  completed.
- **Phase 1 (Light Client)**: focused desktop security review and hardening pass
  completed, including Phase 1.5 usability and reliability improvements.

Key points:

- No telemetry.
- No phone-home services.
- No new cryptographic code in the desktop apps.
- No wallet-layer signing, proving, encryption, or note-handling code.
- Wallets use encrypted descriptor wallets with post-quantum signatures in the
  audited BTX core.
- Sensitive operations are delegated to official `btxchain/btx`.

See [AUDIT-CHECKLIST.md](AUDIT-CHECKLIST.md),
[RELEASE-CHECKLIST.md](RELEASE-CHECKLIST.md),
[DESKTOP-RELEASE-CHECKLIST.md](DESKTOP-RELEASE-CHECKLIST.md), and
[SECURITY.md](SECURITY.md) for details.

## Current Limitations

- Large or complex shielded sends can still be affected by note limits, which
  is common in shielded systems. BTX Wallet Light includes warnings, readiness
  guidance, and basic consolidation support, with further improvements planned.
- Public RPC infrastructure for BTX is still limited. Running your own node is
  currently the most reliable option.
- Windows Authenticode signing and Apple notarization are not yet implemented;
  release artifacts are GPG-signed and SHA256-verified.

## Development

- Phase 0 build scripts and workflows are in the repository root.
- Phase 1 Tauri light client lives in the [desktop](desktop) folder.
- Security checklists and verification scripts are included in the repository.

### Building the Light Client Locally

```bash
cd desktop
npm install
npm run tauri:build
```

### Building the Full Node Wallet Locally

```bash
scripts/build-qt-wallet.sh
scripts/sign-release-artifacts.sh artifacts
scripts/verify-release-artifacts.sh artifacts
```

See [docs/FIRST-RUN.md](docs/FIRST-RUN.md),
[docs/PHASE1-LIGHT-CLIENT.md](docs/PHASE1-LIGHT-CLIENT.md), and
[docs/REPRODUCIBLE-BUILDS.md](docs/REPRODUCIBLE-BUILDS.md) for more detail.
