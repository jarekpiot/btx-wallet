import assert from "node:assert/strict";
import test from "node:test";

import {
  buildShieldedSendGuidance,
  consolidationCopy,
  shieldedComplexityLabel
} from "../src/lib/shielded.js";

const highFragmentation = {
  available: true,
  noteCount: 72,
  spendableCount: 72,
  immatureCount: 0,
  totalAmount: 12,
  largestNote: 1,
  smallNoteCount: 50,
  complexity: "high",
  guidance: ["Consider consolidating to a fresh shielded address or splitting the payment."]
};

test("shielded guidance warns on high note count and large amount", () => {
  const guidance = buildShieldedSendGuidance("5", 12, highFragmentation);
  assert.match(guidance.join("\n"), /High note count/);
  assert.match(guidance.join("\n"), /largest shielded note/);
  assert.match(guidance.join("\n"), /consolidating/);
});

test("shielded guidance warns before nearly full balance sends", () => {
  const guidance = buildShieldedSendGuidance("11", 12, highFragmentation);
  assert.match(guidance.join("\n"), /too little room for fees/);
});

test("shielded guidance explains unavailable note detail", () => {
  const guidance = buildShieldedSendGuidance("1", 12, undefined);
  assert.equal(guidance.length, 1);
  assert.match(guidance[0], /local synced node/);
});

test("shielded status and consolidation copy stay user-facing", () => {
  assert.equal(shieldedComplexityLabel(highFragmentation, true), "High complexity");
  assert.match(consolidationCopy(highFragmentation), /normal shielded self-send/);
  assert.equal(shieldedComplexityLabel(undefined, false), "Unavailable");
});
