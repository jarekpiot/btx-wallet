use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::sync::Mutex;
use std::time::Duration;
use tauri::State;
use thiserror::Error;

const MAX_RPC_URL_LEN: usize = 512;
const MAX_RPC_CREDENTIAL_LEN: usize = 256;
const MAX_WALLET_NAME_LEN: usize = 64;
const MAX_WALLET_PASSPHRASE_LEN: usize = 1024;
const MAX_PATH_LEN: usize = 1024;
const MAX_ADDRESS_LEN: usize = 512;
const MAX_COMMENT_LEN: usize = 80;
const TXID_HEX_LEN: usize = 64;
const MAX_AMOUNT_LEN: usize = 32;
const MAX_BTX_AMOUNT_WHOLE: &str = "21000000";
const NOTE_WARN_THRESHOLD: usize = 32;
const NOTE_HIGH_THRESHOLD: usize = 64;
const SMALL_NOTE_BTX: f64 = 0.01;

#[derive(Debug, Error)]
enum WalletError {
    #[error("{0}")]
    Message(String),
    #[error("{0}")]
    Rpc(String),
    #[error("RPC auth cookie could not be read: {0}")]
    Io(#[from] std::io::Error),
}

impl From<reqwest::Error> for WalletError {
    fn from(error: reqwest::Error) -> Self {
        WalletError::Rpc(explain_transport_error(&error.to_string()))
    }
}

impl serde::Serialize for WalletError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

type WalletResult<T> = Result<T, WalletError>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RpcConfig {
    url: String,
    username: Option<String>,
    password: Option<String>,
    cookie_path: Option<String>,
    wallet: Option<String>,
    allow_remote: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Overview {
    connected: bool,
    node: Option<Value>,
    wallet: Option<Value>,
    balances: Option<Balances>,
    shielded_note_summary: Option<ShieldedNoteSummary>,
    transactions: Vec<Value>,
    configured_wallet: Option<String>,
}

#[derive(Debug, Serialize)]
struct Balances {
    transparent: f64,
    shielded: f64,
    total: f64,
    immature: f64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ShieldedNoteSummary {
    available: bool,
    note_count: usize,
    spendable_count: usize,
    immature_count: usize,
    total_amount: f64,
    largest_note: f64,
    small_note_count: usize,
    complexity: String,
    guidance: Vec<String>,
    unavailable_reason: Option<String>,
}

struct AppState {
    config: Mutex<Option<RpcConfig>>,
    client: Client,
}

impl AppState {
    fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("BTX-Wallet-Light/0.2.0")
            .build()
            .expect("reqwest client");
        Self {
            config: Mutex::new(None),
            client,
        }
    }
}

#[derive(Debug, Deserialize)]
struct RpcEnvelope {
    result: Option<Value>,
    error: Option<Value>,
}

fn validate_config(config: &RpcConfig) -> WalletResult<()> {
    ensure_len("RPC URL", config.url.trim(), 1, MAX_RPC_URL_LEN)?;
    if let Some(username) = &config.username {
        ensure_len("RPC username", username, 0, MAX_RPC_CREDENTIAL_LEN)?;
    }
    if let Some(password) = &config.password {
        ensure_len("RPC password", password, 0, MAX_RPC_CREDENTIAL_LEN)?;
    }
    if let Some(cookie_path) = &config.cookie_path {
        ensure_len("RPC cookie path", cookie_path.trim(), 0, MAX_PATH_LEN)?;
    }
    if let Some(wallet) = &config.wallet {
        validate_wallet_name(wallet)?;
    }

    let parsed = reqwest::Url::parse(config.url.trim())
        .map_err(|_| WalletError::Message("RPC URL is not valid.".to_string()))?;
    match parsed.scheme() {
        "http" | "https" => {}
        _ => {
            return Err(WalletError::Message(
                "RPC URL must use http or https.".to_string(),
            ))
        }
    }
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err(WalletError::Message(
            "Put RPC credentials in the username/password fields, not in the URL.".to_string(),
        ));
    }
    if parsed.query().is_some() || parsed.fragment().is_some() {
        return Err(WalletError::Message(
            "RPC URL must not include query strings or fragments.".to_string(),
        ));
    }

    let host = parsed.host_str().unwrap_or_default();
    let loopback = matches!(host, "127.0.0.1" | "localhost" | "::1");
    if !loopback && !config.allow_remote {
        return Err(WalletError::Message(
            "Remote RPC endpoints must be explicitly allowed.".to_string(),
        ));
    }
    if !loopback && parsed.scheme() != "https" {
        return Err(WalletError::Message(
            "Remote RPC over plain HTTP is blocked. Use an HTTPS tunnel or connect locally."
                .to_string(),
        ));
    }
    Ok(())
}

fn wallet_path_segment(wallet: &str) -> String {
    let mut output = String::new();
    for byte in wallet.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                output.push(*byte as char)
            }
            _ => output.push_str(&format!("%{byte:02X}")),
        }
    }
    output
}

