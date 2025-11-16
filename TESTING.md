# 测试说明

## 粘贴功能测试

由于macOS的辅助功能权限限制，我添加了一个测试模式，可以在不需要辅助功能权限的情况下测试核心功能。

### 可用的命令

应用提供了两个粘贴相关的命令：

1. **`paste_and_close`** - 完整的粘贴功能
   - 复制内容到剪贴板
   - 模拟键盘快捷键（Cmd+V 或 Ctrl+V）自动粘贴
   - 关闭窗口
   - ⚠️ **需要辅助功能权限** (macOS: 系统设置 > 隐私与安全性 > 辅助功能)

2. **`copy_to_clipboard_only`** - 测试模式（新增）
   - 只复制内容到剪贴板
   - 不模拟键盘输入
   - 关闭窗口
   - ✅ **不需要辅助功能权限**

### 如何使用测试模式

在前端代码中，你可以临时使用 `copy_to_clipboard_only` 命令来测试：

```typescript
// 原来的粘贴命令
await invoke('paste_and_close', { content: clipboardText });

// 测试模式（不需要辅助功能权限）
await invoke('copy_to_clipboard_only', { content: clipboardText });
```

### 测试步骤

1. **启动应用**
   ```bash
   bun run tauri dev
   ```

2. **复制一些文本** - 应用会自动保存到历史记录

3. **打开剪贴板历史窗口** - 点击系统托盘图标

4. **选择一个历史记录** - 使用测试模式时：
   - 内容会被复制到剪贴板
   - 窗口会关闭
   - 然后你可以手动按 Cmd+V (或 Ctrl+V) 来粘贴

5. **验证功能**
   - 检查内容是否正确复制到剪贴板
   - 窗口是否正确关闭
   - 去重功能是否工作（复制相同内容时只更新时间戳）

### 错误处理

如果在使用 `paste_and_close` 时遇到权限错误，应用会显示友好的错误消息而不是崩溃：

```
Failed to initialize keyboard simulator: [error details].
On macOS, please grant Accessibility permissions in
System Settings > Privacy & Security > Accessibility.
```

### 授予辅助功能权限（macOS）

如果你想使用完整的自动粘贴功能：

1. 打开 **系统设置** (System Settings)
2. 进入 **隐私与安全性** (Privacy & Security)
3. 点击 **辅助功能** (Accessibility)
4. 点击 **+** 按钮添加应用
5. 找到并添加 **superkbd** 应用
6. 确保开关是打开状态

重启应用后，`paste_and_close` 命令就可以正常工作了。

## 单元测试

运行Rust单元测试：

```bash
cd src-tauri
cargo test --lib
```

当前所有 19 个测试都应该通过，包括：
- 数据库操作测试
- 去重逻辑测试
- 剪贴板监控测试
- 窗口管理测试
