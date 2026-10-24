# tick — roadmap

Torrent interface for Omarchy, with the same look and the same keybindings as flea.
The engine is `qbittorrent-nox`; tick only shows and controls what it does.

## Architecture

```
┌──────────────────────┐  line-delimited JSON  ┌──────────────────┐  HTTP (Web API v2)  ┌─────────────────┐
│ UI  (QML/Quickshell) │ ◄──── stdin/stdout ──► │ tick --backend   │ ◄────────────────► │ qbittorrent-nox │
│ ui/*.qml             │                        │ (Rust)           │   localhost:8080    │ (systemd --user)│
└──────────────────────┘                        └──────────────────┘                     └─────────────────┘
```

Same design as flea: the QML UI starts `tick --backend` and talks to it through JSON
messages, one per line. The UI never talks to qBittorrent directly.

- **Rust (`src/`)**: qBittorrent API client, UI protocol, CLI and TUI.
- **QML (`ui/`)**: Quickshell window, theme read from `~/.local/state/omarchy/current/theme/colors.toml`.

## Phase 0 — Foundation
- [x] Install `qbittorrent-nox` and create the `systemd --user` service
- [x] Bind the Web UI to `127.0.0.1` only, with username and password
- [x] Crates: `tokio`, `reqwest` (with cookies), `serde`/`serde_json`, `anyhow`, `clap`
- [x] Config file at `~/.config/tick/config.toml` (URL, username; password outside the file or with mode 600)

## Phase 1 — Basics (MVP)
- [ ] API client: `auth/login`, `torrents/info`, `torrents/add`, pause/resume (`stop`/`start` on API 5.x), `delete`
- [ ] CLI to test without the UI: `tick list`, `tick add <magnet|file>`, `tick pause <hash>`
- [ ] `--backend` protocol: commands `list`, `add`, `pause`, `resume`, `remove`; `torrents` events updated every 1–2 s (via `sync/maindata`)
- [ ] UI: list with name, progress, speed, ETA and state
- [ ] Keybindings like flea: `j`/`k`, `gg`/`G`, `Space` pause/resume, `dd` remove (with confirmation), `a` add, `/` filter, `?` help
- [ ] Omarchy theme applied and updated when the theme changes

## Phase 2 — System integration
- [ ] Open magnet links: `.desktop` file with `x-scheme-handler/magnet` and `.torrent`
- [ ] Notification when a download finishes
- [ ] Open the torrent's folder in flea (`o`)
- [ ] Hyprland keybinding and entry in the Omarchy menu
- [ ] Arch package (PKGBUILD)

## Phase 3 — Fine control
- [ ] Details panel (`i`): files, peers, trackers
- [ ] Choose files and priorities
- [ ] Speed limits (global and per torrent) and alternative mode
- [ ] Categories and tags, with a destination folder per category
- [ ] Sequential download

## Phase 4 — Automation
- [ ] RSS: feeds and auto-download rules
- [ ] Search with qBittorrent plugins
- [ ] Speed scheduling

## Phase 5 — Extras
- [ ] Terminal mode (`tick --tui`, with ratatui) using the same core
- [ ] Indicator in the Omarchy bar
- [ ] Connect to a qBittorrent on another machine