fn method_url(config: &RpcConfig, wallet: Option<&str>) -> WalletResult<String> {
    let base = config.url.trim().trim_end_matches('/');
    if let Some(wallet) = wallet {
        if !wallet.is_empty() {
            return Ok(format!("{base}/wallet/{}", wallet_path_segment(wallet)));
        }
    }
    Ok(base.to_string())
}

fn cookie_auth(config: &RpcConfig) -> WalletResult<Option<(String, String)>> {
    if let (Some(username), Some(password)) = (&config.username, &config.password) {
        if !username.is_empty() || !password.is_empty() {
            return Ok(Some((username.clone(), password.clone())));
        }
    }
    if let Some(path) = &config.cookie_path {
        if !path.trim().is_empty() {
            let cookie = fs::read_to_string(path.trim())?;
            let (username, password) = cookie
                .trim()
                .split_once(':')
                .ok_or_else(|| WalletError::Message("RPC cookie format is invalid.".to_string()))?;
            return Ok(Some((username.to_string(), password.to_string())));
        }
    }
    Ok(None)
}

fn ensure_len(label: &str, value: &str, min: usize, max: usize) -> WalletResult<()> {
    let len = value.len();
    if len < min || len > max {
        return Err(WalletError::Message(format!(
            "{label} length must be between {min} and {max} bytes."
        )));
    }
    Ok(())
}

fn validate_wallet_name(wallet_name: &str) -> WalletResult<&str> {
    let name = wallet_name.trim();
    ensure_len("Wallet name", name, 1, MAX_WALLET_NAME_LEN)?;
    if name.contains('/') || name.contains('\\') || name == "." || name == ".." {
        return Err(WalletError::Message(
            "Wallet name must be a simple name, not a path.".to_string(),
        ));
    }
    Ok(name)
}

fn validate_path_input<'a>(label: &str, path: &'a str, required: bool) -> WalletResult<&'a str> {
    let trimmed = path.trim();
    ensure_len(label, trimmed, usize::from(required), MAX_PATH_LEN)?;
    Ok(trimmed)
}

fn validate_address(address: &str) -> WalletResult<&str> {
    let trimmed = address.trim();
    ensure_len("Address", trimmed, 1, MAX_ADDRESS_LEN)?;
    Ok(trimmed)
}

fn validate_txid(txid: &str) -> WalletResult<&str> {
    let trimmed = txid.trim();
    if trimmed.len() != TXID_HEX_LEN {
        return Err(WalletError::Message(
            "Transaction id must be exactly 64 hexadecimal characters.".to_string(),
        ));
    }
    if !trimmed
        .chars()
        .all(|character| character.is_ascii_hexdigit())
    {
        return Err(WalletError::Message(
            "Transaction id must be hexadecimal.".to_string(),
        ));
    }
    Ok(trimmed)
}

fn validate_new_wallet_passphrase(passphrase: &str) -> WalletResult<()> {
    ensure_len(
        "Wallet passphrase",
        passphrase,
        12,
        MAX_WALLET_PASSPHRASE_LEN,
    )
}

fn validate_existing_wallet_passphrase(passphrase: &str) -> WalletResult<()> {
    ensure_len(
        "Wallet passphrase",
        passphrase,
        1,
        MAX_WALLET_PASSPHRASE_LEN,
    )
}

async fn rpc_call(
    state: &AppState,
    config: &RpcConfig,
    wallet: Option<&str>,
    method: &str,
    params: Value,
) -> WalletResult<Value> {
    let url = method_url(config, wallet)?;
    let mut request = state.client.post(url).json(&json!({
        "jsonrpc": "1.0",
        "id": "btx-wallet-light",
        "method": method,
        "params": params,
    }));

    if let Some((username, password)) = cookie_auth(config)? {
        request = request.basic_auth(username, Some(password));
    }

    let envelope: RpcEnvelope = request.send().await?.error_for_status()?.json().await?;
    if let Some(error) = envelope.error {
        return Err(WalletError::Message(format_rpc_error(&error)));
    }
    Ok(envelope.result.unwrap_or(Value::Null))
}

fn format_rpc_error(error: &Value) -> String {
    let message = if let Some(message) = error.get("message").and_then(Value::as_str) {
        message.to_string()
    } else {
        error.to_string()
    };
    explain_rpc_error(&message)
}

