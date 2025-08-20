use std::io;
use std::path::Path;
use std::fs;

/// Convert a filename into a valid Rust enum variant in PascalCase
pub fn sanitize_filename(filename: &str) -> String {
    // First, remove the file extension
    let stem = remove_extension(filename);

    let mut sanitized = String::new();
    let mut capitalize_next = true;

    for c in stem.chars() {
        if c.is_ascii_alphanumeric() {
            if capitalize_next {
                sanitized.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                sanitized.push(c);
            }
        } else {
            // Treat invalid character as word boundary
            if matches!(c, '_' | '-' |'@' |' ') {
                capitalize_next = true;
            }
        }
    }

    // Prefix with '_' if it starts with a number
    if sanitized.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
        sanitized = format!("_{}", sanitized);
    }

    sanitized
}

/// Remove the file extension from a filename
pub fn remove_extension(filename: &str) -> &str {
    Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename)
}

/// Get all icons from a directory
pub fn search_icons(dir_path: &str) -> Result<Vec<String>, io::Error> {
    let dir = Path::new(dir_path);
    let mut paths: Vec<String> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // Convert PathBuf to String
            let path_str = path.to_string_lossy().into_owned();
            paths.push(path_str);
        }
    }

    Ok(paths)
}

pub fn create_enum_text(paths: &Vec<String>) -> Result<String, io::Error> {

    const START: &str = "pub enum Icon {";
    const START_END_BRACKET: char = '}';
    const MIDDLE: &str = "impl Icon { pub fn path(&self) -> &'static str { match self {";
    const ICON_START: &str = "Icon::";
    const ARROW: &str = " => ";
    const  MIDDLE_BRACKET: &str = "}}}";

    let mut enum_content = String::from("");
    enum_content.push_str(START);
    for path in paths {
        enum_content.push_str(&sanitize_filename(&path));
        enum_content.push(',');
    }
    enum_content.push(START_END_BRACKET);
    enum_content.push_str(MIDDLE);

    for path in paths {
        enum_content.push_str(ICON_START);
        enum_content.push_str(&sanitize_filename(&path));
        enum_content.push_str(ARROW);

        enum_content.push('"');
        enum_content.push_str(&path.replace("\\", "/"));
        enum_content.push('"');

        enum_content.push_str(", ");
    }
    enum_content.push_str(MIDDLE_BRACKET);

    Ok(enum_content)
}
