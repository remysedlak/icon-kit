
#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::icons;
    use crate::icons::remove_extension;
    use crate::icons::sanitize_filename;
    use crate::icons::search_icons;

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
