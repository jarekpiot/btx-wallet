use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::sync::Mutex;
use std::time::Duration;
use tauri::State;
use thiserror::Error;

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
    let name = wallet_name.trim();
    if name.is_empty() {
        return Err(WalletError::Message("Wallet name is required.".to_string()));
    }
    if passphrase.len() < 12 {
        return Err(WalletError::Message(
            "Use a wallet passphrase of at least 12 characters.".to_string(),
        ));
    }
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
    let name = wallet_name.trim();
    let backup = backup_file.trim();
    if name.is_empty() || backup.is_empty() {
        return Err(WalletError::Message(
            "Wallet name and backup file are required.".to_string(),
        ));
    }
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
    let method = if mode == "shielded" {
        "z_getnewaddress"
    } else {
        "getnewaddress"
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
    rpc_call(
        &state,
        &config,
        Some(wallet),
        "sendtoaddress",
        json!([address.trim(), parsed_amount]),
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
    let result = rpc_call(
        &state,
        &config,
        Some(wallet),
        "z_sendtoaddress",
        json!([address.trim(), parsed_amount, comment.trim()]),
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
    rpc_call(
        &state,
        &config,
        Some(wallet),
        "z_viewtransaction",
        json!([txid.trim(), include_sensitive]),
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
        if archive_passphrase.len() < 12 {
            return Err(WalletError::Message(
                "Archive passphrase must be at least 12 characters.".to_string(),
            ));
        }
        return rpc_call(
            &state,
            &config,
            Some(wallet),
            "backupwalletbundlearchive",
            json!([
                archive_path.trim(),
                archive_passphrase,
                wallet_passphrase,
                false
            ]),
        )
        .await;
    }

    if destination.trim().is_empty() {
        return Err(WalletError::Message(
            "Backup directory or archive path is required.".to_string(),
        ));
    }
    rpc_call(
        &state,
        &config,
        Some(wallet),
        "backupwalletbundle",
        json!([destination.trim(), wallet_passphrase, false]),
    )
    .await
}

fn parse_amount(amount: &str) -> WalletResult<f64> {
    let parsed = amount
        .trim()
        .parse::<f64>()
        .map_err(|_| WalletError::Message("Amount is not a valid number.".to_string()))?;
    if parsed <= 0.0 {
        return Err(WalletError::Message(
            "Amount must be greater than zero.".to_string(),
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
        assert!(parse_amount("0").is_err());
        assert!(parse_amount("-1").is_err());
        assert!(parse_amount("not-a-number").is_err());
    }
}
