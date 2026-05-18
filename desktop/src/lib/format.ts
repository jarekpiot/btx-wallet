export function formatBtx(value: number | undefined): string {
  if (typeof value !== "number" || Number.isNaN(value)) {
    return "0.00000000";
  }
  return value.toLocaleString("en-US", {
    minimumFractionDigits: 8,
    maximumFractionDigits: 8
  });
}

export function shortenHash(value: string | undefined): string {
  if (!value) {
    return "pending";
  }
  if (value.length <= 18) {
    return value;
  }
  return `${value.slice(0, 10)}...${value.slice(-8)}`;
}

export function formatTime(epoch: number | undefined): string {
  if (!epoch) {
    return "unconfirmed";
  }
  return new Date(epoch * 1000).toLocaleString();
}

export function isLoopbackRpc(url: string): boolean {
  try {
    const parsed = new URL(url);
    return ["127.0.0.1", "localhost", "::1"].includes(parsed.hostname);
  } catch {
    return false;
  }
}
