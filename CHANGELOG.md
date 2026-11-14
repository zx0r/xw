# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-15

### Added

- **Core Engine**: Ultra-fast, zero-allocation terminal telemetry coprocessor written in Rust 2024.
- **Hardware Probes**: Sub-millisecond kernel probes for CPU (`cpu`), Memory (`mem`), Disk space (`ssd`), and System uptime (`uptime`).
- **Git VCS Telemetry**: Stale-While-Revalidate (SWR) cached porcelain v2 Git status widget (`git`) with worktree indicators, branch tracking, and commit SHA.
- **Dialect Rendering**: First-class support for tmux formatting (`--style tmux`), ANSI 256-color escape sequences (`--style ansi`), and plain text (`--style plain`).
- **Nerd Fonts**: Vectorized powerline and nerd-font glyph integration with toggle flag (`--no-icons`).
- **Target Path Resolution**: Out-of-tree Git telemetry resolution via `xw git [PATH]`.

[0.1.0]: https://github.com/zx0r/xw/releases/tag/v0.1.0