fn explain_rpc_error(message: &str) -> String {
    let lower = message.to_ascii_lowercase();
    let hint = if lower.contains("walletpassphrase")
        || lower.contains("wallet is locked")
        || lower.contains("unlock")
    {
        Some("Unlock the wallet temporarily in Settings, then try again.")
    } else if lower.contains("too many")
        || lower.contains("limit")
        || lower.contains("transaction too large")
        || lower.contains("too large")
    {
        Some("This shielded send may require too many notes. Try consolidating first, or split the payment into smaller shielded sends.")
    } else if lower.contains("no spendable")
        || lower.contains("spendable note")
        || lower.contains("no notes")
    {
        Some("Wait for confirmations, refresh the wallet, and check shielded note health before retrying.")
    } else if lower.contains("fee") || lower.contains("dust") {
        Some("Leave more BTX available for fees, or try a slightly smaller shielded amount.")
    } else if lower.contains("mempool") || lower.contains("broadcast") || lower.contains("rejected")
    {
        Some("The transaction was built but not accepted for broadcast. Refresh the wallet, confirm the node is synced, and retry with a smaller amount if needed.")
    } else if lower.contains("rescan")
        || lower.contains("scanning")
        || lower.contains("scan")
        || lower.contains("sync")
    {
        Some("Let the node finish syncing or wallet scanning, then refresh the wallet before retrying shielded operations.")
    } else if lower.contains("insufficient") || lower.contains("not enough") {
        Some("Check the available balance and fee reserve. For shielded sends, fragmented notes may need consolidation first.")
    } else if lower.contains("note") || lower.contains("anchor") || lower.contains("witness") {
        Some("Refresh the wallet and let the node finish scanning. If the wallet has many small shielded notes, consolidate or split the send.")
    } else if lower.contains("timeout") || lower.contains("timed out") {
        Some("The node took too long to answer. Check that btxd is running, synced, and reachable, then retry.")
    } else if lower.contains("connection refused") || lower.contains("connect") {
        Some("Check the RPC URL, credentials, and whether btxd is running.")
    } else if lower.contains("invalid address") || lower.contains("address") {
        Some("Confirm the recipient address and selected send mode match.")
    } else {
        None
    };

    match hint {
        Some(hint) => format!("{message}\n\nWhat to try: {hint}"),
        None => message.to_string(),
    }
}

fn explain_transport_error(message: &str) -> String {
    let lower = message.to_ascii_lowercase();
    let hint = if lower.contains("timed out") || lower.contains("timeout") {
        Some("The node did not answer in time. Check that btxd is running, synced, and not overloaded.")
    } else if lower.contains("connection refused") || lower.contains("error trying to connect") {
        Some("Start btxd, confirm the RPC port, and check the RPC URL in Settings.")
    } else if lower.contains("401") || lower.contains("unauthorized") {
        Some("Check the RPC username/password or local cookie path.")
    } else if lower.contains("403") || lower.contains("forbidden") {
        Some("Check btxd RPC allowlist settings and make sure this wallet is allowed to connect.")
    } else if lower.contains("404") || lower.contains("not found") {
        Some("Check the wallet name. The node may be reachable, but the wallet endpoint is not loaded.")
    } else if lower.contains("certificate") || lower.contains("tls") {
        Some("Remote RPC must use a valid HTTPS endpoint. For local nodes, use http://127.0.0.1 with the local RPC port.")
    } else {
        None
    };

    match hint {
        Some(hint) => format!("RPC request failed: {message}\n\nWhat to try: {hint}"),
        None => format!("RPC request failed: {message}"),
    }
}

fn current_config(state: &State<'_, AppState>) -> WalletResult<RpcConfig> {
    state
        .config
        .lock()
        .map_err(|_| WalletError::Message("Connection state is unavailable.".to_string()))?
        .clone()
        .ok_or_else(|| WalletError::Message("Connect to a BTX node first.".to_string()))
}

