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
const MAX_TXID_LEN: usize = 128;
const MAX_BTX_AMOUNT: f64 = 21_000_000.0;

#[derive(Debug, Error)]
enum WalletError {
    #[error("{0}")]
    Message(String),
    #[error("RPC request failed: {0}")]
    Rpc(#[from] reqwest::Error),
    #[error("RPC auth cookie could not be read: {0}")]
    Io(#[from] std::io::Error),
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
    ensure_len("Transaction id", trimmed, 1, MAX_TXID_LEN)?;
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

fn validate_wallet_passphrase(passphrase: &str) -> WalletResult<()> {
    ensure_len(
        "Wallet passphrase",
        passphrase,
        12,
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
    if let Some(message) = error.get("message").and_then(Value::as_str) {
        message.to_string()
    } else {
        error.to_string()
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
    validate_wallet_passphrase(&passphrase)?;
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
    validate_wallet_passphrase(&passphrase)?;
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
        validate_wallet_passphrase(&archive_passphrase)?;
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

fn parse_amount(amount: &str) -> WalletResult<f64> {
    let trimmed = amount.trim();
    if trimmed.len() > 32 {
        return Err(WalletError::Message("Amount is too long.".to_string()));
    }
    if let Some((_, fractional)) = trimmed.split_once('.') {
        if fractional.len() > 8 {
            return Err(WalletError::Message(
                "Amount must use no more than 8 decimal places.".to_string(),
            ));
        }
    }
    let parsed = amount
        .trim()
        .parse::<f64>()
        .map_err(|_| WalletError::Message("Amount is not a valid number.".to_string()))?;
    if !parsed.is_finite() || parsed <= 0.0 || parsed > MAX_BTX_AMOUNT {
        return Err(WalletError::Message(
            "Amount must be finite and within the valid BTX supply range.".to_string(),
        ));
    }
    Ok(parsed)
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
        assert!(parse_amount("1.25").is_ok());
        assert!(parse_amount("0.00000001").is_ok());
        assert!(parse_amount("0").is_err());
        assert!(parse_amount("-1").is_err());
        assert!(parse_amount("not-a-number").is_err());
        assert!(parse_amount("inf").is_err());
        assert!(parse_amount("1.123456789").is_err());
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
        assert!(validate_txid("aabbcc").is_ok());
        assert!(validate_txid("not-a-txid").is_err());
    }
}
