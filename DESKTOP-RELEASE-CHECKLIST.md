# BTX Wallet Light Release Checklist

Release tag: `v0.2.0-light`

Release URL: <https://github.com/jarekpiot/btx-wallet/releases/tag/v0.2.0-light>

Successful workflow run: <https://github.com/jarekpiot/btx-wallet/actions/runs/26027012066>

Review date: 2026-05-18

## Build Gates

- [x] Windows Tauri build completed.
- [x] macOS Tauri build completed.
- [x] Linux Tauri build completed.
- [x] Frontend tests passed with `npm test`.
- [x] Frontend production build passed with `npm run build`.
- [x] Rust backend tests passed with locked dependencies.
- [x] Rust formatting passed with `cargo fmt --check`.
- [x] Local Windows desktop binary launched without requiring full-chain sync.
- [x] Release artifacts were hashed with SHA256.
- [x] `SHA256SUMS` was GPG-signed.
- [x] Each artifact has a detached `.asc` signature.

## Security Gates

- [x] No new cryptographic primitives were added.
- [x] All wallet/key/signing/shielded operations route through official BTX RPC.
- [x] Tauri capabilities remain minimal.
- [x] No telemetry, analytics, updater phone-home, or bundled remote logging.
- [x] RPC credentials and wallet passphrases are not persisted by the app.
- [x] Remote plain HTTP RPC is blocked.
- [x] Remote RPC requires explicit user approval.

## Functional Gates

- [x] App opens without requiring full-chain sync.
- [x] Local RPC connection path works in backend validation (`http://127.0.0.1` and `http://localhost` allowed).
- [x] Trusted remote HTTPS RPC connection path works in backend validation.
- [x] Remote HTTP and unapproved remote RPC are rejected.
- [x] Encrypted descriptor wallet creation is implemented by the `createwallet` RPC command.
- [x] Wallet restore is implemented by the `restorewallet` RPC command.
- [x] Wallet unlock and lock are implemented by `walletpassphrase` and `walletlock`.
- [x] Transparent receive address generation is implemented by `getnewaddress`.
- [x] Shielded receive address generation is implemented by `z_getnewaddress`.
- [x] Transparent send is implemented by `sendtoaddress`.
- [x] SMILE v2 shielded send is implemented by `z_sendtoaddress`.
- [x] Redacted shielded transaction view is implemented by `z_viewtransaction` with sensitive output disabled.
- [x] Sensitive local shielded transaction view is implemented by explicit `z_viewtransaction` opt-in.
- [x] Backup bundle creation is implemented by `backupwalletbundle`.
- [x] Encrypted backup archive creation is implemented by `backupwalletbundlearchive`.

Functional note: Phase 1 is a light-client control surface over trusted BTX JSON-RPC. The live wallet transaction proof remains the Phase 0 BTX core smoke test against `btxd`/`btx-cli`, where encrypted wallet creation, backup, restore, transparent send, shielded SMILE v2 send, and shielded balance checks passed. The Phase 1 app does not implement those sensitive operations; it forwards to the audited BTX core methods listed above.

## Published Artifacts

Release GPG fingerprint: `599F9E7A4192E1BD7CEBA82ABB9A6F689BB11C30`

```text
efe5c4fabab34e4111f45482df2bd6708418fa037df31074069deb7f54f753b5  btx-wallet-v0.2.0-light-linux-x86_64-BTX.Wallet.Light-0.2.0-1.x86_64.rpm
5fcbdfdea8fc60009fc226e1d0e3620a9aa5ac8f19ab27239966e9f054180be4  btx-wallet-v0.2.0-light-linux-x86_64-BTX.Wallet.Light_0.2.0_amd64.AppImage
756dc1e01f5d74b44398559d9fcbf810bc96bf4517bff370f79aac8da73d538d  btx-wallet-v0.2.0-light-linux-x86_64-BTX.Wallet.Light_0.2.0_amd64.deb
a5349bd4d6c9cbf76d44531900bc6f270b962bdc0c7ab6c2ef31dab193857468  btx-wallet-v0.2.0-light-macos-arm64-BTX.Wallet.Light_0.2.0_aarch64.dmg
cd248f38966ab49346a1aef84d024ab3902882848fc2217b37e496e2e776281c  btx-wallet-v0.2.0-light-windows-x86_64-BTX.Wallet.Light_0.2.0_x64-setup.exe
072bbb824e8916c1176e25b56934b99a0825f6c68f980103a27987803ddaab49  btx-wallet-v0.2.0-light-windows-x86_64-BTX.Wallet.Light_0.2.0_x64_en-US.msi
```

Public release verification was run from a fresh download directory:

```text
SHA256SUMS.asc: Good signature from "BTX Wallet Release <security@btx.dev>"
All six published desktop artifact hashes matched SHA256SUMS.
All six detached artifact signatures verified successfully.
desktop_release_verification=PASS
```

## Local Verification

```text
npm test: 2 passed, 0 failed
npm run build: vite production build passed
cargo test: 4 passed, 0 failed
cargo fmt --check: passed
scripts/verify-no-new-crypto.sh: No wallet-layer cryptographic implementation found
desktop_app_launch=PASS
```

## Sign-Off

Approver: automated Phase 1 desktop release gate

Decision: GO for `v0.2.0-light` MVP release.

Residual risk: macOS and Linux GUI launch were not manually opened in this Windows environment. The platform installers were built in GitHub Actions, signed, hash-verified, and published. Live wallet transaction execution is covered by the Phase 0 BTX core smoke proof; Phase 1 forwards those same operations to official BTX RPC without adding wallet-layer cryptography.