async fn build_overview(state: &State<'_, AppState>, config: &RpcConfig) -> WalletResult<Overview> {
    let node = rpc_call(state, config, None, "getblockchaininfo", json!([])).await?;
    let configured_wallet = config.wallet.clone();

    let wallet = match configured_wallet.as_deref() {
        Some(wallet_name) if !wallet_name.is_empty() => {
            rpc_call(state, config, Some(wallet_name), "getwalletinfo", json!([]))
                .await
                .ok()
        }
        _ => None,
    };

    let balances = match configured_wallet.as_deref() {
        Some(wallet_name) if !wallet_name.is_empty() => {
            Some(read_balances(state, config, wallet_name).await?)
        }
        _ => None,
    };

    let shielded_note_summary = match configured_wallet.as_deref() {
        Some(wallet_name) if !wallet_name.is_empty() => {
            Some(read_shielded_note_summary(state, config, wallet_name).await)
        }
        _ => None,
    };

    let transactions = match configured_wallet.as_deref() {
        Some(wallet_name) if !wallet_name.is_empty() => rpc_call(
            state,
            config,
            Some(wallet_name),
            "listtransactions",
            json!(["*", 25, 0, true]),
        )
        .await
        .ok()
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default(),
        _ => Vec::new(),
    };

    Ok(Overview {
        connected: true,
        node: Some(node),
        wallet,
        balances,
        shielded_note_summary,
        transactions,
        configured_wallet,
    })
}

async fn read_balances(
    state: &State<'_, AppState>,
    config: &RpcConfig,
    wallet: &str,
) -> WalletResult<Balances> {
    let getbalances = rpc_call(state, config, Some(wallet), "getbalances", json!([])).await?;
    let transparent = getbalances
        .pointer("/mine/trusted")
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let immature = getbalances
        .pointer("/mine/immature")
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let shielded = rpc_call(state, config, Some(wallet), "z_gettotalbalance", json!([1]))
        .await
        .ok()
        .and_then(|value| {
            value
                .get("shielded")
                .or_else(|| value.get("private"))
                .and_then(|inner| {
                    if let Some(number) = inner.as_f64() {
                        Some(number)
                    } else {
                        inner.as_str().and_then(|text| text.parse::<f64>().ok())
                    }
                })
        })
        .unwrap_or(0.0);

    Ok(Balances {
        transparent,
        shielded,
        total: transparent + shielded,
        immature,
    })
}

async fn read_shielded_note_summary(
    state: &State<'_, AppState>,
    config: &RpcConfig,
    wallet: &str,
) -> ShieldedNoteSummary {
    match rpc_call(
        state,
        config,
        Some(wallet),
        "z_listunspent",
        json!([1, 9999999]),
    )
    .await
    {
        Ok(value) => summarize_shielded_notes(&value),
        Err(error) => ShieldedNoteSummary {
            available: false,
            note_count: 0,
            spendable_count: 0,
            immature_count: 0,
            total_amount: 0.0,
            largest_note: 0.0,
            small_note_count: 0,
            complexity: "unknown".to_string(),
            guidance: vec![
                "Shielded note detail is unavailable from this node. You can still send, but large shielded sends may be less predictable.".to_string(),
            ],
            unavailable_reason: Some(error.to_string()),
        },
    }
}

fn summarize_shielded_notes(value: &Value) -> ShieldedNoteSummary {
    let notes = value.as_array().cloned().unwrap_or_default();
    let mut note_count = 0usize;
    let mut spendable_count = 0usize;
    let mut immature_count = 0usize;
    let mut total_amount = 0.0f64;
    let mut largest_note = 0.0f64;
    let mut small_note_count = 0usize;

    for note in notes {
        let amount = value_amount(&note).unwrap_or(0.0);
        if amount <= 0.0 {
            continue;
        }
        note_count += 1;
        total_amount += amount;
        largest_note = largest_note.max(amount);
        if amount < SMALL_NOTE_BTX {
            small_note_count += 1;
        }
        let spendable = note
            .get("spendable")
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let confirmations = note
            .get("confirmations")
            .and_then(Value::as_u64)
            .unwrap_or(1);
        if spendable && confirmations > 0 {
            spendable_count += 1;
        } else {
            immature_count += 1;
        }
    }

    let mut guidance = Vec::new();
    let complexity = if spendable_count >= NOTE_HIGH_THRESHOLD {
        guidance.push("This wallet has many spendable shielded notes. Large sends may hit note limits or take longer to build.".to_string());
        guidance.push("Consider consolidating to a fresh shielded address or splitting the payment into smaller sends.".to_string());
        "high"
    } else if spendable_count >= NOTE_WARN_THRESHOLD || small_note_count >= NOTE_WARN_THRESHOLD {
        guidance.push("Shielded balance is fragmented across many notes. Most sends should work, but large sends may need consolidation.".to_string());
        "medium"
    } else {
        guidance.push("Shielded note count looks manageable for normal sends.".to_string());
        "low"
    };

    if immature_count > 0 {
        guidance.push("Some shielded notes are not spendable yet. Wait for confirmations before sending the full balance.".to_string());
    }

    ShieldedNoteSummary {
        available: true,
        note_count,
        spendable_count,
        immature_count,
        total_amount,
        largest_note,
        small_note_count,
        complexity: complexity.to_string(),
        guidance,
        unavailable_reason: None,
    }
}

