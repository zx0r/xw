<p align="center">
  <a href="https://github.com/zx0r/xw/releases">
    <img src="https://img.shields.io/github/v/release/zx0r/xw?color=4be1ff&label=RELEASE&style=flat-square&logo=github&labelColor=1a1a1a" alt="Release" />
  </a>
  <a href="LICENSE-MIT">
    <img src="https://img.shields.io/badge/LICENSE-MIT%2FApache--2.0-9fef00?style=flat-square&logo=opensourceinitiative&logoColor=white&labelColor=1a1a1a" alt="License" />
  </a>
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/CORE-RUST_2024-orange?style=flat-square&logo=rust&logoColor=white&labelColor=1a1a1a" alt="Rust 2024" />
  </a>
</p>

<p align="center">
  <img src="assets/images/xw.png" alt="xw telemetry in tmux — zero-allocation status widgets" style="border-radius: 8px; max-width: 100%;" />
</p>

---

### Installation

```bash
# Cargo (crates.io)
cargo install xw

# Homebrew (macOS / Linux)
brew install zx0r/tap/xw

# Build from source (LTO fat, stripped)
gh repo clone zx0r/xw
cd xw && just release
```

---

### Integration

```bash
# TOP Bar pane-border-format
set -gF @mod_git "xw git"

# Bottom Bar
set -gF @mod_hw "xw cpu mem ssd uptime"
set -g @mod_right "#(#{E:@mod_hw}) #{E:@zoom}"
```

---

### [ l i n e x ]

<p align="center">
  <img src="./assets/gif/xstory-crab.gif" width="360" height="" alt="xstory" />
</p>

```shell
xw [WIDGET] [OPTIONS]

[✓] zero-allocation stack formatting
[✓] sub-millisecond telemetry probes
[✓] native tmux and ANSI dialects
[✓] zero runtime dependencies

fast telemetry → rendered instantly for tmux and status lines
```

---

### License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
