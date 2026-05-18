<script lang="ts">
  import type { WalletMode } from "../lib/types";

  export let value: WalletMode;
</script>

<div class="mode-switch" role="group" aria-label="Wallet mode">
  <button type="button" class:active={value === "shielded"} onclick={() => (value = "shielded")}>
    Shielded
  </button>
  <button type="button" class:active={value === "transparent"} onclick={() => (value = "transparent")}>
    Transparent
  </button>
</div>
