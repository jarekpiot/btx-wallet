# BTX Wallet Light Manual E2E Checklist

Use this checklist before publishing a BTX Wallet Light release. Test with a
local `btxd` node when possible. Record the app version, node version, network,
OS, and any failed commands in the release checklist.

## Test Setup

- [ ] Install the release candidate on a clean Windows, macOS, or Linux machine.
- [ ] Start a trusted local `btxd` node.
- [ ] Confirm `btx-cli getblockchaininfo` works against the same node.
- [ ] Confirm the app launches without a blank window or console errors.
- [ ] Confirm no telemetry, analytics, or unexpected network requests are made.

## Node Connection

- [ ] Open Settings.
- [ ] Add or select a local node profile.
- [ ] Connect to `http://127.0.0.1:18443` or the local node URL for the test network.
- [ ] Verify connection status shows connected.
- [ ] Stop the node and confirm the app shows a clear connection error.
- [ ] Restart the node and confirm the app recovers after reconnect/refresh.

## Wallet Create / Restore

- [ ] Create a new encrypted descriptor wallet.
- [ ] Confirm the app requires a non-empty passphrase.
- [ ] Lock and unlock the wallet.
- [ ] Confirm sensitive passphrase fields clear after use.
- [ ] Create a backup bundle.
- [ ] Restore the backup into a fresh wallet name.
- [ ] Confirm restored balances and addresses match expectations.

## Receive Flow

- [ ] Generate a transparent receive address.
- [ ] Generate a shielded receive address.
- [ ] Confirm receive empty states and address safety guidance are clear.
- [ ] Copy each address and verify it pastes exactly.
- [ ] Confirm the app does not silently copy seed phrases, keys, or passphrases.

## Transparent Send

- [ ] Fund the wallet with transparent test funds.
- [ ] Send a small transparent transaction to a second wallet/address.
- [ ] Confirm the send button shows an in-progress state.
- [ ] Confirm success feedback includes a txid.
- [ ] Confirm transaction history updates after confirmation.
- [ ] Try an invalid transparent address and confirm the error is actionable.
- [ ] Try an amount larger than balance and confirm the error is actionable.

## Shielded Send

- [ ] Fund the wallet with shielded test funds.
- [ ] Refresh balances and note health.
- [ ] Confirm shielded note count, fragmentation, and largest-note guidance are visible.
- [ ] Send a small SMILE v2 shielded transaction to another wallet/address.
- [ ] Confirm the app shows clear in-progress feedback.
- [ ] Confirm success feedback includes a txid.
- [ ] Confirm receiver balance updates after confirmation.
- [ ] Verify selective disclosure for the shielded tx.
- [ ] Try an invalid shielded address and confirm the error is actionable.
- [ ] Try a shielded amount larger than balance and confirm the error is actionable.

## Large / Complex Shielded Send

- [ ] Prepare a wallet with many small shielded notes.
- [ ] Refresh note health.
- [ ] Confirm high fragmentation or high-risk send guidance appears.
- [ ] Attempt a larger shielded send that is near note limits.
- [ ] Confirm the app warns before or during the send flow.
- [ ] Confirm guidance suggests note consolidation or splitting the payment.
- [ ] Confirm no sensitive note details are exposed unnecessarily.

## Note Consolidation

- [ ] Open the shielded send/consolidation area.
- [ ] Review consolidation copy and expectations.
- [ ] Run a basic note consolidation to a fresh shielded address in the same wallet.
- [ ] Confirm the app shows in-progress and success feedback.
- [ ] Wait for confirmation.
- [ ] Refresh note health and confirm fragmentation improves or guidance changes.
- [ ] Confirm the consolidation transaction appears in history.

## Error Guidance

- [ ] Test wrong RPC URL.
- [ ] Test wrong RPC username/password if auth is enabled.
- [ ] Test locked wallet send attempt.
- [ ] Test node not synced.
- [ ] Test insufficient transparent balance.
- [ ] Test insufficient shielded balance.
- [ ] Test fragmented-note or note-limit failure where possible.
- [ ] Confirm each error includes a clear "What to try" style next step.

## Release Sign-Off

- [ ] Manual E2E passed on at least one clean machine.
- [ ] Scripted E2E examples were reviewed or run against a local test node.
- [ ] Failures are documented with logs, screenshots, and remediation notes.
- [ ] Release checklist links to this completed checklist.
