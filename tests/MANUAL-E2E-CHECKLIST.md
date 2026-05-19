# BTX Wallet Manual E2E Testing Checklist

This checklist covers critical end-to-end flows for both BTX desktop wallets:

- **BTX Wallet Light** (Tauri)
- **BTX Wallet Full Node** (`btx-qt`)

Use a trusted local `btxd` node when possible. Record the app version, node
version, network, OS, test date, logs, screenshots, and any failed commands in
the relevant release checklist.

## Test Setup

- [ ] Install the release candidate on a clean Windows, macOS, or Linux machine.
- [ ] Start a trusted local `btxd` node.
- [ ] Confirm `btx-cli getblockchaininfo` works against the same node.
- [ ] Confirm the wallet app launches without a blank window or startup error.
- [ ] Confirm no telemetry, analytics, or unexpected network requests are made.
- [ ] Confirm test wallets are funded with transparent and shielded test funds.

## Light Client (Tauri)

### High Priority

| # | Test Case | Expected Result | Pass? | Notes |
|---|-----------|-----------------|-------|-------|
| 1 | Create new encrypted wallet | Wallet created successfully; passphrase is required and sensitive fields clear after use | | |
| 2 | Restore wallet from backup | Wallet restores successfully and expected balance/history is visible | | |
| 3 | Connect to local node | Settings shows connected status and wallet data refreshes | | |
| 4 | Send transparent transaction | Transaction succeeds, txid is shown, and balance/history update after confirmation | | |
| 5 | Send normal shielded transaction | SMILE v2 send succeeds; warnings are shown if applicable | | |
| 6 | Send large/complex shielded transaction | Warning and guidance appear before or during the send flow | | |
| 7 | Perform note consolidation | Consolidation completes with clear guidance and confirmation expectations | | |
| 8 | Check shielded note health/readiness | Spendable notes, fragmentation/readiness, and suggestions are visible and accurate | | |
| 9 | Test error cases: bad address, insufficient funds, locked wallet | Clear errors plus actionable "What to try" guidance | | |
| 10 | Final confirmation before broadcast | User must confirm before send; irreversible-send language is clear | | |

### Medium Priority

| # | Test Case | Expected Result | Pass? | Notes |
|---|-----------|-----------------|-------|-------|
| 11 | View shielded transaction history | History is clear, readable, and does not overexpose private note details | | |
| 12 | Backup and restore flow | Backup bundle can be created and restored end-to-end | | |
| 13 | Test long operations such as scans or refreshes | Progress feedback is clear and the UI remains understandable | | |
| 14 | Node disconnection handling | App shows graceful connection errors and recovers after reconnect | | |

## Full Node Client (Qt / btx-qt)

### High Priority

| # | Test Case | Expected Result | Pass? | Notes |
|---|-----------|-----------------|-------|-------|
| 1 | Create or restore wallet via Qt | Wallet opens correctly and encryption/unlock flow is clear | | |
| 2 | Send shielded transaction via supported RPC flow | Send works with guidance/warnings and no new wallet-layer crypto | | |
| 3 | Send large/complex shielded transaction | RPC result or console guidance shows warnings and suggested next steps | | |
| 4 | Use RPC console with shielded commands | Console shows helpful guidance; sensitive shielded commands are not retained in history | | |
| 5 | Check empty states: Overview, History, Receive | Empty states are clear and helpful | | |
| 6 | Test error messages for shielded sends | Errors include clear, actionable guidance | | |
| 7 | Perform operations while wallet locked | Proper unlock or locked-wallet messages are shown | | |

### Medium Priority

| # | Test Case | Expected Result | Pass? | Notes |
|---|-----------|-----------------|-------|-------|
| 8 | Progress dialogs during scans/imports | Labels explain what is happening and whether cancellation may require another scan | | |
| 9 | Address book and receive flow | Receive requests and address book guidance are clear and smooth | | |
| 10 | Navigation and status indicators | Overview, Send, Receive, Transactions, sync, and status tips are clear and consistent | | |

## Scripted E2E Smoke Tests

- [ ] Review `tests/e2e/light-client/shielded-send-test.sh` configuration.
- [ ] Run `tests/e2e/light-client/shielded-send-test.sh` against a local test node.
- [ ] Review `tests/e2e/light-client/consolidation-test.sh` configuration.
- [ ] Run `tests/e2e/light-client/consolidation-test.sh` against a wallet with multiple shielded notes.
- [ ] Review `tests/e2e/full-node/shielded-send-test.sh` configuration.
- [ ] Run `tests/e2e/full-node/shielded-send-test.sh` against a local full-node wallet.
- [ ] Review `tests/e2e/full-node/rpc-console-test.sh` configuration.
- [ ] Run `tests/e2e/full-node/rpc-console-test.sh` to verify shielded RPC guidance surfaces are available.
- [ ] Attach command output, txids, and any failures to the release checklist.

## Release Sign-Off

- [ ] Manual E2E passed for BTX Wallet Light on at least one clean machine.
- [ ] Manual E2E passed for `btx-qt` Full Node on at least one clean machine.
- [ ] Scripted E2E examples were reviewed or run against a local test node.
- [ ] Failures are documented with logs, screenshots, and remediation notes.
- [ ] Release checklist links to this completed checklist.
