# BTX Wallet Light Release Checklist

Release tag: `v0.2.0-light`

Release URL: <https://github.com/jarekpiot/btx-wallet/releases/tag/v0.2.0-light>

Successful workflow run: <https://github.com/jarekpiot/btx-wallet/actions/runs/26029737954>

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
5e73ec87e0664044ba7c238f4bb6022d644116e900b96f9a9716d02ca0926d46  btx-wallet-v0.2.0-light-linux-x86_64-BTX.Wallet.Light-0.2.0-1.x86_64.rpm
8ca467b05aa93fc6f048d4751d9054d3920255f29df85ef6989050e25d139d69  btx-wallet-v0.2.0-light-linux-x86_64-BTX.Wallet.Light_0.2.0_amd64.AppImage
6159201a1a59eb36b76339333b0109e89dd0c859321eb6c20d1e6b2c44c6168b  btx-wallet-v0.2.0-light-linux-x86_64-BTX.Wallet.Light_0.2.0_amd64.deb
553eaf8b570336c53f954ce251bb62ecb94652045c28857068a1436ddf938dc5  btx-wallet-v0.2.0-light-macos-arm64-BTX.Wallet.Light_0.2.0_aarch64.dmg
abe1455f8561e97953ca33d53522eb162086ef4f5bd43b2ef9d0fbfead53c57f  btx-wallet-v0.2.0-light-windows-x86_64-BTX.Wallet.Light_0.2.0_x64-setup.exe
3adf95f387f9036d47eb4ab1ffc87a7785e9a4f23506056ff8d30abfeffb5804  btx-wallet-v0.2.0-light-windows-x86_64-BTX.Wallet.Light_0.2.0_x64_en-US.msi
```

Public release verification was run from a fresh download directory:

```text
SHA256SUMS.asc: Good signature from "BTX Wallet Release <security@btx.dev>"
All six published desktop artifact hashes matched SHA256SUMS.
All six detached artifact signatures verified successfully.
desktop_release_verification=PASS
hardened_desktop_release_verification=PASS
```

## Blank Window Fix Verification

Root causes fixed:

- Vite now emits relative production asset paths with `base: "./"`, so Tauri's embedded webview can load `./assets/...` from the packaged app.
- The Svelte entrypoint now uses the Svelte 5 `mount(App, { target })` API instead of the legacy `new App({ target })` constructor pattern.

Regression guard:

```text
npm test: production asset path and Svelte 5 mount assertions passed
```

Packaged Windows verification:

```text
npm run tauri:build: produced MSI and NSIS bundles
target/release/btx_wallet_light.exe: UI rendered, not blank
MSI administrative payload extraction: btx_wallet_light.exe launched and UI rendered, not blank
fixed_desktop_release_verification=PASS
```

## Local Verification

```text
npm test: 4 passed, 0 failed
npm run build: vite production build passed
cargo test: 8 passed, 0 failed
cargo fmt --check: passed
scripts/verify-no-new-crypto.sh: No wallet-layer cryptographic implementation found
desktop_app_launch=PASS
```

## Focused Desktop Security Review

Review scope: Phase 1 Tauri/Svelte desktop attack surface only. BTX consensus, SMILE v2, ML-DSA, SLH-DSA, note handling, and wallet cryptography remain Phase 0/core scope.

Findings and fixes applied on 2026-05-18:

- [x] Tauri permissions reviewed: only `core:default` is granted; no shell, filesystem, dialog, updater, HTTP, clipboard, or notification plugin permission is enabled.
- [x] `withGlobalTauri` is disabled.
- [x] CSP reviewed: `default-src 'self'`; image loading restricted to app assets; connect policy only permits Tauri IPC.
- [x] Frontend XSS review passed: no `{@html}`, `innerHTML`, `eval`, `new Function`, local/session storage, clipboard use, WebSocket use, or frontend `fetch`.
- [x] Telemetry review passed: no analytics, phone-home, updater, bundled logger, or background network client in the frontend.
- [x] JSON-RPC is routed through the Rust backend only; the frontend does not directly reach arbitrary network endpoints.
- [x] Remote RPC still requires explicit user opt-in and HTTPS; local loopback HTTP remains allowed for local `btxd`.
- [x] RPC URLs now reject embedded credentials, query strings, and fragments.
- [x] IPC inputs are now bounded for RPC URL, credentials, wallet names, paths, addresses, comments, txids, and passphrases.
- [x] Wallet names are now rejected if they look like paths.
- [x] Address mode is now an explicit backend enum check (`shielded` or `transparent`), instead of defaulting unexpected input to transparent.
- [x] Amount parsing now rejects non-finite values, more than 8 decimal places, over-supply amounts, negative values, zero, and oversized amount strings.
- [x] Shielded disclosure txids are now required to be hexadecimal and length-bounded before RPC forwarding.
- [x] The visible RPC password field is cleared after a successful connection; RPC credentials remain in process memory only for the active session and are not persisted by the app.
- [x] `npm audit --audit-level=moderate` passed with 0 vulnerabilities.
- [x] Rust dependency review confirmed `reqwest` uses `rustls-tls`; no app-level wallet cryptography was introduced.

Focused review proof:

```text
rg review: no frontend HTML injection, storage, clipboard, shell, updater, telemetry, WebSocket, or direct fetch patterns found.
npm audit --audit-level=moderate: found 0 vulnerabilities
cargo tree -e features: reqwest v0.12.28 uses rustls-tls / webpki roots
scripts/verify-no-new-crypto.sh: No wallet-layer cryptographic implementation found
npm run tauri:build: produced Windows MSI and NSIS bundles after hardening changes
desktop_hardened_launch=PASS
```

Desktop residual risks:

- RPC trust remains explicit: a malicious remote `btxd` can lie about balances/history and can observe RPC metadata. Users should prefer local `btxd` or a trusted HTTPS endpoint.
- RPC credentials and wallet passphrases necessarily exist briefly in frontend/backend process memory while commands are submitted to BTX core. They are not persisted by the app, but memory zeroization is not implemented in this MVP.
- Backup and restore path access is performed by `btxd`, not by Tauri filesystem permissions. The desktop app validates input shape and length, but the node enforces actual filesystem policy.
- Release artifacts are GPG-signed and SHA256-verified. Windows Authenticode signing and Apple notarization are not yet implemented.
- macOS and Linux GUI launch remain CI build/signature verified but not manually opened in this Windows review environment.

## Phase 1.5 / Phase 2 Focused Security Delta Review

Review date: 2026-05-19

Scope: targeted review of new desktop logic added after the original Phase 1 review: onboarding and saved node profiles, shielded note health and send-readiness guidance, note consolidation UI flow, improved error messages, loading/empty states, and send button blocking. BTX consensus, SMILE v2 internals, ML-DSA, SLH-DSA, wallet cryptography, and note-spending logic remain official BTX core scope.

Findings and fixes applied:

- [x] Tauri permissions remain minimal: `core:default` only; no filesystem, shell, dialog, updater, clipboard, notification, or HTTP plugin permissions were added.
- [x] `withGlobalTauri` remains disabled and the CSP remains app-local with Tauri IPC only.
- [x] New shielded note-health, risk-label, send-readiness, empty-state, and guidance logic is frontend display logic only. It does not create keys, sign, prove, encrypt, decrypt, select notes, or implement wallet-layer cryptography.
- [x] Shielded note health uses aggregate RPC data only (`z_listunspent`) and shows counts, fragmentation guidance, and largest-note style summaries. It does not expose note secrets, nullifiers, spending keys, or shielded addresses in the UI.
- [x] Note consolidation remains an official BTX RPC flow: request a fresh shielded address with `z_getnewaddress`, then submit a self-send with `z_sendtoaddress`. The desktop app does not build shielded transactions itself.
- [x] Send button readiness is UX gating only. Backend validation remains authoritative for URL, wallet, amount, address mode, passphrase, address, comment, txid, and path bounds.
- [x] Improved error and "What to try" messages use static text and Svelte text bindings; no `{@html}`, `innerHTML`, `eval`, `new Function`, clipboard access, direct frontend network calls, or WebSocket use was found.
- [x] Onboarding saved node profiles are now hardened. The app persists only bounded public fields (`id`, `label`, `url`, `wallet`, `allowRemote`) and rejects saved URLs with embedded credentials, query strings, fragments, or non-HTTP(S) schemes.
- [x] Legacy unsafe saved node profiles are dropped when loaded or rewritten.
- [x] RPC username, RPC password, cookie path/content, and wallet passphrases are not saved in node profiles or local storage. The visible RPC password and passphrase fields continue to be cleared after successful sensitive operations where applicable.
- [x] No telemetry, analytics, updater phone-home, bundled remote logging, direct frontend `fetch`, or background network client was introduced.
- [x] No new wallet-layer cryptographic implementation was introduced.

Focused review proof:

```text
npm test: 26 passed, 0 failed
npm run build: vite production build passed
cargo test: 13 passed, 0 failed
cargo fmt --check: passed
npm audit --audit-level=moderate: found 0 vulnerabilities
scripts/verify-no-new-crypto.sh: No wallet-layer cryptographic implementation found
rg review: no frontend HTML injection, direct fetch, WebSocket, clipboard, telemetry, updater, shell, or Tauri fs/dialog/http permissions found
rg review: localStorage use is limited to sanitized saved node profiles; Rust fs use is limited to optional RPC cookie reading
npm run tauri:build: produced Windows MSI and NSIS bundles
desktop_light_launch=PASS
```

Fix verification:

```text
saved node profiles reject credential-bearing URLs
saved node profiles reject secret-like URL extras
saved node profiles only allow HTTP and HTTPS URLs
saved node profiles drop legacy unsafe entries from storage
```

Phase 1.5 / Phase 2 residual risks:

- Saved node profiles intentionally use browser `localStorage` for public metadata. A local OS account with profile access may see saved node URLs, labels, wallet labels, and the remote-node opt-in flag. The app does not store RPC credentials, cookie paths, cookie contents, or passphrases.
- RPC credentials and wallet passphrases still exist briefly in the Svelte webview state, Tauri IPC payloads, Rust process memory, and BTX RPC request path while commands run. They are cleared from visible fields after successful sensitive operations, but memory zeroization is not implemented.
- Remote `btxd` trust remains the largest light-client risk. A malicious or compromised remote node can misreport balances/history and observe RPC metadata. Users should prefer a local `btxd` or a trusted HTTPS endpoint.
- Shielded note health and consolidation readiness are guidance heuristics. The BTX core remains authoritative for note selection, fees, proving, signing, and final broadcast success.
- `z_listunspent` availability and detail quality depend on the connected node version and wallet state. Unknown note health is shown as a degraded state, not as a hard security failure.
- Windows package launch was verified locally. macOS and Linux GUI launch remain CI build/signature verified but were not manually opened in this Windows review environment.

Practical recommendations:

- Keep saved profiles limited to public metadata unless OS keychain support is added for credentials.
- Add a mocked or regtest `btxd` integration test for note-health, consolidation, and send-readiness edge cases before the next release.
- Keep remote-node UX explicit: local node first, trusted HTTPS remote only with user opt-in.
- Add Authenticode signing and Apple notarization when release infrastructure is ready.
- Continue running this focused delta review after every change to saved profiles, passphrase handling, RPC routing, or shielded send/consolidation UX.

## Rigorous Wallet-Funds Security Review

Review date: 2026-05-19

Scope: higher-rigor review of BTX Wallet Light fund-loss and privacy risks in the Tauri/Svelte desktop layer. This review covered user/RPC input validation, shielded send and consolidation flows, note-health guidance, sensitive-field handling, Tauri permissions, IPC, local storage, and release build behavior. It did not re-audit BTX consensus, SMILE v2 proving, ML-DSA, SLH-DSA, note selection, or wallet cryptography, which remain official BTX core scope.

Severity-rated findings:

- [x] Critical: none found.
- [x] High: none remaining after review.
- [x] Medium: send and consolidation amounts were parsed through `f64` before JSON-RPC submission. Floating-point conversion is inappropriate for wallet send amounts because it can lose exact decimal intent. Fixed by validating BTX amounts as bounded plain decimal strings, rejecting signs, exponent notation, zero, over-supply values, and more than 8 decimal places, then forwarding the canonical string to BTX core.
- [x] Medium: existing wallet unlock used the new-wallet minimum passphrase policy. This could block users from unlocking older/restored wallets with shorter existing passphrases through the light client. Fixed by keeping the 12-character minimum for new wallet/archive passphrases while allowing any non-empty bounded passphrase for existing wallet unlock.
- [x] Medium: send and shielded consolidation actions did not require a final confirmation immediately before asking BTX core to broadcast. Fixed by adding explicit confirmation prompts that show amount, mode/action, address preview, irreversibility, fee, and confirmation expectations.
- [x] Low: shielded disclosure txid validation accepted arbitrary-length hexadecimal strings. Fixed by requiring exactly 64 hexadecimal characters before forwarding to `z_viewtransaction`.
- [x] Low: source-only crypto scan confirmed desktop code contains display/documentation references to encryption and SMILE v2 only; no wallet-layer cryptographic implementation was added.
- [x] Low: frontend attack-surface scan found no HTML injection, `eval`, direct frontend network calls, WebSocket, telemetry, clipboard, shell, updater, filesystem, or dialog permissions. The only intentional browser storage remains sanitized saved node profile metadata.

Fixes applied:

- [x] Exact decimal amount validation and RPC forwarding in `desktop/src-tauri/src/lib.rs`.
- [x] Separate new-wallet and existing-wallet passphrase validators in `desktop/src-tauri/src/lib.rs`.
- [x] Strict 64-character hexadecimal txid validation in `desktop/src-tauri/src/lib.rs`.
- [x] Final user confirmation before transparent send, shielded send, and shielded note consolidation in `desktop/src/App.svelte`.
- [x] Regression tests added for exact amount validation, txid validation, and legacy-short existing wallet unlock.

Rigorous review proof:

```text
npm test: 26 passed, 0 failed
npm run build: vite production build passed
cargo test: 14 passed, 0 failed
cargo fmt --check: passed
npm audit --audit-level=moderate: found 0 vulnerabilities
npm run tauri:build: produced Windows MSI and NSIS bundles
source crypto scan: no desktop wallet-layer crypto implementation found; sensitive words are UI/docs/dependency-lock references only
attack-surface scan: no frontend HTML injection, eval, direct fetch, WebSocket, telemetry, updater, shell, clipboard, or Tauri fs/dialog/http permissions found
```

Tooling note: `scripts/verify-no-new-crypto.sh` is Bash-based and could not run directly in this Windows shell because `bash` resolves to WSL and no Linux distribution is installed. The equivalent checks were run with `git ls-files` and `rg` over tracked desktop source files.

Current release-safety decision:

- Safe for wider release after CI completes and artifacts are signed/verified.
- No known Critical or High findings remain open in the desktop wallet layer.
- The Medium findings identified in this review were fixed and covered by tests/build verification.

Residual risks:

- Remote `btxd` trust remains the largest light-client risk. A malicious or compromised remote node can misreport balances/history, observe RPC metadata, and degrade shielded operation reliability. UX should continue steering users toward local `btxd` or trusted HTTPS endpoints.
- RPC credentials, wallet passphrases, and send details necessarily pass through Svelte state, Tauri IPC, Rust memory, and the BTX RPC request path during operations. They are not persisted by the app, but memory zeroization is not implemented.
- Address validity and address-family correctness are still delegated to BTX core. The app length-bounds and confirms user intent, but it does not implement wallet-layer address parsing.
- Shielded note-health and send-readiness warnings are heuristics. BTX core remains authoritative for note selection, fees, proving, signing, and broadcast success.
- The final confirmation prompt reduces accidental sends and address-poisoning mistakes, but it is not a hardware-wallet-grade trusted display.
- macOS and Linux GUI launch were not manually opened in this Windows review environment; CI/release signing should remain mandatory for those platforms.

Practical recommendations:

- Add a mocked or regtest integration harness for amount forwarding, send confirmation, shielded note-health edge cases, and consolidation failures.
- Consider OS keychain support before ever persisting RPC credentials.
- Consider optional address book / verified recipient labels to reduce address-poisoning risk without weakening privacy defaults.
- Keep the desktop app as a control surface over BTX core; do not add wallet-layer signing, proving, note selection, encryption, or key-handling logic.

## Sign-Off

Approver: automated Phase 1 desktop release gate

Decision: GO for `v0.2.0-light` MVP release.

Residual risk: macOS and Linux GUI launch were not manually opened in this Windows environment. The platform installers were built in GitHub Actions, signed, hash-verified, and published. Live wallet transaction execution is covered by the Phase 0 BTX core smoke proof; Phase 1 forwards those same operations to official BTX RPC without adding wallet-layer cryptography.
