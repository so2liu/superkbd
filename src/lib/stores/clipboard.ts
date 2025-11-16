import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface ClipboardEntry {
  id: number;
  content_type: string;
  text_content: string | null;
  file_path: string | null;
  metadata: string | null;
  created_at: number;
  favorite: boolean;
  source_app: string | null;
}

export const searchQuery = writable('');
export const allEntries = writable<ClipboardEntry[]>([]);
export const selectedIndex = writable(0);

// Derived store for filtered entries
export const filteredEntries = derived(
  [allEntries, searchQuery],
  ([$allEntries, $searchQuery]) => {
    if (!$searchQuery.trim()) {
      return $allEntries;
    }

    const query = $searchQuery.toLowerCase();
    return $allEntries.filter(entry =>
      entry.text_content?.toLowerCase().includes(query)
    );
  }
);

export async function loadHistory() {
  try {
    const entries = await invoke<ClipboardEntry[]>('get_clipboard_history', {
      searchQuery: null,
      limit: 100,
      favoritesOnly: false
    });
    allEntries.set(entries);
  } catch (error) {
    console.error('Failed to load clipboard history:', error);
  }
}

export async function toggleFavorite(id: number) {
  try {
    await invoke('toggle_favorite', { id });
    await loadHistory();
  } catch (error) {
    console.error('Failed to toggle favorite:', error);
  }
}

export async function pasteAndClose(content: string) {
  try {
    console.log('[Frontend] pasteAndClose called with:', content);
    await invoke('paste_and_close', { content });
    console.log('[Frontend] paste_and_close command completed');
  } catch (error) {
    console.error('[Frontend] Failed to paste:', error);
  }
}

export async function hideWindow() {
  try {
    await invoke('hide_window_command');
  } catch (error) {
    console.error('Failed to hide window:', error);
  }
}

// Listen for clipboard updates
export function initializeListeners() {
  listen('clipboard-update', async () => {
    await loadHistory();
  });

  listen('window-shown', () => {
    selectedIndex.set(0);
    searchQuery.set('');
  });
}
