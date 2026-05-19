# BTX Wallet E2E Tests

This folder contains lightweight Bash end-to-end test assets for the two BTX
desktop wallets:

- `light-client/` covers flows used by BTX Wallet Light.
- `full-node/` covers flows used by `btx-qt` / the full-node wallet.

The scripts are intentionally small `btx-cli` smoke tests designed to run in
WSL or any normal Bash environment. They assume a trusted local `btxd` or
`btx-qt` node is already running and reachable by `btx-cli`.

## Manual Checklist

Run the manual release checklist first:

```bash
tests/MANUAL-E2E-CHECKLIST.md
```

Record the OS, app version, node version, network, command output, txids, and
any screenshots in the relevant release checklist.

## Scripted Smoke Tests

Light client RPC-path checks:

```bash
BTX_NETWORK_ARGS="-regtest" tests/e2e/light-client/create-wallet.sh
BTX_NETWORK_ARGS="-regtest" tests/e2e/light-client/shielded-send.sh
BTX_NETWORK_ARGS="-regtest" tests/e2e/light-client/consolidation.sh
BTX_NETWORK_ARGS="-regtest" tests/e2e/light-client/error-handling.sh
```

Full-node RPC-path checks:

```bash
BTX_NETWORK_ARGS="-regtest" tests/e2e/full-node/shielded-send.sh
BTX_NETWORK_ARGS="-regtest" tests/e2e/full-node/rpc-console.sh
```

Useful environment variables:

- `BTX_CLI`: path to `btx-cli` if it is not on `PATH`.
- `BTX_NETWORK_ARGS`: network/RPC flags, such as `-regtest`.
- `SENDER_WALLET`: sender wallet name for shielded send tests.
- `RECEIVER_WALLET`: receiver wallet name for shielded send tests.
- `WALLET_NAME`: wallet name for consolidation/RPC-console tests.
- `AMOUNT`: shielded send amount.
- `WALLET_PASSPHRASE`: optional passphrase used to unlock encrypted test wallets.
- `E2E_CLEANUP=1`: unload wallets created by the scripts on exit.

These scripts do not replace manual UI testing. Use them to confirm the critical
local RPC paths before doing release sign-off in the desktop apps.
