mod cli;
mod collection;
mod color;
mod targets;
mod tmpstore;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
#[inline(always)]
fn home_dir() -> std::path::PathBuf {
    std::env::home_dir().unwrap()
}

fn list_nerd_fonts() -> Result<Vec<String>> {
    let mut fonts = Vec::new();

    #[cfg(target_os = "macos")]
    {
        let path = home_dir().join("Library/Fonts");
        for entry in std::fs::read_dir(path)? {
            let path = entry?.path();
            if !path.is_file() {
                continue;
            }

            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if let Some((name, _)) = file_name.split_once('-') {
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

    // #[cfg(target_os = "linux")]
    // {
    //     // TODO:
    //     println!("list_nerd_fonts: not implemented for Linux");
    // }

    fonts.sort();
    fonts.dedup();
    Ok(fonts)
}

fn apply_theme_for_targets(args: &cli::Args, t: &mut collection::Theme) -> Result<()> {
    if let Some(path) = targets::config_path(targets::Target::Alacritty) {
        targets::alacritty::write_theme_into_config(&path, t)?;
    }
    if let Some(path) = targets::config_path(targets::Target::Ghostty) {
        targets::ghostty::write_theme_into_config(&path, t)?;
    }
    if let Some(ref path) = args.nvim_config {
        let path = std::path::PathBuf::from(path);
        if path.exists() && path.is_file() {
            targets::nvim::write_theme_into_config(&path, t)?;
        }
    } else {
        if let Some(path) = targets::config_path(targets::Target::Nvim) {
            targets::nvim::write_theme_into_config(&path, t)?;
        }
    }

    Ok(())
}

fn apply_font_for_targets(f: &str) -> Result<()> {
    if let Some(path) = targets::config_path(targets::Target::Alacritty) {
        targets::alacritty::set_font_into_config(&path, f.into())?;
    }
    if let Some(path) = targets::config_path(targets::Target::Ghostty) {
        targets::ghostty::set_font_into_config(&path, f.into())?;
    }

    Ok(())
}

fn main() -> Result<()> {
    // #[cfg(debug_assertions)]
    // collection::build::create_colorshemes_bin("colorschemes")?;

    let args = cli::Args::parse();
    let col = collection::Collection::new(collection::COLOR_SCHEMES);

    let mut filter = collection::LazyThemeFilter::default();

    if args.light {
        filter = collection::LazyThemeFilter::Light;
    }
    if args.dark {
        filter = collection::LazyThemeFilter::Dark;
    }
    if args.theme_list {
        for name in col.name_list(Some(&filter), true) {
            println!("{}", name);
        }
    }

    let mut font_list = Vec::new();
    if args.font_list || args.font_rand || args.font.is_some() {
        font_list = list_nerd_fonts().unwrap_or_default();
    }
    if args.font_list {
        for f in &font_list {
            println!("{f}");
        }
    }

    tmpstore::init();

    let mut font = None;
    if args.font_rand {
        let font_history = tmpstore::read_font_history(2);
        font = fastrand::choice(&font_list).cloned();
        let mut n: isize = 0;
        while n < 5 {
            if let Some(ref f) = font {
                if font_history.contains(f) {
                    font = fastrand::choice(&font_list).cloned();
                    n += 1;
                    continue;
                }
            }
            break;
        }
    }
    if let Some(ref query) = args.font {
        font = utils::fuzzy_search_strings(&font_list, query).map(String::from);
    }
    if let Some(ref font) = font {
        apply_font_for_targets(font)?;
        tmpstore::append_font_history(font);
    }

    let mut theme = None;
    if args.rand {
        let theme_history = tmpstore::read_theme_history(21);
        theme.replace(col.rand(Some(&filter)).unwrap().into_theme());
        let mut n: isize = 0;
        while n < 9 {
            if let Some(ref t) = theme {
                if theme_history.contains(&t.name) {
                    theme.replace(col.rand(Some(&filter)).unwrap().into_theme());
                    n += 1;
                    continue;
                }
            }
            break;
        }
    }
    if let Some(ref query) = args.theme {
        theme.replace(
            col.fuzzy_search(query, Some(&filter))
                .unwrap_or(col.rand(None).unwrap())
                .into_theme(),
        );
    }
    if let Some(ref mut theme) = theme {
        let print_header = || {
            let tmod = if theme.is_light { "LIGHT" } else { "DARK" };
            println!("{} <{tmod}>", theme.name);
        };
        loop {
            if args.show {
                print_header();
                theme.print_palette();
                break;
            }
            if args.show_json {
                let json_str = serde_json::to_string_pretty(theme.prepare(None, None, None))?;
                println!("{}", json_str);
                break;
            }
            if args.show_toml {
                let toml_str = toml::to_string_pretty(theme.prepare(None, None, None))?;
                println!("{}", toml_str);
                break;
            }
            print_header();
            apply_theme_for_targets(&args, theme)?;
            tmpstore::append_theme_history(&theme.name);
            break;
        }
    }

    Ok(())
}
