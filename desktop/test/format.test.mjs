import assert from "node:assert/strict";
import test from "node:test";

function formatBtx(value) {
  if (typeof value !== "number" || Number.isNaN(value)) {
    return "0.00000000";
  }
  return value.toLocaleString("en-US", {
    minimumFractionDigits: 8,
    maximumFractionDigits: 8
  });
}

function shortenHash(value) {
  if (!value) return "pending";
  if (value.length <= 18) return value;
  return `${value.slice(0, 10)}...${value.slice(-8)}`;
}

test("formats BTX values with eight decimals", () => {
  assert.equal(formatBtx(12), "12.00000000");
  assert.equal(formatBtx(0.123456789), "0.12345679");
  assert.equal(formatBtx(undefined), "0.00000000");
});

test("shortens hashes without losing ends", () => {
  assert.equal(shortenHash("abcdef"), "abcdef");
  assert.equal(shortenHash("0123456789abcdef0123456789abcdef"), "0123456789...89abcdef");
});
