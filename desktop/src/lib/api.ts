import { invoke } from "@tauri-apps/api/core";
import type { Overview, RpcConfig, WalletMode } from "./types";

export function configureConnection(config: RpcConfig): Promise<Overview> {
  return invoke("configure_connection", { config });
}

export function getOverview(): Promise<Overview> {
  return invoke("get_overview");
}

export function createWallet(walletName: string, passphrase: string): Promise<Overview> {
  return invoke("create_wallet", { walletName, passphrase });
}

export function restoreWallet(walletName: string, backupFile: string): Promise<Overview> {
  return invoke("restore_wallet", { walletName, backupFile });
}

export function unlockWallet(passphrase: string, seconds: number): Promise<Overview> {
  return invoke("unlock_wallet", { passphrase, seconds });
}

export function lockWallet(): Promise<Overview> {
  return invoke("lock_wallet");
}

export function newAddress(mode: WalletMode): Promise<string> {
  return invoke("new_address", { mode });
}

export function sendTransparent(address: string, amount: string): Promise<string> {
  return invoke("send_transparent", { address, amount });
}

export function sendShielded(address: string, amount: string, comment: string): Promise<string> {
  return invoke("send_shielded", { address, amount, comment });
}

export function viewShielded(txid: string, includeSensitive: boolean): Promise<unknown> {
  return invoke("view_shielded_transaction", { txid, includeSensitive });
}

export function backupBundle(
  destination: string,
  walletPassphrase: string,
  archivePath: string,
  archivePassphrase: string
): Promise<unknown> {
  return invoke("backup_wallet_bundle", {
    destination,
    walletPassphrase,
    archivePath,
    archivePassphrase
  });
}
