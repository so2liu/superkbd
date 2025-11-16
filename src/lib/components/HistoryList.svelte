<script lang="ts">
  import { onMount } from 'svelte';
  import HistoryItem from './HistoryItem.svelte';
  import { filteredEntries, selectedIndex, pasteAndClose } from '$lib/stores/clipboard';
  import type { ClipboardEntry } from '$lib/stores/clipboard';

  let entries = $derived($filteredEntries);
  let selected = $state(0);

  // Sync with store
  $effect(() => {
    selectedIndex.set(selected);
  });

  // Reset selected index when entries change
  $effect(() => {
    if (entries.length > 0 && selected >= entries.length) {
      selected = 0;
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        selected = Math.min(selected + 1, entries.length - 1);
        break;
      case 'ArrowUp':
        e.preventDefault();
        selected = Math.max(selected - 1, 0);
        break;
      case 'Enter':
        e.preventDefault();
        handleSelect(entries[selected]);
        break;
      case 'Escape':
        e.preventDefault();
        window.__TAURI__?.invoke('hide_window_command');
        break;
    }
  }

  async function handleSelect(entry: ClipboardEntry) {
    const startTime = performance.now();
    console.log('⏱️  [PERF] T+0ms: Enter pressed, handleSelect called');

    if (entry?.text_content) {
      console.log(`⏱️  [PERF] T+${(performance.now() - startTime).toFixed(1)}ms: Calling pasteAndClose`);
      await pasteAndClose(entry.text_content);
      console.log(`⏱️  [PERF] T+${(performance.now() - startTime).toFixed(1)}ms: pasteAndClose returned`);
    } else {
      console.log('[HistoryList] No text_content found in entry');
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeydown);
    return () => {
      window.removeEventListener('keydown', handleKeydown);
    };
  });
</script>

<div class="history-list">
  {#if entries.length === 0}
    <div class="empty-state">
      <p>No clipboard history yet</p>
      <p class="hint">Copy something to get started</p>
    </div>
  {:else}
    <div class="items">
      {#each entries as entry, index}
        <HistoryItem
          {entry}
          selected={index === selected}
          onclick={() => handleSelect(entry)}
        />
      {/each}
    </div>
  {/if}
</div>

<style>
  .history-list {
    flex: 1;
    overflow-y: auto;
    background: white;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #999;
    text-align: center;
    padding: 40px 20px;
  }

  .empty-state p {
    margin: 0;
    font-size: 16px;
  }

  .hint {
    font-size: 14px;
    margin-top: 8px;
  }

  .items {
    max-height: 350px;
    overflow-y: auto;
  }

  .items::-webkit-scrollbar {
    width: 8px;
  }

  .items::-webkit-scrollbar-track {
    background: #f1f1f1;
  }

  .items::-webkit-scrollbar-thumb {
    background: #c1c1c1;
    border-radius: 4px;
  }

  .items::-webkit-scrollbar-thumb:hover {
    background: #a8a8a8;
  }
</style>
