import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const viteConfig = readFileSync(new URL("../vite.config.ts", import.meta.url), "utf8");
const mainTs = readFileSync(new URL("../src/main.ts", import.meta.url), "utf8");

test("vite production build uses relative asset paths for Tauri bundles", () => {
  assert.match(viteConfig, /base:\s*["']\.\/["']/);
});

test("svelte app uses Svelte 5 mount API", () => {
  assert.match(mainTs, /import\s+\{\s*mount\s*\}\s+from\s+["']svelte["']/);
  assert.match(mainTs, /mount\s*\(\s*App\s*,/);
  assert.doesNotMatch(mainTs, /new\s+App\s*\(/);
});
