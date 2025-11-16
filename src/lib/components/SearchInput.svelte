<script lang="ts">
  import { onMount } from 'svelte';
  import { searchQuery } from '$lib/stores/clipboard';

  let inputElement: HTMLInputElement;
  let query = $state('');

  // Sync with store
  $effect(() => {
    searchQuery.set(query);
  });

  onMount(() => {
    // Auto-focus on mount
    inputElement?.focus();

    return () => {
      query = '';
    };
  });

  // Focus when query changes
  $effect(() => {
    if (query === '') {
      inputElement?.focus();
    }
  });
</script>

<div class="search-container">
  <input
    bind:this={inputElement}
    bind:value={query}
    type="text"
    placeholder="Search clipboard history..."
    class="search-input"
    autocomplete="off"
  />
</div>

<style>
  .search-container {
    padding: 16px;
    background: rgba(255, 255, 255, 0.95);
    border-bottom: 1px solid #e0e0e0;
  }

  .search-input {
    width: 100%;
    padding: 12px 16px;
    font-size: 16px;
    border: 2px solid #e0e0e0;
    border-radius: 8px;
    outline: none;
    transition: border-color 0.2s;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    box-sizing: border-box;
  }

  .search-input:focus {
    border-color: #007aff;
  }

  .search-input::placeholder {
    color: #999;
  }
</style>
