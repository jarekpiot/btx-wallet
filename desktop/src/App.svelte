<script lang="ts">
  import { onMount } from "svelte";
  import {
    backupBundle,
    consolidateShielded,
    configureConnection,
    createWallet,
    getOverview,
    lockWallet,
    newAddress,
    restoreWallet,
    sendShielded,
    sendTransparent,
    unlockWallet,
    viewShielded
  } from "./lib/api";
  import { formatBtx, isLoopbackRpc } from "./lib/format";
  import {
    connectionModeLabel,
    deleteNodeProfile,
    loadSavedNodes,
    LOCAL_NODE_PROFILE,
    saveNodeProfile,
    syncStatusLabel
  } from "./lib/nodes";
  import {
    buildShieldedSendGuidance,
    consolidationCopy,
    shieldedComplexityLabel
  } from "./lib/shielded";
  import type { Overview, RpcConfig, WalletMode } from "./lib/types";
  import ModeSwitch from "./components/ModeSwitch.svelte";
  import TransactionList from "./components/TransactionList.svelte";

  let overview = $state<Overview>({ connected: false, transactions: [] });
  let selected = $state<"overview" | "send" | "receive" | "history" | "backup" | "settings">("overview");
  let walletMode = $state<WalletMode>("shielded");
  let busy = $state(false);
  let error = $state("");
  let notice = $state("");
  let savedNodes = $state<Array<{ id: string; label: string; url: string; wallet: string; allowRemote: boolean }>>([]);
  let nodeLabel = $state("Local BTX node");

  let rpcUrl = $state("http://127.0.0.1:18443");
  let rpcUser = $state("");
  let rpcPassword = $state("");
  let cookiePath = $state("");
  let walletName = $state("main");
  let allowRemote = $state(false);

  let newWalletName = $state("main");
  let newWalletPassphrase = $state("");
  let restoreName = $state("restored");
  let restorePath = $state("");
  let unlockPassphrase = $state("");
  let unlockSeconds = $state(300);

  let sendAddress = $state("");
  let sendAmount = $state("");
  let sendComment = $state("");
  let consolidationAmount = $state("");
  let consolidationComment = $state("Consolidate shielded notes");
  let receiveAddress = $state("");

  let backupDir = $state("");
  let walletPassphrase = $state("");
  let archivePath = $state("");
  let archivePassphrase = $state("");
  let disclosureTxid = $state("");
  let includeSensitive = $state(false);
  let disclosureResult = $state("");

  const connected = $derived(overview.connected);
  const walletReady = $derived(Boolean(overview.wallet?.walletname));
  const locked = $derived(Boolean(overview.wallet?.encrypted) && !overview.wallet?.unlocked_until);
  const remoteWarning = $derived(!isLoopbackRpc(rpcUrl));
  const transparentBalance = $derived(overview.balances?.transparent ?? 0);
  const shieldedBalance = $derived(overview.balances?.shielded ?? 0);
  const totalBalance = $derived(overview.balances?.total ?? transparentBalance + shieldedBalance);
  const shieldedNoteSummary = $derived(overview.shieldedNoteSummary);
  const shieldedSendGuidance = $derived(
    walletReady ? buildShieldedSendGuidance(sendAmount, shieldedBalance, shieldedNoteSummary) : []
  );
  const shieldedNoteStatus = $derived(shieldedComplexityLabel(shieldedNoteSummary, walletReady));
  const shieldedConsolidationCopy = $derived(consolidationCopy(shieldedNoteSummary));
  const connectionMode = $derived(connectionModeLabel(rpcUrl, allowRemote));
  const syncStatus = $derived(syncStatusLabel(overview.node));

  function clearMessages() {
    error = "";
    notice = "";
  }

  async function run<T>(operation: () => Promise<T>, success?: string): Promise<T | undefined> {
    clearMessages();
    busy = true;
    try {
      const result = await operation();
      if (success) notice = success;
      return result;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      return undefined;
    } finally {
      busy = false;
    }
  }

  async function refresh() {
    const result = await run(() => getOverview());
    if (result) overview = result;
  }

  async function connect() {
    const config: RpcConfig = {
      url: rpcUrl.trim(),
      username: rpcUser.trim() || undefined,
      password: rpcPassword || undefined,
      cookiePath: cookiePath.trim() || undefined,
      wallet: walletName.trim() || undefined,
      allowRemote
    };
    const result = await run(() => configureConnection(config), "Connected to BTX node.");
    if (result) {
      overview = result;
      rpcPassword = "";
    }
  }

  function applyNodeProfile(profile: { label: string; url: string; wallet: string; allowRemote: boolean }) {
    nodeLabel = profile.label;
    rpcUrl = profile.url;
    walletName = profile.wallet || "main";
    allowRemote = profile.allowRemote;
    rpcUser = "";
    rpcPassword = "";
    cookiePath = "";
    selected = "settings";
    notice = "Node profile loaded. Enter RPC credentials or a local cookie path, then connect.";
    error = "";
  }

  function useLocalNode() {
    applyNodeProfile(LOCAL_NODE_PROFILE);
  }

  function handleSaveNodeProfile() {
    savedNodes = saveNodeProfile({
      id: `${rpcUrl.trim()}|${walletName.trim()}`,
      label: nodeLabel.trim() || connectionMode,
      url: rpcUrl.trim(),
      wallet: walletName.trim() || "main",
      allowRemote
    }, savedNodes);
    notice = "Node profile saved without RPC username, password, or cookie data.";
    error = "";
  }

  function handleDeleteNodeProfile(id: string) {
    savedNodes = deleteNodeProfile(id, savedNodes);
    notice = "Node profile removed.";
    error = "";
  }

  async function handleCreateWallet() {
    const result = await run(
      () => createWallet(newWalletName.trim(), newWalletPassphrase),
      "Encrypted descriptor wallet created."
    );
    if (result) {
      overview = result;
      walletName = newWalletName.trim();
      newWalletPassphrase = "";
    }
  }

  async function handleRestoreWallet() {
    const result = await run(
      () => restoreWallet(restoreName.trim(), restorePath.trim()),
      "Wallet restored from official BTX backup."
    );
    if (result) {
      overview = result;
      walletName = restoreName.trim();
      restorePath = "";
    }
  }

  async function handleUnlock() {
    const result = await run(
      () => unlockWallet(unlockPassphrase, Number(unlockSeconds)),
      "Wallet unlocked temporarily."
    );
    if (result) {
      overview = result;
      unlockPassphrase = "";
    }
  }

  async function handleLock() {
    const result = await run(() => lockWallet(), "Wallet locked.");
    if (result) overview = result;
  }

  async function handleNewAddress() {
    const result = await run(() => newAddress(walletMode), `${walletMode} address created.`);
    if (result) receiveAddress = result;
  }

  async function handleSend() {
    const operation =
      walletMode === "shielded"
        ? () => sendShielded(sendAddress.trim(), sendAmount.trim(), sendComment.trim())
        : () => sendTransparent(sendAddress.trim(), sendAmount.trim());
    const result = await run(operation, "Transaction submitted.");
    if (result) {
      notice = `${walletMode === "shielded" ? "Shielded" : "Transparent"} txid: ${result}`;
      sendAddress = "";
      sendAmount = "";
      sendComment = "";
      await refresh();
    }
  }

  async function handleConsolidateShielded() {
    const result = await run(
      () => consolidateShielded(consolidationAmount.trim(), consolidationComment.trim()),
      "Shielded note consolidation submitted."
    );
    if (result) {
      notice = `Consolidation txid: ${result}`;
      consolidationAmount = "";
      await refresh();
    }
  }

  async function handleDisclosure() {
    const result = await run(() => viewShielded(disclosureTxid.trim(), includeSensitive));
    if (result) disclosureResult = JSON.stringify(result, null, 2);
  }

  async function handleBackup() {
    const result = await run(
      () => backupBundle(backupDir.trim(), walletPassphrase, archivePath.trim(), archivePassphrase),
      "Backup export completed."
    );
    if (result) {
      notice = `Backup complete: ${JSON.stringify(result, null, 2)}`;
      walletPassphrase = "";
      archivePassphrase = "";
    }
  }

  onMount(() => {
    savedNodes = loadSavedNodes();
  });

  $effect(() => {
    refresh();
  });

