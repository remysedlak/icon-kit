# icon_enum

A small Rust crate for generating enums from icon filenames in a directory.  
This is useful for creating a Rust-friendly namespace for your icons automatically.

## Library
```rust
use icon_enum::create_enum_file;

let file = create_enum_file("assets/icons", "src/icon.rs"); // generate namespace
```

## CLI
install
```install
cargo install icon_enum
```
usage
```text
icon_enum assets/icons src/icon.rs
```
