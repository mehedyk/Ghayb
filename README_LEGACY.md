# Legacy Version (Egui-based) — README\_LEGACY.md

## Ghayb Browser (Phase 1)

Minimal, secure browser proof-of-concept using `eframe` + `egui`.

## Developer
**Mehedyk**

### Features

* **Custom UI** built with egui
* **DNS-over-HTTPS** resolution
* **Tabbed** fetching and display of page titles and content snippets
* **Configurable** via `~/.config/ghayb.toml`

### Installation

```sh
# Clone
git clone https://github.com/mehedyk/ghayb.git
cd ghayb

# Ensure Rust toolchain installed
rustup update

# Build and run
cargo run
```

### Dependencies

* `eframe = "0.31"`
* `egui = "0.31"`
* `reqwest = "0.11"` (blocking)
* `scraper = "0.13"`
* `url = "2"`

### Usage

1. Enter a URL in the input box.
2. Click **Go** to fetch and parse title/headers/paragraphs.
3. Tabs appear for each visited site.

### Configuration

Edit `~/.config/ghayb.toml`:

```toml
[network]
proxy = "none"

[logging]
level = "info"
```

### License

MIT © 2025 mehedyk

---
