import { vi } from 'vitest';

// Mock Tauri APIs for testing
const mockIPC = vi.fn();

// Mock @tauri-apps/api modules
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockIPC,
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
  emit: vi.fn(() => Promise.resolve()),
}));

vi.mock('@tauri-apps/plugin-global-shortcut', () => ({
  register: vi.fn(() => Promise.resolve()),
  unregister: vi.fn(() => Promise.resolve()),
}));

// Global test utilities
global.mockIPC = mockIPC;
