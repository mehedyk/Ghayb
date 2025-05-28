# Current Version (WebView2-based) — README.md

## Ghayb Secure Browser (Phase 2)

Rust wrapper around Microsoft Edge WebView2 for a fully featured, secure browser.

## Developer
**Mehedyk**

### Features

* **Full HTML/CSS/JS** support via embedded Edge/Chromium
* **Minimal code**: \~30 lines in `main.rs`
* **Secure**: DevTools disabled, HTTPS default
* **Lightweight**: No heavy GUI crates

### Prerequisites

* Windows 10+ with **WebView2 Runtime** installed
  (Evergreen Bootstrapper from Microsoft)
* Rust toolchain (via `rustup`)

### Installation

```sh
# Clone
git clone https://github.com/mehedyk/ghayb.git
cd ghayb

# Build
cargo build --release

# Run
target\release\ghayb.exe
```

### Usage

* Starts at `https://www.example.com` by default
* Resize window, navigate just like a normal browser
* To enable DevTools, rebuild with `.debug(true)` in `main.rs`

### Customization

* Change default URL in `src/main.rs`
* Intercept links by implementing `invoke_handler`

### License

MIT © 2025 mehedyk

---

# LICENSE (MIT)

```
MIT License

Copyright (c) 2025 mehedyk
