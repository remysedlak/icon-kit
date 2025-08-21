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

## Example Output
```rust
pub enum Icon {
    Ferris,
    GithubMark,
    Linkedin,
    Menu,
    OpenLinkBlue,
    OpenLinkIndigo,
    OpenLink,
}
impl Icon {
    pub fn path(&self) -> &'static str {
        match self {
            Icon::Ferris => "assets/icons/ferris.svg",
            Icon::GithubMark => "assets/icons/github-mark.svg",
            Icon::Linkedin => "assets/icons/linkedin.svg",
            Icon::Menu => "assets/icons/menu.svg",
            Icon::OpenLinkBlue => "assets/icons/open-link-blue.svg",
            Icon::OpenLinkIndigo => "assets/icons/open-link-indigo.svg",
            Icon::OpenLink => "assets/icons/open-link.svg",
        }
    }
}
```
