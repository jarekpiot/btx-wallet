# Phase 1 Light Client Plan

Release target: `v0.2.0-light`

Phase 1 adds a modern Tauri desktop application in `desktop/`. It keeps the
inclusive BTC, NEAR, and ZEC-friendly tone from Phase 0 while making the wallet
easier for normal users to install and use.

## Product Scope

The first light client is intentionally small:

- connect to local or trusted remote `btxd`;
- create and restore encrypted descriptor wallets;
- show transparent and shielded balances clearly;
- send transparent BTX;
- send SMILE v2 shielded BTX;
- generate transparent and shielded receive addresses;
- view shielded transactions with optional sensitive local details;
- create official BTX wallet backup bundles and encrypted backup archives;
- lock and temporarily unlock the wallet.

The app launches without starting or syncing a full node. Users who already run
`btxd` locally can connect to `http://127.0.0.1:18443`. Users who want a light
setup can connect to a trusted HTTPS RPC endpoint.

## Security Boundaries

The Tauri app is a control surface, not a cryptographic implementation.

- ML-DSA, SLH-DSA, SMILE v2 proving, note handling, descriptor derivation,
  signing, encryption, and backup internals remain in official `btxchain/btx`.
- RPC credentials and wallet passphrases are never persisted by the app.
- Remote plain HTTP RPC is rejected.
- The Tauri webview has minimal permissions and no shell/filesystem/updater
  plugins.
- No telemetry, analytics, crash upload, or phone-home behavior is added.

## Release Requirements

Before publishing `v0.2.0-light`:

1. Build desktop bundles on Windows, macOS, and Linux.
2. Run frontend tests and Tauri/Rust checks.
3. Verify local and remote-node connection flows.
4. Create an encrypted descriptor wallet through RPC.
5. Restore an official BTX wallet backup.
6. Send transparent and shielded test transactions on regtest or testnet.
7. Verify redacted and sensitive `z_viewtransaction` flows.
8. Sign all artifacts and `SHA256SUMS` with the release GPG key.
9. Attach `DESKTOP-RELEASE-CHECKLIST.md` to the release.
