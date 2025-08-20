
#[cfg(test)]
mod tests {
    use crate::icons;
    use crate::icons::remove_extension;
    use crate::icons::sanitize_filename;
    use crate::icons::search_icons;
    use std::path::Path;

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
        let enum_file = icons::create_enum_text(&paths).unwrap();
        println!("{}", enum_file);
    }
}
