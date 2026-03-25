---
title: Hooks
summary: Guide to configuring lifecycle hooks in bulletty
show_datetime: false
---

**bulletty** supports hooks. Set up commands that run at key lifecycle points. That let you integrate bulletty with your desktop environment, notification system, or any external tool.

## ⚙️ Configuration

Hooks are in your **local configuration**. That means it's not synced with your library, allowing you to have different hooks for different systems that interact with the same library. They can be configured in your local config dir (`bulletty dirs local-config`) `$CONFIG_DIR/config.toml` under a `[hooks]` table. All fields are optional.

```toml
[hooks]
before_tui = "some-command"
after_tui  = "some-command"
open_link  = "xdg-open %s"
```

## 🪝 Available Hooks

### 💠 `before_tui` & `after_tui`

A shell command run via `sh -c` immediately before the TUI launches. Use this to set up your environment, start background services, synchronize your library, or display a notification:

```toml
[hooks]
before_tui = "notify-send 'bulletty' 'Starting RSS reader'"
after_tui = "notify-send 'bulletty' 'Closed RSS reader'"
```

If the command fails, the error is logged but bulletty still starts normally.

### 💠 `open_link`

A command template used to open URLs. Use `%s` as the placeholder for the URL. Unlike the other hooks, this command is executed directly (not through a shell).

```toml
[hooks]
open_link = "xdg-open %s"
```

If `open_link` is not set, bulletty falls back to the default system browser.

**USE CASE:**: opening the link on a browser on Windows if you're running bulletty on a WSL system:

```toml
[hooks]
open_link = "powershell.exe -Command Start-Process \"%s\""
```

## ⛓️‍💥 Disabling Hooks at Runtime

Pass the `--no-hooks` flag to skip all hooks for a session:

```
bulletty --no-hooks
```

This is useful for debugging or when you want a clean run without side effects.
