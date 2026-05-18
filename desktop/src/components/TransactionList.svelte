<script lang="ts">
  import { formatBtx, formatTime, shortenHash } from "../lib/format";
  import type { TransactionItem } from "../lib/types";

  export let transactions: TransactionItem[] = [];
</script>

{#if transactions.length === 0}
  <div class="empty">No wallet transactions loaded.</div>
{:else}
  <div class="tx-list">
    {#each transactions as tx}
      <div class="tx-row">
        <div>
          <strong>{tx.category ?? tx.family ?? "transaction"}</strong>
          <span>{shortenHash(tx.txid)}</span>
        </div>
        <div>
          <strong>{formatBtx(tx.amount)} BTX</strong>
          <span>{tx.confirmations ?? 0} conf - {formatTime(tx.time)}</span>
        </div>
      </div>
    {/each}
  </div>
{/if}