</script>

<main>
  <aside class="sidebar">
    <div class="brand">
      <div class="mark">B</div>
      <div>
        <strong>BTX Wallet</strong>
        <span>Light v0.2.0</span>
      </div>
    </div>

    <nav aria-label="Primary">
      {#each ["overview", "send", "receive", "history", "backup", "settings"] as item}
        <button class:active={selected === item} type="button" onclick={() => (selected = item)}>
          <span>{item}</span>
        </button>
      {/each}
    </nav>

    <div class="connection-pill" class:online={connected}>
      <span></span>
      {connected ? "Connected" : "Disconnected"}
    </div>
    <div class="node-summary">
      <strong>{connectionMode}</strong>
      <span>{syncStatus}</span>
    </div>
  </aside>

  <section class="workspace">
    <header class="topbar">
      <div>
        <p>{overview.node?.chain ?? "BTX"}</p>
        <h1>{selected}</h1>
      </div>
      <div class="actions">
        {#if walletReady}
          <button type="button" class="ghost" onclick={handleLock} disabled={busy || locked}>Lock</button>
          <button type="button" class="primary" onclick={refresh} disabled={busy}>Refresh</button>
        {:else}
          <button type="button" class="primary" onclick={() => (selected = "settings")}>Connect</button>
        {/if}
      </div>
    </header>

    {#if error}
      <div class="alert danger">{error}</div>
    {/if}
    {#if notice}
      <div class="alert success">{notice}</div>
    {/if}

    {#if selected === "overview"}
      {#if !connected}
        <section class="onboarding">
          <div>
            <span>Welcome</span>
            <h2>Connect to BTX and start with shielded privacy</h2>
            <p>
              BTX Wallet Light opens quickly and uses BTX core over JSON-RPC for wallet creation,
              signing, shielded sends, and note handling.
            </p>
            <div class="onboarding-actions">
              <button type="button" class="primary" onclick={useLocalNode}>Use local node</button>
              <button type="button" class="ghost" onclick={() => (selected = "settings")}>Open settings</button>
            </div>
          </div>
          <div class="setup-steps">
            <div>
              <strong>1. Run or choose a node</strong>
              <span>Local btxd is best. Trusted HTTPS remote nodes are supported.</span>
            </div>
            <div>
              <strong>2. Connect safely</strong>
              <span>Saved profiles keep only public node details, never RPC passwords.</span>
            </div>
            <div>
              <strong>3. Create a wallet</strong>
              <span>Wallet encryption, keys, and shielded transactions stay in official BTX core.</span>
            </div>
          </div>
        </section>
      {/if}

      <section class="balance-grid">
        <div class="balance-panel total">
          <span>Total</span>
          <strong>{formatBtx(totalBalance)}</strong>
          <small>BTX</small>
        </div>
        <div class="balance-panel shielded">
          <span>Shielded</span>
          <strong>{formatBtx(shieldedBalance)}</strong>
          <small>SMILE v2 private balance</small>
        </div>
        <div class="balance-panel transparent">
          <span>Transparent</span>
          <strong>{formatBtx(transparentBalance)}</strong>
          <small>Public UTXO balance</small>
        </div>
      </section>

      <section class="status-grid">
        <div>
          <span>Wallet</span>
          <strong>{overview.wallet?.walletname ?? "Not loaded"}</strong>
        </div>
        <div>
          <span>Lock state</span>
          <strong>{locked ? "Locked" : walletReady ? "Ready" : "Unavailable"}</strong>
        </div>
        <div>
          <span>Node height</span>
          <strong>{syncStatus}</strong>
        </div>
        <div>
          <span>Pruned</span>
          <strong>{overview.node?.pruned ? "Yes" : "No"}</strong>
        </div>
        <div>
          <span>Shielded notes</span>
          <strong>{shieldedNoteStatus}</strong>
        </div>
      </section>

      <section class="panel">
        <div class="panel-head">
          <h2>Recent Activity</h2>
          <button type="button" class="ghost" onclick={() => (selected = "history")}>View all</button>
        </div>
        <TransactionList transactions={overview.transactions.slice(0, 6)} />
      </section>
    {/if}

    {#if selected === "send"}
      <section class="panel split">
        <div>
          <h2>Send BTX</h2>
          <ModeSwitch bind:value={walletMode} />
          <label>
            Recipient
            <textarea bind:value={sendAddress} rows="4" spellcheck="false"></textarea>
          </label>
          <label>
            Amount
            <input bind:value={sendAmount} inputmode="decimal" placeholder="0.00000000" />
          </label>
          {#if walletMode === "shielded"}
            <label>
              Local note
              <input bind:value={sendComment} maxlength="80" placeholder="Optional local-only comment" />
            </label>
          {/if}
          <button type="button" class="primary wide" onclick={handleSend} disabled={busy || !walletReady || locked}>
            Send {walletMode === "shielded" ? "shielded" : "transparent"}
          </button>
        </div>
        <div class="context-pane">
          <h3>{walletMode === "shielded" ? "Shielded send" : "Transparent send"}</h3>
          <p>
            {walletMode === "shielded"
              ? "SMILE v2 sends are built and signed by the official BTX core. The app only asks the node to prepare and broadcast."
              : "Transparent sends are public on-chain and are useful for exchange or bridge-style flows."}
          </p>
          <div class="mini-balance">
            <span>Available</span>
            <strong>{formatBtx(walletMode === "shielded" ? shieldedBalance : transparentBalance)} BTX</strong>
          </div>
          {#if walletMode === "shielded"}
            <div class="note-health" class:warning={shieldedNoteSummary?.complexity === "medium"} class:danger={shieldedNoteSummary?.complexity === "high"}>
              <div>
                <span>Shielded notes</span>
                <strong>
                  {shieldedNoteSummary?.available
                    ? `${shieldedNoteSummary.spendableCount} spendable`
                    : "Not available"}
                </strong>
              </div>
              {#if shieldedNoteSummary?.available}
                <div class="note-stats">
                  <span>Largest note: {formatBtx(shieldedNoteSummary.largestNote)} BTX</span>
                  <span>Small notes: {shieldedNoteSummary.smallNoteCount}</span>
                </div>
              {/if}
            </div>
            {#if shieldedSendGuidance.length}
              <div class="guidance">
                {#each shieldedSendGuidance as item}
                  <p>{item}</p>
                {/each}
              </div>
            {/if}
            <div class="consolidation">
              <h3>Consolidate notes</h3>
              <p>{shieldedConsolidationCopy}</p>
              <label>
                Amount to consolidate
                <input bind:value={consolidationAmount} inputmode="decimal" placeholder="0.00000000" />
              </label>
              <label>
                Local note
                <input bind:value={consolidationComment} maxlength="80" />
              </label>
              <button type="button" class="ghost wide" onclick={handleConsolidateShielded} disabled={busy || !walletReady || locked || !consolidationAmount.trim()}>
                Consolidate shielded notes
              </button>
            </div>
          {/if}
        </div>
      </section>
    {/if}

    {#if selected === "receive"}
      <section class="panel split">
        <div>
          <h2>Receive</h2>
          <ModeSwitch bind:value={walletMode} />
          <button type="button" class="primary" onclick={handleNewAddress} disabled={busy || !walletReady}>
            New {walletMode} address
          </button>
          <label>
            Address
            <textarea readonly rows="6" value={receiveAddress}></textarea>
          </label>
        </div>
        <div class="context-pane">
          <h3>Address safety</h3>
          <p>Use shielded addresses for private receives. For large transfers, send a small test amount first.</p>
        </div>
      </section>
    {/if}

    {#if selected === "history"}
      <section class="panel">
        <div class="panel-head">
          <h2>Transaction History</h2>
          <button type="button" class="ghost" onclick={refresh} disabled={busy}>Refresh</button>
        </div>
        <TransactionList transactions={overview.transactions} />
      </section>
      <section class="panel">
        <h2>Selective Disclosure</h2>
        <div class="form-row">
          <label>
            Shielded txid
            <input bind:value={disclosureTxid} spellcheck="false" />
          </label>
          <label class="checkbox">
            <input type="checkbox" bind:checked={includeSensitive} />
            Include sensitive local details
          </label>
        </div>
        <button type="button" class="primary" onclick={handleDisclosure} disabled={busy || !walletReady}>
          View shielded transaction
        </button>
        {#if disclosureResult}
          <pre>{disclosureResult}</pre>
        {/if}
      </section>
    {/if}

    {#if selected === "backup"}
      <section class="panel split">
        <div>
          <h2>Backup & Restore</h2>
          <label>
            Bundle directory
            <input bind:value={backupDir} placeholder="C:\\secure\\btx-main-bundle" />
          </label>
          <label>
            Optional encrypted archive path
            <input bind:value={archivePath} placeholder="C:\\secure\\btx-main.btxbundle" />
          </label>
          <label>
            Wallet passphrase
            <input bind:value={walletPassphrase} type="password" autocomplete="current-password" />
          </label>
          <label>
            Archive passphrase
            <input bind:value={archivePassphrase} type="password" autocomplete="new-password" />
          </label>
          <button type="button" class="primary wide" onclick={handleBackup} disabled={busy || !walletReady}>
            Create secure backup
          </button>
        </div>
        <div>
          <h2>Restore Wallet</h2>
          <label>
            Restored wallet name
            <input bind:value={restoreName} />
          </label>
          <label>
            Backup file
            <input bind:value={restorePath} placeholder="Path to official BTX backup file" />
          </label>
          <button type="button" class="ghost wide" onclick={handleRestoreWallet} disabled={busy || !connected}>
            Restore
          </button>
        </div>
      </section>
    {/if}

    {#if selected === "settings"}
      <section class="panel split">
        <div>
          <h2>Node Connection</h2>
          <div class="connection-card">
            <div>
              <span>{connectionMode}</span>
              <strong>{syncStatus}</strong>
            </div>
            <button type="button" class="ghost" onclick={useLocalNode}>Local preset</button>
          </div>
          {#if savedNodes.length}
            <div class="saved-nodes">
              <span>Saved nodes</span>
              {#each savedNodes as profile}
                <div class="saved-node">
                  <button type="button" onclick={() => applyNodeProfile(profile)}>
                    <strong>{profile.label}</strong>
                    <span>{profile.url}</span>
                  </button>
                  <button type="button" class="ghost compact" onclick={() => handleDeleteNodeProfile(profile.id)}>
                    Remove
                  </button>
                </div>
              {/each}
            </div>
          {/if}
          <label>
            Profile name
            <input bind:value={nodeLabel} placeholder="Local BTX node" />
          </label>
          <label>
            RPC URL
            <input bind:value={rpcUrl} spellcheck="false" />
          </label>
          {#if remoteWarning}
            <label class="checkbox">
              <input type="checkbox" bind:checked={allowRemote} />
              Allow non-local RPC endpoint
            </label>
          {/if}
          <label>
            RPC username
            <input bind:value={rpcUser} autocomplete="username" />
          </label>
          <label>
            RPC password
            <input bind:value={rpcPassword} type="password" autocomplete="current-password" />
          </label>
          <label>
            Cookie path
            <input bind:value={cookiePath} placeholder="Optional .cookie path for local node" />
          </label>
          <label>
            Wallet name
            <input bind:value={walletName} />
          </label>
          <button type="button" class="primary wide" onclick={connect} disabled={busy || (remoteWarning && !allowRemote)}>
            {busy ? "Connecting..." : "Connect"}
          </button>
          <button type="button" class="ghost wide" onclick={handleSaveNodeProfile} disabled={!rpcUrl.trim()}>
            Save node profile
          </button>
          <p class="field-note">
            Profiles save node URL, wallet name, and remote approval only. RPC credentials stay out of saved data.
          </p>
        </div>
        <div>
          <h2>Wallet Access</h2>
          <label>
            New wallet name
            <input bind:value={newWalletName} />
          </label>
          <label>
            Encryption passphrase
            <input bind:value={newWalletPassphrase} type="password" autocomplete="new-password" />
          </label>
          <button type="button" class="ghost wide" onclick={handleCreateWallet} disabled={busy || !connected}>
            Create encrypted descriptor wallet
          </button>
          <div class="divider"></div>
          <label>
            Unlock passphrase
            <input bind:value={unlockPassphrase} type="password" autocomplete="current-password" />
          </label>
          <label>
            Unlock seconds
            <input bind:value={unlockSeconds} type="number" min="30" max="1800" />
          </label>
          <button type="button" class="primary wide" onclick={handleUnlock} disabled={busy || !walletReady}>
            Unlock temporarily
          </button>
        </div>
      </section>
    {/if}
  </section>
</main>
