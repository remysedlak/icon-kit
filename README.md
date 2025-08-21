# icon_enums

A small Rust crate for generating enums from icon filenames in a directory.  
This is useful for creating a Rust-friendly namespace for your icons automatically.

## Code Usage

```rust
use icon_enums::create_enum_file;

let file = create_enum_file("assets/icons", "src/icon.rs"); // generate namespace
```

## CLI  Usage

```text
cargo run --bin icon_enums -- <icon directory> <enum output>
```

### Example
```text
cargo run --bin icon_enums -- assets/icons src/icon.rs
```
