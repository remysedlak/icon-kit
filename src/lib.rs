/// # CLI
///
/// This crate provides a command-line interface for generating Rust enums
/// from a directory of icon files.
///
/// # Usage
///
/// ```text
/// cargo run --bin icon_enum -- assets/icons icon.rs
/// ```
///
/// # Programmatic usage
///
/// ```no_run
/// use icon_enum::create_enum_file;
/// create_enum_file("assets/icons", "icon.rs").unwrap();
/// ```
pub mod cli_docs {}

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

/// return true if a character has an accent
fn has_accent(c: char) -> bool {
    c.nfkd().any(|d| {
        // combining diacritics are in the Unicode range \u{0300}–\u{036F}
        let u = d as u32;
        (0x0300..=0x036F).contains(&u)
    })
}

/// returns char without accent
fn remove_accents_char(c: char) -> char {
    // Decompose the char
    for d in c.to_string().nfkd() {
        // Pick the first non-combining mark character
        let u = d as u32;
        if !(0x0300..=0x036F).contains(&u) {
            return d;
        }
    }
    // fallback in case something weird happens
    c
}

/// Converts a filename into a valid Rust enum variant in PascalCase.
/// Returns `None` if the filename is not a valid icon.
fn sanitize_filename(filename: &str) -> Option<String> {
    if is_valid_icon(&filename) {
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
                if matches!(c, '_' | '-' | '@' | ' ' | '.') {
                    capitalize_next = true;
                } else if has_accent(c) {
                    sanitized.push(remove_accents_char(c))
                }
            }
        }

        // Prefix with '_' if it starts with a number
        if sanitized
            .chars()
            .next()
            .map(|c| c.is_numeric())
            .unwrap_or(false)
        {
            sanitized = format!("_{}", sanitized);
        }

        Some(sanitized)
    } else {
        None
    }
}

/// Returns `true` if the given file path points to a valid icon file.
fn is_valid_icon(filename: &str) -> bool {
    let path = Path::new(filename);

    // Must have an extension
    let ext = match path.extension().and_then(|s| s.to_str()) {
        Some(e) => e.to_lowercase(),
        None => return false, // no extension
    };

    // Only allow certain extensions
    let valid_extensions = ["png", "jpg", "jpeg", "svg", "webp"];
    if !valid_extensions.contains(&ext.as_str()) {
        return false;
    }

    // Must have a stem (the part before extension)
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    if stem.trim().is_empty() {
        return false;
    }

    true
}

/// Removes the file extension from a filename.
fn remove_extension(filename: &str) -> &str {
    Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename)
}

/// Returns a vector of icon file paths found in the given directory.
fn search_icons(dir_path: &str) -> Result<Vec<String>, io::Error> {
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

/// Generates the Rust enum text for the given icon paths.
pub fn create_enum_text(paths: &Vec<String>) -> Result<String, io::Error> {
    const START: &str = "pub enum Icon {";
    const START_END_BRACKET: char = '}';
    const MIDDLE: &str = "impl Icon { pub fn path(&self) -> &'static str { match self {";
    const ICON_START: &str = "Icon::";
    const ARROW: &str = " => ";
    const MIDDLE_BRACKET: &str = "}}}";

    let mut enum_content = String::from("");
    enum_content.push_str(START);
    for path in paths {
        if let Some(name) = sanitize_filename(&path) {
            enum_content.push_str(&name);
            enum_content.push(',');
        }
    }
    enum_content.push(START_END_BRACKET);
    enum_content.push_str(MIDDLE);

    for path in paths {
        if let Some(name) = sanitize_filename(path) {
            enum_content.push_str(ICON_START);
            enum_content.push_str(&name);
            enum_content.push_str(ARROW);

            enum_content.push('"');
            enum_content.push_str(&path.replace("\\", "/"));
            enum_content.push('"');
        }

        enum_content.push_str(", ");
    }
    enum_content.push_str(MIDDLE_BRACKET);

    Ok(enum_content)
}

/// Generates the Rust enum text and saves it to the given output file.
pub fn create_enum_file(input_dir: &str, output_file: &str) -> Result<(), io::Error> {
    let paths = search_icons(input_dir)?;
    let enum_text = create_enum_text(&paths)?;
    let mut file = fs::File::create(output_file)?;
    file.write_all(enum_text.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_cli_creates_enum_file() -> Result<(), std::io::Error> {
        use std::fs;

        let input_dir = "assets/icons";
        let output_file = "test_output.rs";

        // Remove any leftover file from previous tests
        let _ = fs::remove_file(output_file);

        // Run the CLI via cargo run
        let status = std::process::Command::new("cargo")
            .args(["run", "--", input_dir, output_file])
            .status()
            .expect("Failed to run CLI");

        // Ensure the CLI exited successfully
        assert!(status.success(), "CLI did not exit successfully");

        // Check that the output file was created
        assert!(
            fs::metadata(output_file).is_ok(),
            "Output file was not created"
        );

        // Optionally check the file content contains at least one enum variant
        let contents = fs::read_to_string(output_file)?;
        assert!(
            contents.contains("Icon"),
            "Output enum does not contain 'Icon'"
        );

        // Clean up the generated file
        // let _ = fs::remove_file(output_file);

        Ok(())
    }

    /// tests enum names from files
    #[test]
    fn test_sanitize_basic_mapping() {
        // Use Some("Expected") for valid icons, None for ignored files
        let test_cases = vec![
            ("ch*eck_box.jpg", Some("CheckBox")),
            ("search-icon@2x.svg", Some("SearchIcon2x")),
            ("home page.png", Some("HomePage")),
            ("settings@dark-mode.svg", Some("SettingsDarkMode")),
            ("123-start.svg", Some("_123Start")),
            ("!weird__name!.svg", Some("WeirdName")),
            ("CAPSLOCK.PNG", Some("CAPSLOCK")),
            ("multi   space  name.jpg", Some("MultiSpaceName")),
            ("icon.v1.2.png", Some("IconV12")),
            ("a.png", Some("A")),
            ("2025.png", Some("_2025")),
            ("LICENSE", None),
            (".gitignore", None),
            ("café-icon.svg", Some("CafeIcon")),
        ];

        for (input, expected) in test_cases {
            let result = sanitize_filename(input);
            println!("{} -> {:?}", input, result);
            assert_eq!(result.as_deref(), expected.map(|s| s));
        }
    }
    /// test removing extensions
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
            println!("Found path: {}", path_str.replace("\\\\", "/"));
        }
    }

    #[test]
    fn test_enum_generation() {
        let paths = search_icons("assets/icons").expect("Failed to read icon directory");
        let enum_file = create_enum_text(&paths).unwrap();
        println!("{}", enum_file);
    }

    #[test]
    fn test_file_save() -> Result<(), io::Error> {
        let _ = create_enum_file("assets/icons", "icon.rs");
        Ok(())
    }
}
