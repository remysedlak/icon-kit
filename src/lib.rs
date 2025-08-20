use std::path::{Path};
use std::fs;
use std::io;

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
            if matches!(c, '_' | '-' |'@' |' '){
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_basic() {
        assert_eq!(sanitize_filename("ch*eck_box.jpg"), "CheckBox");
        assert_eq!(sanitize_filename("search-icon@2x.svg"), "SearchIcon2x");
        assert_eq!(sanitize_filename("home page.png"), "HomePage");
        assert_eq!(sanitize_filename("settings@dark-mode.svg"), "SettingsDarkMode");
        assert_eq!(sanitize_filename("123-start.svg"), "_123Start");
        assert_eq!(sanitize_filename("!weird__name!.svg"), "WeirdName");
        assert_eq!(sanitize_filename("CAPSLOCK.PNG"), "CAPSLOCK");
    }

    #[test]
    fn test_remove_extension() {
        assert_eq!(remove_extension("file.txt"), "file");
        assert_eq!(remove_extension("archive.tar.gz"), "archive.tar");
        assert_eq!(remove_extension("no_extension"), "no_extension");
    }

    #[test]
    fn test_icon_path() {
        // Call search_icons and unwrap or assert success
        let paths = search_icons("assets/icons").expect("Failed to read icon directory");

        // Make sure at least one icon was found
        assert!(!paths.is_empty(), "No icons found in directory");

        // Optionally check that each path exists and is a file
        for path_str in &paths {
            let path = Path::new(path_str);
            assert!(path.exists(), "File does not exist: {:?}", path);
            assert!(path.is_file(), "Path is not a file: {:?}", path);
        }

        // Optional: print found paths for debugging
        for path_str in &paths {
            println!("Found path: {}", path_str.replace("\\", "/"));
        }
    }

}
