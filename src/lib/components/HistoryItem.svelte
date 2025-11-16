<script lang="ts">
  import type { ClipboardEntry } from '$lib/stores/clipboard';
  import { toggleFavorite } from '$lib/stores/clipboard';

  interface Props {
    entry: ClipboardEntry;
    selected: boolean;
    onclick: () => void;
  }

  let { entry, selected, onclick }: Props = $props();

  function formatTimestamp(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diff = now.getTime() - date.getTime();

    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 1) return 'Just now';
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    if (days < 7) return `${days}d ago`;

    return date.toLocaleDateString();
  }

  function truncateText(text: string | null, maxLength: number = 100): string {
    if (!text) return '';
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength) + '...';
  }

  async function handleFavoriteClick(e: MouseEvent) {
    e.stopPropagation();
    await toggleFavorite(entry.id);
  }
</script>

<div
  class="history-item"
  class:selected
  role="button"
  tabindex="0"
  onclick={onclick}
  onkeypress={(e) => e.key === 'Enter' && onclick()}
>
  <div class="item-content">
    <div class="item-text">{truncateText(entry.text_content)}</div>
    <div class="item-meta">
      {#if entry.source_app}
        <span class="source-app">{entry.source_app}</span>
        <span class="separator">•</span>
      {/if}
      <span class="timestamp">{formatTimestamp(entry.created_at)}</span>
      {#if entry.favorite}
        <span class="favorite-badge">★</span>
      {/if}
    </div>
  </div>
  <button
    class="favorite-btn"
    class:active={entry.favorite}
    onclick={handleFavoriteClick}
    title={entry.favorite ? 'Unfavorite' : 'Favorite'}
  >
    {entry.favorite ? '★' : '☆'}
  </button>
</div>

<style>
  .history-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-bottom: 1px solid #f0f0f0;
    cursor: pointer;
    background: white;
    transition: background-color 0.15s;
  }

  .history-item:hover {
    background: #f8f8f8;
  }

  .history-item.selected {
    background: #e3f2fd;
    border-left: 3px solid #007aff;
    padding-left: 13px;
  }

  .item-content {
    flex: 1;
    min-width: 0;
  }

  .item-text {
    font-size: 14px;
    line-height: 1.4;
    color: #333;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .item-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 4px;
  }

  .source-app {
    font-size: 11px;
    color: #666;
    font-weight: 500;
    background: #f0f0f0;
    padding: 2px 6px;
    border-radius: 3px;
  }

  .separator {
    font-size: 12px;
    color: #ccc;
  }

  .timestamp {
    font-size: 12px;
    color: #999;
  }

  .favorite-badge {
    color: #ffa500;
    font-size: 12px;
  }

  .favorite-btn {
    padding: 4px 8px;
    border: none;
    background: transparent;
    font-size: 18px;
    color: #ccc;
    cursor: pointer;
    transition: color 0.2s;
  }

  .favorite-btn:hover {
    color: #ffa500;
  }

  .favorite-btn.active {
    color: #ffa500;
  }
</style>
