export type ConnectionMode = "local" | "remote";
export type WalletMode = "transparent" | "shielded";

export interface RpcConfig {
  url: string;
  username?: string;
  password?: string;
  cookiePath?: string;
  wallet?: string;
  allowRemote: boolean;
}

export interface NodeInfo {
  chain?: string;
  blocks?: number;
  headers?: number;
  initialblockdownload?: boolean;
  pruned?: boolean;
  verificationprogress?: number;
  warnings?: string[];
}

export interface WalletInfo {
  walletname?: string;
  encrypted?: boolean;
  unlocked_until?: number;
  descriptors?: boolean;
  private_keys_enabled?: boolean;
  txcount?: number;
  balance?: number;
}

export interface BalanceSet {
  transparent: number;
  shielded: number;
  total: number;
  immature?: number;
}

export interface TransactionItem {
  txid?: string;
  category?: string;
  address?: string;
  amount?: number;
  confirmations?: number;
  time?: number;
  family?: string;
}

export interface Overview {
  connected: boolean;
  node?: NodeInfo;
  wallet?: WalletInfo;
  balances?: BalanceSet;
  transactions: TransactionItem[];
  configuredWallet?: string;
}

export interface RpcResult {
  ok: boolean;
  message: string;
}
