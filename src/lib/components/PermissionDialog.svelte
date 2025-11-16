<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';

  let showDialog = $state(false);
  let isCheckingPermission = $state(false);

  onMount(() => {
    // Listen for permission needed event
    const unlisten = listen('accessibility-permission-needed', () => {
      showDialog = true;
    });

    // Check permission on mount
    checkPermission();

    return () => {
      unlisten.then((fn) => fn());
    };
  });

  async function checkPermission() {
    try {
      const hasPermission = await invoke<boolean>('check_accessibility_permission');
      if (!hasPermission) {
        showDialog = true;
      }
    } catch (error) {
      console.error('Failed to check accessibility permission:', error);
    }
  }

  async function openSettings() {
    try {
      await invoke('open_accessibility_settings');
      // Check again after a delay
      setTimeout(async () => {
        isCheckingPermission = true;
        await checkPermission();
        const hasPermission = await invoke<boolean>('check_accessibility_permission');
        if (hasPermission) {
          showDialog = false;
        }
        isCheckingPermission = false;
      }, 3000);
    } catch (error) {
      console.error('Failed to open settings:', error);
    }
  }

  async function recheckPermission() {
    isCheckingPermission = true;
    const hasPermission = await invoke<boolean>('check_accessibility_permission');
    if (hasPermission) {
      showDialog = false;
    }
    isCheckingPermission = false;
  }

  function dismissDialog() {
    showDialog = false;
  }
</script>

{#if showDialog}
  <div class="permission-overlay">
    <div class="permission-dialog">
      <h2>需要辅助功能权限</h2>
      <p>SuperKBD 需要辅助功能权限才能实现自动粘贴功能。</p>

      <div class="steps">
        <h3>如何授权：</h3>
        <ol>
          <li>点击下方"打开系统设置"按钮</li>
          <li>在隐私与安全性 > 辅助功能中找到 SuperKBD</li>
          <li>启用 SuperKBD 的开关</li>
          <li>返回应用并点击"重新检查权限"</li>
        </ol>
      </div>

      <div class="actions">
        <button onclick={openSettings} class="primary">
          打开系统设置
        </button>
        <button onclick={recheckPermission} disabled={isCheckingPermission}>
          {isCheckingPermission ? '检查中...' : '重新检查权限'}
        </button>
        <button onclick={dismissDialog} class="secondary">
          稍后提醒
        </button>
      </div>

      <p class="note">
        注意：没有此权限，应用只能复制内容到剪贴板，您需要手动按 Cmd+V 粘贴。
      </p>
    </div>
  </div>
{/if}

<style>
  .permission-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }

  .permission-dialog {
    background: white;
    border-radius: 12px;
    padding: 24px;
    max-width: 500px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  }

  h2 {
    margin: 0 0 16px 0;
    color: #ff9500;
    font-size: 20px;
  }

  h3 {
    margin: 16px 0 8px 0;
    font-size: 14px;
    color: #333;
  }

  p {
    margin: 0 0 16px 0;
    color: #666;
    line-height: 1.5;
  }

  .steps {
    background: #f5f5f5;
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 20px;
  }

  ol {
    margin: 0;
    padding-left: 20px;
  }

  li {
    margin: 8px 0;
    color: #333;
  }

  .actions {
    display: flex;
    gap: 8px;
    margin-bottom: 16px;
  }

  button {
    flex: 1;
    padding: 10px 16px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s;
  }

  button.primary {
    background: #007aff;
    color: white;
  }

  button.primary:hover {
    background: #0051d5;
  }

  button:not(.primary):not(.secondary) {
    background: #34c759;
    color: white;
  }

  button:not(.primary):not(.secondary):hover {
    background: #248a3d;
  }

  button.secondary {
    background: #e5e5e5;
    color: #333;
  }

  button.secondary:hover {
    background: #d1d1d1;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .note {
    font-size: 12px;
    color: #999;
    margin: 0;
  }
</style>
