import assert from "node:assert/strict";
import test from "node:test";

import {
  buildShieldedSendGuidance,
  consolidationCopy,
  consolidationExpectations,
  shieldedComplexityLabel,
  shieldedRiskLabel,
  shieldedRiskLevel,
  suggestedConsolidationAmount
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
  assert.match(guidance.join("\n"), /roughly 5 or more notes/);
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

test("shielded risk labels summarize send attention level", () => {
  assert.equal(shieldedRiskLevel("5", 12, highFragmentation), "high");
  assert.equal(shieldedRiskLabel("high"), "High attention");
  assert.equal(shieldedRiskLabel("low"), "Looks normal");
});

test("consolidation helper suggests a fee-aware amount and expectations", () => {
  assert.equal(suggestedConsolidationAmount(10, highFragmentation), "9.00000000");
  assert.equal(suggestedConsolidationAmount(10, { ...highFragmentation, complexity: "low" }), "");
  assert.match(consolidationExpectations(highFragmentation).join("\n"), /fresh shielded address/);
  assert.match(consolidationExpectations(highFragmentation).join("\n"), /many tiny notes/);
});
