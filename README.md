# BTX Wallet

Official desktop wallets for **BTX** (`btx.dev`) - a post-quantum blockchain
with native shielded transactions (SMILE v2).

This repository contains two desktop wallet options:

- **BTX Wallet (Full Node)** - Phase 0
- **BTX Wallet Light** - Phase 1, recommended for most users

Security remains the default: no telemetry, signed releases, verifiable
SHA256 sums, and no wallet-layer cryptography in this repo.

## Two Desktop Wallets

| Wallet | Type | GUI | Runs Full Node | Best For | Download |
|---|---|---|---|---|---|
| **BTX Wallet** | Full Node Client | Qt | Yes | Users who want to validate the chain themselves | [v0.1.0-qt](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.1.0-qt) |
| **BTX Wallet Light** | Light Client | Tauri | No | Most users who want a fast native wallet | [v0.2.0-light](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.2.0-light) |

## BTX Wallet Light - Recommended

BTX Wallet Light is a modern Tauri desktop app for Windows, macOS, and Linux.
It connects to a local or trusted remote `btxd` node over JSON-RPC, so users can
install and open the wallet without syncing a full node first.

- Fast startup and simple native UI.
- Clear shielded and transparent wallet modes.
- Encrypted descriptor wallet creation and restore through official BTX RPC.
- SMILE v2 shielded send, receive, and selective disclosure through official
  BTX core.
- Shielded note health guidance for larger or fragmented sends.
- Basic shielded note consolidation by sending to a fresh shielded address in
  the same wallet.
- No telemetry, no updater phone-home, and no persisted RPC password or wallet
  passphrase.

### Quick Start

1. Download **BTX Wallet Light** from the
   [v0.2.0-light release](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.2.0-light).
2. Install and open the app.
3. Go to **Settings**.
4. Enter your node RPC URL, for example `http://127.0.0.1:18443`.
5. Create or restore an encrypted descriptor wallet.
6. Start with a small test receive and send.
7. Use shielded mode for private SMILE v2 transactions.

For the best trust model, run your own `btxd` node locally and connect the light
client to it. Trusted remote nodes are supported, but they can see RPC metadata
and can misreport wallet state.

## BTX Wallet Full Node - Phase 0

The Phase 0 wallet ships the audited `btx-qt` GUI from the official
[`btxchain/btx`](https://github.com/btxchain/btx) core, pinned to `v0.29.7`
commit `2d983afab1338762b43d2614cb1104ac8a1520ec`.

- Runs a full BTX node and wallet.
- Maximum trustlessness.
- Includes pruning defaults for normal gaming PCs.
- Best for users who want to validate the chain themselves.

Download the full-node wallet from the
[v0.1.0-qt release](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.1.0-qt).

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

Both wallets have undergone security reviews:

- **Phase 0 (`btx-qt`)**: adversarial security and hardening audit completed.
- **Phase 1 (Light Client)**: focused desktop security review and hardening pass
  completed.

Key notes:

- No telemetry.
- No phone-home services.
- No new cryptographic primitives.
- No wallet-layer signing, proving, encryption, or note-handling code.
- All cryptographic operations remain inside official `btxchain/btx`.
- Release artifacts are built, hashed, signed, and verified.

See [SECURITY.md](SECURITY.md), [RELEASE-CHECKLIST.md](RELEASE-CHECKLIST.md),
and [DESKTOP-RELEASE-CHECKLIST.md](DESKTOP-RELEASE-CHECKLIST.md).

## Current Limitations

- Large shielded sends can be more complex when a wallet has many small notes.
  BTX Wallet Light now warns about this and offers basic note consolidation.
- Public BTX RPC infrastructure is still limited. Running your own node is
  recommended for the best experience.
- Windows Authenticode signing and Apple notarization are not yet implemented;
  release artifacts are GPG-signed and SHA256-verified.

## Development

The repository contains:

- Phase 0 full-node build scripts and release workflow.
- Phase 1 Tauri light client in [desktop](desktop).
- Security checklists and verification scripts.

Build Phase 1 locally:

```bash
cd desktop
npm install
npm run tauri:build
```

Build Phase 0 locally:

```bash
scripts/build-qt-wallet.sh
scripts/sign-release-artifacts.sh artifacts
scripts/verify-release-artifacts.sh artifacts
```

See [docs/FIRST-RUN.md](docs/FIRST-RUN.md),
[docs/PHASE1-LIGHT-CLIENT.md](docs/PHASE1-LIGHT-CLIENT.md), and
[docs/REPRODUCIBLE-BUILDS.md](docs/REPRODUCIBLE-BUILDS.md) for more detail.
