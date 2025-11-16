import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import SearchInput from './SearchInput.svelte';

describe('SearchInput', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders search input', () => {
    render(SearchInput);
    const input = screen.getByPlaceholderText('Search clipboard history...');
    expect(input).toBeTruthy();
  });

  it('has correct input type', () => {
    render(SearchInput);
    const input = screen.getByPlaceholderText('Search clipboard history...') as HTMLInputElement;
    expect(input.type).toBe('text');
  });

  it('has autocomplete off', () => {
    render(SearchInput);
    const input = screen.getByPlaceholderText('Search clipboard history...') as HTMLInputElement;
    expect(input.autocomplete).toBe('off');
  });
});
