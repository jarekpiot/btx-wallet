import assert from "node:assert/strict";
import test from "node:test";

import { friendlyError, successMessage } from "../src/lib/messages.js";

test("friendly errors add operation context and next steps", () => {
  const message = friendlyError("Insufficient funds", "shieldedSend");
  assert.match(message, /Shielded send was not submitted/);
  assert.match(message, /What to try:/);
  assert.match(message, /consolidate or split/);
});

test("friendly errors preserve backend what-to-try guidance", () => {
  const message = friendlyError("transaction too large\n\nWhat to try: split the send.", "shieldedSend");
  assert.match(message, /Shielded send was not submitted/);
  assert.equal((message.match(/What to try:/g) ?? []).length, 1);
});

test("friendly errors handle unreadable node failures", () => {
  const message = friendlyError("[object Object]", "connect");
  assert.match(message, /Could not connect/);
  assert.match(message, /readable error/);
});

test("success messages explain shielded follow-up", () => {
  const message = successMessage("consolidate", "abc123");
  assert.match(message, /Shielded consolidation submitted/);
  assert.match(message, /refresh note health/);
});
