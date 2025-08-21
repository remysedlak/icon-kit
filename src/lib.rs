/// # Get namespace file from directory path
///
/// # Example
///```
/// use icon_enums::create_enum_file;
///
/// let file = create_enum_file("assets/icons", "src/icon.rs"); // generate namespace
/// ```

pub mod icons;

pub use icons::{create_enum_text, create_enum_file};