fn value_amount(value: &Value) -> Option<f64> {
    value
        .get("amount")
        .or_else(|| value.get("value"))
        .and_then(|amount| {
            if let Some(number) = amount.as_f64() {
                Some(number)
            } else {
                amount.as_str().and_then(|text| text.parse::<f64>().ok())
            }
        })
}

#[tauri::command]
async fn configure_connection(
    config: RpcConfig,
    state: State<'_, AppState>,
) -> WalletResult<Overview> {
    validate_config(&config)?;
    rpc_call(&state, &config, None, "getblockchaininfo", json!([])).await?;
    {
        let mut guard = state
            .config
            .lock()
            .map_err(|_| WalletError::Message("Connection state is unavailable.".to_string()))?;
        *guard = Some(config.clone());
    }
    build_overview(&state, &config).await
}

#[tauri::command]
async fn get_overview(state: State<'_, AppState>) -> WalletResult<Overview> {
    let config = match state.config.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => None,
    };
    match config {
        Some(config) => build_overview(&state, &config).await,
        None => Ok(Overview {
            connected: false,
            node: None,
            wallet: None,
            balances: None,
            shielded_note_summary: None,
            transactions: Vec::new(),
            configured_wallet: None,
        }),
    }
}

#[tauri::command]
async fn create_wallet(
    wallet_name: String,
    passphrase: String,
    state: State<'_, AppState>,
) -> WalletResult<Overview> {
    let mut config = current_config(&state)?;
    let name = validate_wallet_name(&wallet_name)?;
    validate_new_wallet_passphrase(&passphrase)?;
    rpc_call(
        &state,
        &config,
        None,
        "createwallet",
        json!({
            "wallet_name": name,
            "disable_private_keys": false,
            "blank": false,
            "passphrase": passphrase,
            "avoid_reuse": false,
            "descriptors": true,
            "load_on_startup": false,
            "external_signer": false
        }),
    )
    .await?;
    config.wallet = Some(name.to_string());
    {
        let mut guard = state
            .config
            .lock()
            .map_err(|_| WalletError::Message("Connection state is unavailable.".to_string()))?;
        *guard = Some(config.clone());
    }
    build_overview(&state, &config).await
}

#[tauri::command]
async fn restore_wallet(
    wallet_name: String,
    backup_file: String,
    state: State<'_, AppState>,
) -> WalletResult<Overview> {
    let mut config = current_config(&state)?;
    let name = validate_wallet_name(&wallet_name)?;
    let backup = validate_path_input("Backup file", &backup_file, true)?;
    rpc_call(
        &state,
        &config,
        None,
        "restorewallet",
        json!({
            "wallet_name": name,
            "backup_file": backup,
            "load_on_startup": false
        }),
    )
    .await?;
    config.wallet = Some(name.to_string());
    {
        let mut guard = state
            .config
            .lock()
            .map_err(|_| WalletError::Message("Connection state is unavailable.".to_string()))?;
        *guard = Some(config.clone());
    }
    build_overview(&state, &config).await
}

#[tauri::command]
async fn unlock_wallet(
    passphrase: String,
    seconds: u64,
    state: State<'_, AppState>,
) -> WalletResult<Overview> {
    let config = current_config(&state)?;
    let wallet = config
        .wallet
        .as_deref()
        .ok_or_else(|| WalletError::Message("No wallet is configured.".to_string()))?;
    let bounded_seconds = seconds.clamp(30, 1800);
    validate_existing_wallet_passphrase(&passphrase)?;
    rpc_call(
        &state,
        &config,
        Some(wallet),
        "walletpassphrase",
        json!([passphrase, bounded_seconds]),
    )
    .await?;
    build_overview(&state, &config).await
}

#[tauri::command]
async fn lock_wallet(state: State<'_, AppState>) -> WalletResult<Overview> {
    let config = current_config(&state)?;
    let wallet = config
        .wallet
        .as_deref()
        .ok_or_else(|| WalletError::Message("No wallet is configured.".to_string()))?;
    rpc_call(&state, &config, Some(wallet), "walletlock", json!([])).await?;
    build_overview(&state, &config).await
}

