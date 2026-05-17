# BTX Wallet Phase 0 First Run

This release is the official Phase 0 desktop wallet path: audited `btx-qt`
from `btxchain/btx`, packaged with reproducible build metadata, SHA256 sums,
and GPG signatures.

## 1. Verify the Download

Download the archive for your operating system, plus:

- `SHA256SUMS`
- `SHA256SUMS.asc`
- the matching artifact `.asc` signature

Verify before opening the wallet:

```bash
gpg --verify SHA256SUMS.asc SHA256SUMS
sha256sum -c SHA256SUMS
gpg --verify btx-wallet-v0.1.0-qt-linux-x86_64.tar.gz.asc btx-wallet-v0.1.0-qt-linux-x86_64.tar.gz
```

On macOS, use `shasum -a 256 -c SHA256SUMS` if `sha256sum` is unavailable.
On Windows, Git Bash, WSL, or PowerShell with GPG installed can run the same
verification flow.

## 2. Start Pruned

Phase 0 is designed for normal BTC, NEAR, and ZEC users who want a private
desktop wallet without running an archival node.

Before the first run, copy `BTX-Wallet-Phase0/btx-pruned.conf` to your BTX data
directory as `btx.conf`:

- Linux: `~/.btx/btx.conf`
- macOS: `~/Library/Application Support/BTX/btx.conf`
- Windows: `%APPDATA%\BTX\btx.conf`

The starter config enables `prune=4096`, keeps the shielded commitment index,
and does not add telemetry or phone-home services.

## 3. Create and Encrypt a Wallet

1. Open `btx-qt`.
2. Choose **Create a new wallet**.
3. Use a clear wallet name, such as `main`.
4. Enable encryption when prompted, or encrypt immediately after creation.
5. Store the passphrase offline. Losing it means losing spend access.

After encryption, close and reopen `btx-qt` once to confirm the wallet loads
cleanly.

## 4. Receive and Send

For a transparent test send:

1. Open the Receive tab.
2. Create a new receiving address.
3. Send a small amount first.
4. Wait for confirmation before sending more.

For a shielded SMILE v2 send:

1. Use a `btxs1...` shielded address.
2. Send a small test amount first.
3. Keep the wallet open until the transaction is built, broadcast, and visible
   in history.

BTX shielded transactions are built by the official audited core. The Phase 0
wallet repo does not add new cryptographic primitives.

## 5. Back Up

Back up after wallet creation and again after meaningful receive activity.
Use the official wallet backup flow from `btx-qt`, then store the backup offline.

Never share:

- wallet files
- seed or descriptor backups
- passphrases
- debug logs that may reveal local paths or operational details
