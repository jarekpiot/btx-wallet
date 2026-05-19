# BTX Wallet

Official desktop wallets for **BTX**: a post-quantum blockchain with native
SMILE v2 shielded transactions.

This repository contains two desktop wallet options:

- **BTX Wallet Light** - Phase 1, recommended for most users.
- **BTX Wallet (Full Node)** - Phase 0, for users who want to run and validate
  their own full node.

Security remains the default: no telemetry, no phone-home services, signed
releases, verifiable SHA256 sums, and no wallet-layer cryptographic code in this
repository.

## Downloads

| Wallet | Release | Best For |
|---|---|---|
| **BTX Wallet Light** | [v0.2.0-light](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.2.0-light) | Most users who want a fast, simple desktop wallet |
| **BTX Wallet (Full Node)** | [v0.1.0-qt](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.1.0-qt) | Users who want maximum trustlessness and local validation |

Download the wallet for your operating system from the release page, then verify
the artifact before opening it. See [Verify Downloads](#verify-downloads).

## Which Wallet Should I Use?

Use **BTX Wallet Light** if you want the easiest desktop experience. It launches
quickly, does not require a full chain sync before opening, and connects to a
local or trusted remote `btxd` node through JSON-RPC. This is the recommended
choice for most BTC, NEAR, and ZEC users who want a simple native wallet with
transparent and SMILE v2 shielded transactions.

Use **BTX Wallet (Full Node)** if you want to run and validate the chain
yourself. It uses the official `btx-qt` GUI from the audited `btxchain/btx`
core, includes pruning defaults for normal gaming PCs, and provides the most
trustless setup. It is heavier than the light client because it runs a full node
and wallet together.

| Wallet | Type | GUI | Runs Full Node | Notes |
|---|---|---|---|---|
| **BTX Wallet Light** | Light client | Tauri | No | Fast startup, simple UX, connects to local or trusted remote `btxd` |
| **BTX Wallet (Full Node)** | Full node client | Qt | Yes | Maximum local validation, pruned defaults, direct `btx-qt` experience |

## Getting Started: Light Client

Recommended setup: run your own local `btxd` node and connect the light client
to it. This keeps the wallet experience simple while reducing trust in public
RPC infrastructure.

1. Download **BTX Wallet Light** from the
   [v0.2.0-light release](https://github.com/jarekpiot/btx-wallet/releases/tag/v0.2.0-light).
2. Verify the download using `SHA256SUMS` and GPG signatures.
3. Install and launch the app.
4. Open **Settings**.
5. Connect to your node. For a local node, use `http://127.0.0.1:18443`.
6. Create or restore an encrypted descriptor wallet.
7. Start with a small receive and send test.
8. Use shielded mode for private SMILE v2 transactions.

BTX Wallet Light includes shielded note-health warnings, send-readiness
guidance, clearer error messages, and basic consolidation support for larger or
more complex shielded sends.

## Full Node Wallet

BTX Wallet (Full Node) is the Phase 0 desktop wallet path. It packages the
official `btx-qt` GUI from `btxchain/btx` with reproducible build metadata,
SHA256 sums, GPG signatures, and pruning defaults.

Use the full node wallet when you want to:

- validate the BTX chain locally;
- avoid relying on a remote node for wallet state;
- run `btx-qt` and wallet functionality together;
- use the hardened Phase 0 release path.

For first-run instructions, see [docs/FIRST-RUN.md](docs/FIRST-RUN.md).

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

On macOS, use `shasum -a 256 -c SHA256SUMS` if `sha256sum` is unavailable. On
Windows, use Git Bash, WSL, or PowerShell with GPG installed.

## Security

Both wallets have gone through security reviews:

- **Phase 0 (`btx-qt`)**: full adversarial security and hardening audit
  completed.
- **Phase 1 (Light Client)**: focused desktop security review and hardening pass
  completed, including Phase 1.5 and Phase 2 usability/reliability
  improvements.

Key points:

- No telemetry.
- No phone-home services.
- No new cryptographic code in the desktop apps.
- No wallet-layer signing, proving, encryption, or note-handling code.
- Wallets use encrypted descriptor wallets with post-quantum signatures in the
  audited BTX core.
- Sensitive operations are delegated to official `btxchain/btx`.

See [SECURITY.md](SECURITY.md), [AUDIT-CHECKLIST.md](AUDIT-CHECKLIST.md),
[RELEASE-CHECKLIST.md](RELEASE-CHECKLIST.md), and
[DESKTOP-RELEASE-CHECKLIST.md](DESKTOP-RELEASE-CHECKLIST.md) for details.

## Known Limitations

- Large or complex shielded sends can still be affected by note limits and
  fragmentation, which is common in shielded systems. BTX Wallet Light includes
  warnings, readiness guidance, clearer errors, and basic consolidation support.
  The full node wallet includes lighter shielded RPC guidance and consolidation
  suggestions.
- Public RPC infrastructure for BTX is still limited. Running your own local
  `btxd` node is currently the most reliable setup.
- Windows Authenticode signing and Apple notarization are not yet implemented.
  Release artifacts are GPG-signed and SHA256-verified.

## Development

- Phase 0 build scripts and workflows are in the repository root.
- Phase 1 Tauri light client lives in the [desktop](desktop) folder.
- Security checklists and verification scripts are included in the repository.

### Build the Light Client Locally

```bash
cd desktop
npm install
npm run tauri:build
```

### Build the Full Node Wallet Locally

```bash
scripts/build-qt-wallet.sh
scripts/sign-release-artifacts.sh artifacts
scripts/verify-release-artifacts.sh artifacts
```

Additional docs:

- [docs/FIRST-RUN.md](docs/FIRST-RUN.md)
- [docs/PHASE1-LIGHT-CLIENT.md](docs/PHASE1-LIGHT-CLIENT.md)
- [docs/PHASE0-QT-USABILITY-BACKPORT.md](docs/PHASE0-QT-USABILITY-BACKPORT.md)
- [docs/REPRODUCIBLE-BUILDS.md](docs/REPRODUCIBLE-BUILDS.md)
