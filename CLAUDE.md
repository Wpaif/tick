# tick

Torrent interface for Omarchy (Rust + QML/Quickshell) on top of `qbittorrent-nox`.

- Toolchain: Rust 1.99.1 pinned via mise (`mise.toml`); use `mise exec -- cargo ...` if the shell doesn't activate mise.
- Roadmap and architecture: `ROADMAP.md`. Follow the current phase.
- To plan or review, use the `tick-guide` agent.
- Reference for visuals, keybindings and protocol: flea, at `/usr/share/flea/ui` (`Backend.qml`, `Theme.qml`).
- Reply to the user in Brazilian Portuguese. Everything in the repository (code, comments, docs, commit messages) is in English.
- The user is learning Rust by doing: guide and explain, don't write the code for them unless asked.
- TDD: every new behavior starts with a failing test (red → green → refactor). Details in the `tick-guide` agent.
- Community project: follow `CONTRIBUTING.md`. Never commit to `main`; create a branch (`feat/`, `fix/`, `docs/`...) and use Conventional Commits.
