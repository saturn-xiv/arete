## Rust

-   Install rustup

```bash
curl https://sh.rustup.rs -sSf | sh
```

-   Add to ~/.zshrc

```bash
source $HOME/.cargo/env
export RUSTFLAGS="-Aproc-macro-derive-resolution-fallback" # https://github.com/diesel-rs/diesel/issues/1785
```

-   Re-login at first

-   Install rust

```bash
rustup override set nightly
rustup component add rustfmt rls-preview rust-analysis rust-src clippy
```