#[tauri::command]
async fn new_address(mode: String, state: State<'_, AppState>) -> WalletResult<String> {
    let config = current_config(&state)?;
    let wallet = config
        .wallet
        .as_deref()
        .ok_or_else(|| WalletError::Message("No wallet is configured.".to_string()))?;
    let method = match mode.as_str() {
        "shielded" => "z_getnewaddress",
        "transparent" => "getnewaddress",
        _ => {
            return Err(WalletError::Message(
                "Address mode must be shielded or transparent.".to_string(),
            ))
        }
    };
    rpc_call(&state, &config, Some(wallet), method, json!([]))
        .await?
        .as_str()
        .map(ToString::to_string)
        .ok_or_else(|| {
            WalletError::Message("Node returned an invalid address response.".to_string())
        })
}

#[tauri::command]
async fn send_transparent(
    address: String,
    amount: String,
    state: State<'_, AppState>,
) -> WalletResult<String> {
    let config = current_config(&state)?;
    let wallet = config
        .wallet
        .as_deref()
        .ok_or_else(|| WalletError::Message("No wallet is configured.".to_string()))?;
    let parsed_amount = parse_amount(&amount)?;
    let recipient = validate_address(&address)?;
    rpc_call(
        &state,
        &config,
        Some(wallet),
        "sendtoaddress",
        json!([recipient, parsed_amount]),
    )
    .await?
    .as_str()
    .map(ToString::to_string)
    .ok_or_else(|| WalletError::Message("Node returned an invalid transaction id.".to_string()))
}

#[tauri::command]
async fn send_shielded(
    address: String,
    amount: String,
    comment: String,
    state: State<'_, AppState>,
) -> WalletResult<String> {
    let config = current_config(&state)?;
    let wallet = config
        .wallet
        .as_deref()
        .ok_or_else(|| WalletError::Message("No wallet is configured.".to_string()))?;
    let parsed_amount = parse_amount(&amount)?;
    let recipient = validate_address(&address)?;
    let comment = comment.trim();
    ensure_len("Shielded comment", comment, 0, MAX_COMMENT_LEN)?;
    let result = rpc_call(
        &state,
        &config,
        Some(wallet),
        "z_sendtoaddress",
        json!([recipient, parsed_amount, comment]),
    )
    .await?;

    shielded_txid_from_result(&result)
}

fn shielded_txid_from_result(result: &Value) -> WalletResult<String> {
    if let Some(txid) = result.as_str() {
        return Ok(txid.to_string());
    }
    if let Some(txid) = result.get("txid").and_then(Value::as_str) {
        return Ok(txid.to_string());
    }
    Err(WalletError::Message(
        "Node returned an invalid shielded transaction response.".to_string(),
    ))
}

#[tauri::command]
async fn consolidate_shielded_notes(
    amount: String,
    comment: String,
    state: State<'_, AppState>,
) -> WalletResult<String> {
    let config = current_config(&state)?;
    let wallet = config
        .wallet
        .as_deref()
        .ok_or_else(|| WalletError::Message("No wallet is configured.".to_string()))?;
    let parsed_amount = parse_amount(&amount)?;
    let comment = comment.trim();
    ensure_len("Shielded comment", comment, 0, MAX_COMMENT_LEN)?;
    let address = rpc_call(&state, &config, Some(wallet), "z_getnewaddress", json!([]))
        .await?
        .as_str()
        .map(ToString::to_string)
        .ok_or_else(|| {
            WalletError::Message("Node returned an invalid shielded address response.".to_string())
        })?;
    let result = rpc_call(
        &state,
        &config,
        Some(wallet),
        "z_sendtoaddress",
        json!([address, parsed_amount, comment]),
    )
    .await?;

    shielded_txid_from_result(&result)
}

#[tauri::command]
async fn view_shielded_transaction(
    txid: String,
    include_sensitive: bool,
    state: State<'_, AppState>,
) -> WalletResult<Value> {
    let config = current_config(&state)?;
    let wallet = config
        .wallet
        .as_deref()
        .ok_or_else(|| WalletError::Message("No wallet is configured.".to_string()))?;
    let txid = validate_txid(&txid)?;
    rpc_call(
        &state,
        &config,
        Some(wallet),
        "z_viewtransaction",
        json!([txid, include_sensitive]),
    )
    .await
}

