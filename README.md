<h1 align="center">panda</h1>
<br />
<div align="center">
    <!-- Crates version -->
    <a href="https://crates.io/crates/panda">
    <img src="https://img.shields.io/crates/v/panda?style=flat-square">
    </a>
</div>

A powerful async Rust library for interacting with Discord's API

Even thought this library is usable, it still under development, so don't use for production yet.

> Note that this library doesn't support the 100% of discord API yet, for example voice. See `TODO list` to more information.

# Installation

`panda` supports a minimum of Rust 1.4.1

```
cargo add panda
```

or in `Cargo.toml`

```
panda = "0.1.0"
```

# Example usage

```rust

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = panda::new("token").await?;

    client.on_ready(|s, ready| async move {
        println!("Bot {} is ready", ready.user().username());
    });

    client.start().await?;
}
```
