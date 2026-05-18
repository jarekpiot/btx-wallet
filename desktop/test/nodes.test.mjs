import assert from "node:assert/strict";
import test from "node:test";

import {
  connectionModeLabel,
  deleteNodeProfile,
  loadSavedNodes,
  saveNodeProfile,
  sanitizeNodeProfile,
  syncStatusLabel
} from "../src/lib/nodes.js";

function memoryStorage(initial = {}) {
  const data = new Map(Object.entries(initial));
  return {
    getItem: (key) => data.get(key),
    setItem: (key, value) => data.set(key, value)
  };
}

test("saved node profiles do not preserve credentials", () => {
  const profile = sanitizeNodeProfile({
    label: "Remote",
    url: "https://node.example.com:18443",
    wallet: "main",
    allowRemote: true,
    username: "user",
    password: "secret",
    cookiePath: "C:/Users/me/.btx/.cookie"
  });

  assert.deepEqual(Object.keys(profile).sort(), ["allowRemote", "id", "label", "url", "wallet"]);
});

test("saved node profiles reject credential-bearing URLs", () => {
  assert.equal(
    sanitizeNodeProfile({
      label: "Remote",
      url: "https://user:secret@node.example.com:18443",
      allowRemote: true
    }),
    undefined
  );
});

test("saved node profiles reject secret-like URL extras", () => {
  assert.equal(
    sanitizeNodeProfile({ label: "Remote", url: "https://node.example.com:18443/?token=secret" }),
    undefined
  );
  assert.equal(
    sanitizeNodeProfile({ label: "Remote", url: "https://node.example.com:18443/#token" }),
    undefined
  );
});

test("saved node profiles only allow HTTP and HTTPS URLs", () => {
  assert.equal(sanitizeNodeProfile({ label: "Socket", url: "file:///tmp/btx.sock" }), undefined);
  assert.equal(sanitizeNodeProfile({ label: "Script", url: "javascript:alert(1)" }), undefined);
});

test("saved node profiles persist bounded public connection fields", () => {
  const storage = memoryStorage();
  const profiles = saveNodeProfile(
    { id: "one", label: "Local", url: "http://127.0.0.1:18443", wallet: "main" },
    [],
    storage
  );
  assert.equal(profiles.length, 1);
  assert.equal(loadSavedNodes(storage)[0].label, "Local");
  assert.equal(deleteNodeProfile("one", profiles, storage).length, 0);
});

test("saved node profiles drop legacy unsafe entries from storage", () => {
  const storage = memoryStorage({
    "btx-wallet-light.nodes.v1": JSON.stringify([
      { id: "safe", label: "Local", url: "http://127.0.0.1:18443", wallet: "main" },
      { id: "unsafe", label: "Leaky", url: "https://user:secret@node.example.com:18443" }
    ])
  });

  const profiles = loadSavedNodes(storage);
  assert.equal(profiles.length, 1);
  assert.equal(profiles[0].id, "safe");
});

test("connection labels are clear for local and remote nodes", () => {
  assert.equal(connectionModeLabel("http://127.0.0.1:18443", false), "Local node");
  assert.equal(connectionModeLabel("https://node.example.com:18443", true), "Trusted remote node");
  assert.equal(connectionModeLabel("https://node.example.com:18443", false), "Remote node blocked");
});

test("sync status labels show first-run node state", () => {
  assert.equal(syncStatusLabel(undefined), "Not connected");
  assert.equal(syncStatusLabel({ blocks: 10, headers: 20 }), "Syncing 10 / 20");
  assert.equal(syncStatusLabel({ blocks: 20, headers: 20 }), "Synced at 20");
  assert.equal(syncStatusLabel({ blocks: 1, headers: 20, initialblockdownload: true }), "Initial sync");
});