#[tauri::command]
async fn backup_wallet_bundle(
    destination: String,
    wallet_passphrase: String,
    archive_path: String,
    archive_passphrase: String,
    state: State<'_, AppState>,
) -> WalletResult<Value> {
    let config = current_config(&state)?;
    let wallet = config
        .wallet
        .as_deref()
        .ok_or_else(|| WalletError::Message("No wallet is configured.".to_string()))?;

    if !archive_path.trim().is_empty() {
        let archive_path = validate_path_input("Archive path", &archive_path, true)?;
        validate_new_wallet_passphrase(&archive_passphrase)?;
        ensure_len(
            "Wallet passphrase",
            &wallet_passphrase,
            0,
            MAX_WALLET_PASSPHRASE_LEN,
        )?;
        return rpc_call(
            &state,
            &config,
            Some(wallet),
            "backupwalletbundlearchive",
            json!([archive_path, archive_passphrase, wallet_passphrase, false]),
        )
        .await;
    }

    let destination = validate_path_input("Backup directory", &destination, true)?;
    ensure_len(
        "Wallet passphrase",
        &wallet_passphrase,
        0,
        MAX_WALLET_PASSPHRASE_LEN,
    )?;
    rpc_call(
        &state,
        &config,
        Some(wallet),
        "backupwalletbundle",
        json!([destination, wallet_passphrase, false]),
    )
    .await
}

