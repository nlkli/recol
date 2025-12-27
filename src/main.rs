use clap::Parser;
mod cli;
mod collection;
mod color;
mod targets;
mod utils;

const DEFAULT_NVIM_CONFIG_PATH: &str = ".config/nvim/init.lua";
const DEFAULT_ALACRITTY_CONFIG_PATH: &str = ".config/alacritty/alacritty.toml";

type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[inline(always)]
fn home_dir() -> std::path::PathBuf {
    std::env::home_dir().unwrap()
}

fn list_nerd_fonts() -> AnyResult<Vec<String>> {
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

    #[cfg(target_os = "linux")]
    {
        // TODO:
        println!("list_nerd_fonts: not implemented for Linux");
    }

    fonts.sort();
    fonts.dedup();
    Ok(fonts)
}

fn apply_theme_for_targets(_args: &cli::Args, t: &mut collection::Theme) -> AnyResult<()> {
    let home_dir = home_dir();
    let path = home_dir.join(DEFAULT_ALACRITTY_CONFIG_PATH);
    if path.exists() && path.is_file() {
        targets::alacritty::write_theme_into_config(&path, t)?;
    }
    let path = home_dir.join(DEFAULT_NVIM_CONFIG_PATH);
    if path.exists() && path.is_file() {
        targets::nvim::write_theme_into_config(&path, t)?;
    }

    Ok(())
}

fn main() -> AnyResult<()> {
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

    let mut font = None;
    if args.font_rand {
        font = fastrand::choice(&font_list).cloned();
    }
    if let Some(ref query) = args.font {
        font = utils::fuzzy_search_strings(&font_list, query).map(String::from);
    }
    if let Some(font) = font {
        let path = home_dir().join(DEFAULT_ALACRITTY_CONFIG_PATH);
        if path.exists() && path.is_file() {
            targets::alacritty::set_font_into_config(&path, font)?;
        }
    }

    let mut theme = None;
    if args.rand {
        theme.replace(col.rand(Some(&filter)).unwrap().into_theme());
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
            if args.show_toml {
                let toml_str = toml::to_string_pretty(theme)?;
                println!("{}", toml_str);
                break;
            }
            if args.show_fmt {
                println!("{:#?}", theme);
                break;
            }
            print_header();
            apply_theme_for_targets(&args, theme)?;
            break;
        }
    }

    Ok(())
}
