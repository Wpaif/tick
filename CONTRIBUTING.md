# Contributing to tick

Thanks for wanting to help! Before you start, read `ROADMAP.md`: it says which phase the project is in and what comes next.

## Setting up

```bash
mise install                          # installs the Rust version pinned in mise.toml
git config core.hooksPath .githooks   # enables the project's git hooks
```

To run it locally you need `qbittorrent-nox` with the Web UI on `127.0.0.1:8080`.

## Workflow

`main` is protected: **nobody commits directly to it**, maintainers included. All work lands through pull requests.

1. Update main: `git switch main && git pull`
2. Create a branch prefixed by the kind of change:
   - `feat/` — new feature (`feat/tick-list`)
   - `fix/` — bug fix (`fix/login-204`)
   - `docs/` — documentation only
   - `refactor/` — internal change with no behavior change
   - `chore/` — build, dependencies, CI
3. Make small commits, each one compiling.
4. Open a pull request against `main` describing what and why.

## Commit messages

We follow [Conventional Commits](https://www.conventionalcommits.org/), in English, imperative mood:

```
feat: add tick list command
fix: handle 204 login response on API 5.x
docs: explain config.toml setup
```

## Before opening a PR

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

- Never commit secrets: passwords go in `.env` (ignored by git) or in `~/.config/tick/config.toml`.
- Changes to the architecture (described in `ROADMAP.md`) must be discussed in an issue first.