fn parse_amount(amount: &str) -> WalletResult<String> {
    let trimmed = amount.trim();
    ensure_len("Amount", trimmed, 1, MAX_AMOUNT_LEN)?;

    if trimmed.starts_with('+')
        || trimmed.starts_with('-')
        || trimmed
            .chars()
            .any(|character| matches!(character, 'e' | 'E'))
    {
        return Err(WalletError::Message(
            "Amount must be a plain positive decimal number.".to_string(),
        ));
    }

    let mut parts = trimmed.split('.');
    let whole_raw = parts.next().unwrap_or_default();
    let fractional = parts.next();
    if parts.next().is_some() || whole_raw.is_empty() {
        return Err(WalletError::Message(
            "Amount must be a valid decimal number.".to_string(),
        ));
    }

    if !whole_raw
        .chars()
        .all(|character| character.is_ascii_digit())
    {
        return Err(WalletError::Message(
            "Amount must be a plain positive decimal number.".to_string(),
        ));
    }

    let fractional = fractional.unwrap_or_default();
    if trimmed.ends_with('.')
        || !fractional
            .chars()
            .all(|character| character.is_ascii_digit())
    {
        return Err(WalletError::Message(
            "Amount must be a valid decimal number.".to_string(),
        ));
    }
    if fractional.len() > 8 {
        return Err(WalletError::Message(
            "Amount must use no more than 8 decimal places.".to_string(),
        ));
    }

    let whole = whole_raw.trim_start_matches('0');
    let whole = if whole.is_empty() { "0" } else { whole };
    if whole.len() > MAX_BTX_AMOUNT_WHOLE.len()
        || (whole.len() == MAX_BTX_AMOUNT_WHOLE.len() && whole > MAX_BTX_AMOUNT_WHOLE)
    {
        return Err(WalletError::Message(
            "Amount must be within the valid BTX supply range.".to_string(),
        ));
    }
    if whole == MAX_BTX_AMOUNT_WHOLE && fractional.chars().any(|character| character != '0') {
        return Err(WalletError::Message(
            "Amount must be within the valid BTX supply range.".to_string(),
        ));
    }
    if whole == "0" && fractional.chars().all(|character| character == '0') {
        return Err(WalletError::Message(
            "Amount must be greater than zero.".to_string(),
        ));
    }

    if fractional.is_empty() {
        Ok(whole.to_string())
    } else {
        Ok(format!("{whole}.{fractional}"))
    }
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            configure_connection,
            get_overview,
            create_wallet,
            restore_wallet,
            unlock_wallet,
            lock_wallet,
            new_address,
            send_transparent,
            send_shielded,
            consolidate_shielded_notes,
            view_shielded_transaction,
            backup_wallet_bundle
        ])
        .run(tauri::generate_context!())
        .expect("error while running BTX Wallet Light");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(url: &str, allow_remote: bool) -> RpcConfig {
        RpcConfig {
            url: url.to_string(),
            username: None,
            password: None,
            cookie_path: None,
            wallet: None,
            allow_remote,
        }
    }

    #[test]
    fn local_http_rpc_is_allowed() {
        assert!(validate_config(&config("http://127.0.0.1:18443", false)).is_ok());
        assert!(validate_config(&config("http://localhost:18443", false)).is_ok());
    }

    #[test]
    fn remote_rpc_requires_https_and_explicit_opt_in() {
        assert!(validate_config(&config("https://node.example.com:18443", false)).is_err());
        assert!(validate_config(&config("http://node.example.com:18443", true)).is_err());
        assert!(validate_config(&config("https://node.example.com:18443", true)).is_ok());
    }

    #[test]
    fn wallet_url_segment_is_percent_encoded() {
        assert_eq!(wallet_path_segment("main wallet"), "main%20wallet");
        assert_eq!(wallet_path_segment("main_wallet-1"), "main_wallet-1");
    }

    #[test]
    fn amount_parser_rejects_invalid_values() {
        assert_eq!(parse_amount("1.25").unwrap(), "1.25");
        assert_eq!(parse_amount("0.00000001").unwrap(), "0.00000001");
        assert_eq!(parse_amount("0001.2300").unwrap(), "1.2300");
        assert_eq!(
            parse_amount("21000000.00000000").unwrap(),
            "21000000.00000000"
        );
        assert!(parse_amount("0").is_err());
        assert!(parse_amount("0.00000000").is_err());
        assert!(parse_amount(".1").is_err());
        assert!(parse_amount("1.").is_err());
        assert!(parse_amount("+1").is_err());
        assert!(parse_amount("-1").is_err());
        assert!(parse_amount("1e-8").is_err());
        assert!(parse_amount("not-a-number").is_err());
        assert!(parse_amount("inf").is_err());
        assert!(parse_amount("1.123456789").is_err());
        assert!(parse_amount("21000000.00000001").is_err());
        assert!(parse_amount("21000001").is_err());
    }

    #[test]
    fn rpc_url_rejects_embedded_credentials_and_url_suffixes() {
        assert!(validate_config(&config("http://user:pass@127.0.0.1:18443", false)).is_err());
        assert!(validate_config(&config("http://127.0.0.1:18443/?x=1", false)).is_err());
        assert!(validate_config(&config("http://127.0.0.1:18443/#fragment", false)).is_err());
    }

    #[test]
    fn wallet_names_are_not_paths() {
        assert!(validate_wallet_name("main").is_ok());
        assert!(validate_wallet_name("../main").is_err());
        assert!(validate_wallet_name("nested/main").is_err());
        assert!(validate_wallet_name("nested\\main").is_err());
    }

    #[test]
    fn ipc_input_lengths_are_bounded() {
        assert!(validate_address(&"b".repeat(MAX_ADDRESS_LEN)).is_ok());
        assert!(validate_address(&"b".repeat(MAX_ADDRESS_LEN + 1)).is_err());
        assert!(ensure_len(
            "Shielded comment",
            &"c".repeat(MAX_COMMENT_LEN),
            0,
            MAX_COMMENT_LEN
        )
        .is_ok());
        assert!(ensure_len(
            "Shielded comment",
            &"c".repeat(MAX_COMMENT_LEN + 1),
            0,
            MAX_COMMENT_LEN
        )
        .is_err());
    }

    #[test]
    fn txids_must_be_hex() {
        assert!(validate_txid(&"a".repeat(TXID_HEX_LEN)).is_ok());
        assert!(validate_txid("aabbcc").is_err());
        assert!(validate_txid("not-a-txid").is_err());
    }

    #[test]
    fn existing_wallet_passphrase_can_be_legacy_short_value() {
        assert!(validate_new_wallet_passphrase("short").is_err());
        assert!(validate_existing_wallet_passphrase("short").is_ok());
        assert!(validate_existing_wallet_passphrase("").is_err());
    }

    #[test]
    fn shielded_note_summary_detects_fragmentation() {
        let notes = json!([
            {"amount": "0.001", "spendable": true, "confirmations": 10},
            {"amount": "0.002", "spendable": true, "confirmations": 10},
            {"amount": "1.5", "spendable": true, "confirmations": 10}
        ]);
        let summary = summarize_shielded_notes(&notes);
        assert!(summary.available);
        assert_eq!(summary.note_count, 3);
        assert_eq!(summary.spendable_count, 3);
        assert_eq!(summary.small_note_count, 2);
        assert_eq!(summary.largest_note, 1.5);
    }

    #[test]
    fn rpc_errors_get_actionable_context() {
        let message = explain_rpc_error("Insufficient funds");
        assert!(message.contains("What to try"));
        assert!(message.contains("consolidation"));
    }

    #[test]
    fn shielded_limit_errors_suggest_consolidating_or_splitting() {
        let message = explain_rpc_error("transaction too large");
        assert!(message.contains("What to try"));
        assert!(message.contains("consolidating"));
        assert!(message.contains("split"));
    }

    #[test]
    fn shielded_broadcast_errors_suggest_sync_and_retry() {
        let message = explain_rpc_error("mempool rejected transaction");
        assert!(message.contains("What to try"));
        assert!(message.contains("not accepted for broadcast"));
    }

    #[test]
    fn transport_errors_get_actionable_context() {
        let message = explain_transport_error("connection refused");
        assert!(message.contains("What to try"));
        assert!(message.contains("Start btxd"));
    }
}
