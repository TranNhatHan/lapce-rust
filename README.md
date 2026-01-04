# lapce-rust

Lapce plugin for the Rust programming language, providing Language Server Protocol (LSP) support through [rust-analyzer](https://github.com/rust-lang/rust-analyzer).

This project is a fork of the official Lapce Rust plugin:
https://github.com/lapce/lapce-rust

---

## Requirements

This plugin can use **rust-analyzer** in three ways:

1. **Server Path** (recommended)
   Set the full path to your `rust-analyzer` executable in the plugin settings under **Server Path**.

2. **System PATH**
   If `Server Path` is empty, the plugin will attempt to use `rust-analyzer` available in your system `PATH`.
   Verify with:

   ```bash
   rust-analyzer --version
   ```

3. **Automatic Download**
  If rust-analyzer is not found in PATH, the plugin will automatically download the appropriate binary for your system based on VOLT_OS and VOLT_ARCH.

## Install rust-analyzer (optional)

If you want to manage it manually with rustup:

```bash
rustup component add rust-analyzer
rust-analyzer --version
```

## Configure rust-analyzer path manually (optional)

Open the plugin settings (Ctrl + ,) and set Server Path to the location of your rust-analyzer executable.
You can find the path by running:
```bash
which rust-analyzer # on Linux/macOS
where rust-analyzer # on Windows
```
## Contributing

Contributions and bug reports are welcome! Please open a pull request or an issue in this repository.
