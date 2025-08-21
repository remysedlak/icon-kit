/// # Get enum text from directory path
///
/// # Example 1
/// ```
/// use icon_folder::icons::{search_icons, create_enum_text};
///
/// let path = "assets/icons"; // define path
/// let icons = search_icons(path); // get vector of files
/// let enum_text = create_enum_text(&icons.unwrap()); // get text for enum file
///```
/// # Example 2
///```
/// use icon_folder::generate_enum_file;
///
/// let file = generate_enum_file("assets/icons", "src/icon.rs"); // generate namespace
/// ```

pub mod icons;

pub use icons::{remove_extension, sanitize_filename, search_icons, create_enum_text, generate_enum_file};

