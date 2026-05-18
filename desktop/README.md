# BTX Wallet Light

BTX Wallet Light is the Phase 1 desktop wallet for `v0.2.0-light`.

It is a Tauri v2 application with a Svelte frontend and Rust backend. The app
does not implement cryptography. Wallet creation, descriptor handling,
post-quantum key material, transparent signing, SMILE v2 shielded sends,
selective disclosure, and backup operations are delegated to the official
audited BTX core through JSON-RPC.

## Security Model

- No wallet-layer cryptography.
- No telemetry, analytics, updater phone-home, or bundled remote logging.
- RPC credentials and wallet passphrases are held in memory only.
- Saved node profiles store only node URL, wallet name, display label, and
  remote-approval preference. They do not store RPC usernames, passwords, or
  cookie contents.
- Remote RPC endpoints are blocked unless the user explicitly allows them.
- Remote RPC over plain HTTP is blocked; use a local node or an HTTPS tunnel.
- Tauri permissions are minimal: no shell, filesystem, dialog, updater, or HTTP
  plugin permissions are granted to the webview.

For `v0.2.0-light`, the app is a lightweight client over trusted BTX JSON-RPC.
Normal users can install and open the app without syncing a full chain. They can
connect to a local node later, or to a trusted remote `btxd` endpoint.

## Shielded Send Reliability

The app reads shielded note metadata from `z_listunspent` when the connected
node exposes it. That metadata is used only for user guidance: note-count
warnings, large-send hints, and a simple consolidation action that sends BTX
back to a fresh shielded address in the same wallet through official BTX RPC.
The desktop layer still does not build, sign, decrypt, or verify shielded
transactions itself.

## Development

```bash
cd desktop
npm install
npm run dev
npm run tauri:dev
```

Production build:

```bash
cd desktop
npm ci
npm run build
npm run tauri:build
```

## RPC Methods Used

- `getblockchaininfo`
- `getwalletinfo`
- `getbalances`
- `z_gettotalbalance`
- `z_listunspent`
- `listtransactions`
- `createwallet`
- `restorewallet`
- `walletpassphrase`
- `walletlock`
- `getnewaddress`
- `z_getnewaddress`
- `sendtoaddress`
- `z_sendtoaddress`
- `z_viewtransaction`
- `backupwalletbundle`
- `backupwalletbundlearchive`

The app intentionally does not expose a raw RPC console in the first release.
