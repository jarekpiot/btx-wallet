import assert from "node:assert/strict";
import test from "node:test";

import {
  historyEmptyCopy,
  receiveEmptyCopy,
  sendBlockReason,
  sendButtonLabel
} from "../src/lib/ui.js";

test("send block reasons guide the next action", () => {
  assert.equal(sendBlockReason({ connected: false }), "Connect to a node before sending.");
  assert.equal(sendBlockReason({ connected: true, walletReady: false }), "Create, restore, or load a wallet before sending.");
  assert.equal(sendBlockReason({ connected: true, walletReady: true, locked: true }), "Unlock the wallet temporarily before sending.");
  assert.equal(sendBlockReason({ connected: true, walletReady: true, locked: false, address: "", amount: "1" }), "Enter a recipient address.");
  assert.equal(sendBlockReason({ connected: true, walletReady: true, locked: false, address: "btxs1...", amount: "" }), "Enter an amount.");
});

test("send button labels stay action-oriented", () => {
  assert.equal(sendButtonLabel({ busy: true, mode: "shielded" }), "Submitting...");
  assert.equal(sendButtonLabel({ busy: false, mode: "shielded", blockReason: "Enter an amount." }), "Complete send details");
  assert.equal(sendButtonLabel({ busy: false, mode: "transparent", blockReason: "" }), "Send transparent");
});

test("receive empty copy matches wallet state and mode", () => {
  assert.match(receiveEmptyCopy("shielded", false).body, /create or restore/);
  assert.match(receiveEmptyCopy("shielded", true).title, /shielded address/);
  assert.match(receiveEmptyCopy("transparent", true).body, /public/);
});

test("history empty copy points users toward activity", () => {
  assert.match(historyEmptyCopy(false).body, /Load a wallet/);
  assert.match(historyEmptyCopy(true).body, /Receive BTX/);
});
