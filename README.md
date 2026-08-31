# ?? OpenCode Info

<img src="assets/icon.svg" height="36" alt="OpenCode Info icon" align="left">

An [OpenAction](https://openaction.amankhanna.me/) plugin that brings your OpenCode Go usage onto any [OpenDeck](https://github.com/nekename/OpenDeck) key.

It keeps an eye on your plan limits so you can see, at a glance, how much of your allowance you have left — without interrupting your flow.

## How it works

The plugin queries your OpenCode Go account and renders your usage on a key as a colour-coded progress bar. Keys are flexible: you can show a single limit, cycle through them with a tap, or show several at once. Bars change colour based on thresholds you can tune, and the values refresh automatically.

## Actions

| Action | UUID | What it does |
|---|---|---|
| **Go Usage (single window)** | `com.dahrkael.opencodeinfo.window` | Shows a single, configurable usage window (5h, week or month) |
| **Go Usage (rotate)** | `com.dahrkael.opencodeinfo.rotate` | Cycles between the 5h, weekly and monthly windows with every press |
| **Go Usage (summary)** | `com.dahrkael.opencodeinfo.summary` | Shows all three windows (5h, week, month) at once |

## Requirements

- An [OpenAction](https://openaction.amankhanna.me/) server such as [OpenDeck](https://github.com/nekename/OpenDeck) running on **Windows** or **Linux** (x86_64).
- An OpenCode Go API key from your OpenCode account, entered once in the plugin's settings.

## Installation

### Via the OpenDeck Plugin Manager (recommended)

1. Open OpenDeck and go to the **Plugins** tab in Settings.
2. Click **Install from URL** and paste:

   ```
   https://github.com/Dahrkael/opencode-info-openaction
   ```

   Or download the latest `com.dahrkael.opencodeinfo.sdPlugin.zip` from the [Releases](../../releases/latest) page and install it from the Plugins tab.

### Manual installation

1. Download `com.dahrkael.opencodeinfo.sdPlugin.zip` from the [Releases](../../releases/latest) page.
2. Extract it into your OpenDeck plugins directory:
   - **Windows**: `%appdata%\opendeck\plugins\`
   - **Linux**: `~/.local/share/opendeck/plugins/`
3. Restart OpenDeck.

## Configuration

After installing, place any of the actions on a key and open its settings:

- **API key** — your OpenCode Go key (required).
- **Window** — which limit to show (only for the single-window action).
- **Refresh (s)** — how often to re-query usage.
- **Text color** — the colour of the rendered text.
- **Yellow = / Red =** — the percentage thresholds that switch the bar colour.

## Building from source

```bash
# Windows
build-windows.bat

# Linux
./build-linux.sh
```

Each produces an `opencode-info.streamDeckPlugin` ready to install.

---

Built with the [OpenAction Rust crate](https://github.com/OpenActionAPI/rust).
