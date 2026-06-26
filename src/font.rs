use crate::targets;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn apply_for_targets(f: &str) -> Result<()> {
    if let Some(path) = targets::config_path(targets::Target::Alacritty) {
        targets::alacritty::set_font_into_config(&path, f.into())?;
    }
    if let Some(path) = targets::config_path(targets::Target::Ghostty) {
        targets::ghostty::set_font_into_config(&path, f.into())?;
    }

    Ok(())
}

pub fn list(filter: fn(&str) -> bool) -> Result<Vec<String>> {
    let mut fonts = Vec::new();

    #[cfg(target_os = "macos")]
    {
        let path = std::env::home_dir().unwrap().join("Library/Fonts");
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

    fonts.sort();
    fonts.dedup();

    Ok(fonts)
}
