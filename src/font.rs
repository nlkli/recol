type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn list(filter: fn(&str) -> bool) -> Result<Vec<String>> {
    let mut fonts = Vec::new();

    #[cfg(target_os = "macos")]
    {
        let Some(path) = std::env::home_dir().map(|d| d.join("Library/Fonts")) else {
            return Ok(fonts);
        };
        for entry in std::fs::read_dir(path)? {
            let path = entry?.path();
            if !path.is_file() {
                continue;
            }
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if let Some((name, _)) = file_name.split_once('-') {
                    if !filter(name) {
                        continue;
                    }
                    if name.contains("NerdFont") {
                        let name = name
                            .replace("NerdFont", " Nerd Font ")
                            .replace("  ", " ")
                            .trim()
                            .to_string();
                        fonts.push(name);
                    }
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        for dir in font_search_dirs() {
            collect_fonts(&dir, filter, &mut fonts);
        }
    }

    fonts.sort();
    fonts.dedup();
    Ok(fonts)
}

/// Builds the list of font directories per the XDG Base Directory spec,
/// falling back to the conventional defaults when the env vars are unset.
#[cfg(target_os = "linux")]
fn font_search_dirs() -> Vec<std::path::PathBuf> {
    let mut dirs = Vec::new();

    // $XDG_DATA_HOME/fonts, defaulting to ~/.local/share/fonts
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::home_dir().map(|h| h.join(".local/share")));
    if let Some(dir) = data_home {
        dirs.push(dir.join("fonts"));
    }

    // $XDG_DATA_DIRS/fonts (colon-separated), defaulting to
    // /usr/local/share:/usr/share
    let data_dirs = std::env::var("XDG_DATA_DIRS")
        .unwrap_or_else(|_| "/usr/local/share:/usr/share".to_string());
    for base in data_dirs.split(':').filter(|s| !s.is_empty()) {
        dirs.push(std::path::PathBuf::from(base).join("fonts"));
    }

    // Legacy path still respected by some tools and older installs.
    if let Some(home) = std::env::home_dir() {
        dirs.push(home.join(".fonts"));
    }

    dirs
}

/// Recursively walks `dir` collecting Nerd Font family names that match
/// `filter`. Missing or unreadable directories/entries are silently
/// skipped, since several standard font locations may not exist.
#[cfg(target_os = "linux")]
fn collect_fonts(dir: &std::path::Path, filter: fn(&str) -> bool, fonts: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            collect_fonts(&path, filter, fonts);
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        let Some((name, _)) = file_name.split_once('-') else {
            continue;
        };

        if !filter(name) {
            continue;
        }

        if name.contains("NerdFont") {
            let name = name
                .replace("NerdFont", " Nerd Font ")
                .replace("  ", " ")
                .trim()
                .to_string();
            fonts.push(name);
        }
    }
}
