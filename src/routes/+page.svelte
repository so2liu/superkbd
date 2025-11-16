<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import SearchInput from '$lib/components/SearchInput.svelte';
  import HistoryList from '$lib/components/HistoryList.svelte';
  import PermissionDialog from '$lib/components/PermissionDialog.svelte';
  import { loadHistory, initializeListeners } from '$lib/stores/clipboard';

  // Handle Esc key to hide window
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      invoke('hide_window_command').catch(console.error);
    }
  }

  onMount(async () => {
    // Initialize event listeners
    initializeListeners();

    // Load initial history
    await loadHistory();

    // Global shortcut (Alt+I) is registered in Rust - see src-tauri/src/lib.rs
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app">
  <SearchInput />
  <HistoryList />
</div>

<PermissionDialog />

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
      'Helvetica Neue', Arial, sans-serif;
  }

  .app {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: rgba(255, 255, 255, 0.95);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  }
</style>
