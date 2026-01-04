# lapce-rust

<<<<<<< HEAD
Lapce plugin for the Rust programming language, providing Language Server Protocol (LSP) support through [rust-analyzer](https://github.com/rust-lang/rust-analyzer).

This project is a fork of the official Lapce Rust plugin:
https://github.com/lapce/lapce-rust

---

## Requirements

This plugin **does not bundle rust-analyzer**.

You must have **rust-analyzer installed and available in your system `PATH`**, or configure its location manually in the plugin settings.

### Install rust-analyzer (recommended)

If you use `rustup`:

```bash
rustup component add rust-analyzer
rust-analyzer --version
```
### Configure rust-analyzer path manually (optional)

Open the plugin settings (Ctrl + ,) and set **Server Path** to the location of your rust-analyzer executable.
You can find the path by running:
```bash
which rust-analyzer # on Linux/macOS
where rust-analyzer # on Windows
```
=======
Lapce plugin for Rust programming language, which provides the LSP through [Rust-Analyzer](https://github.com/rust-lang/rust-analyzer)
This project is a fork of https://github.com/lapce/lapce-rust
>>>>>>> bfd9687f840c079cd9bbbe8db328fc114c42d911
