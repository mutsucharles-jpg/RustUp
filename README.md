# RustUp

A small Rust command-line program that greets you by name.

## Requirements

- [Rust](https://rustup.rs) (stable toolchain, installed via `rustup`)
- On Windows (MSVC target): Visual Studio Build Tools with the C++ workload

## Usage

```sh
cargo run                # Hello, world!
cargo run -- Charles     # Hello, Charles!
```

The `--` separates cargo's own arguments from the program's. With no name,
the greeting defaults to "world".

## Testing

```sh
cargo test
```

## Project layout

- `src/main.rs` — the `greeting` function, `main`, and unit tests
- `Cargo.toml` — package manifest
