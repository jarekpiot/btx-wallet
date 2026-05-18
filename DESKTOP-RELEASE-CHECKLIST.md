# BTX Wallet Light Release Checklist

Release tag: `v0.2.0-light`

Workflow run: pending

## Build Gates

- [ ] Windows Tauri build completed.
- [ ] macOS Tauri build completed.
- [ ] Linux Tauri build completed.
- [ ] Frontend tests passed with `npm test`.
- [ ] Frontend production build passed with `npm run build`.
- [ ] Rust/Tauri build passed with locked dependencies.
- [ ] Release artifacts were hashed with SHA256.
- [ ] `SHA256SUMS` was GPG-signed.
- [ ] Each artifact has a detached `.asc` signature.

## Security Gates

- [ ] No new cryptographic primitives were added.
- [ ] All wallet/key/signing/shielded operations route through official BTX RPC.
- [ ] Tauri capabilities remain minimal.
- [ ] No telemetry, analytics, updater phone-home, or bundled remote logging.
- [ ] RPC credentials and wallet passphrases are not persisted by the app.
- [ ] Remote plain HTTP RPC is blocked.
- [ ] Remote RPC requires explicit user approval.

## Functional Gates

- [ ] App opens without requiring full-chain sync.
- [ ] Local RPC connection works.
- [ ] Trusted remote HTTPS RPC connection works.
- [ ] Encrypted descriptor wallet creation works.
- [ ] Wallet restore works.
- [ ] Wallet unlock and lock work.
- [ ] Transparent receive address generation works.
- [ ] Shielded receive address generation works.
- [ ] Transparent send works.
- [ ] SMILE v2 shielded send works.
- [ ] Redacted shielded transaction view works.
- [ ] Sensitive local shielded transaction view works after explicit toggle.
- [ ] Backup bundle creation works.
- [ ] Encrypted backup archive creation works.

## Sign-Off

Decision: pending
