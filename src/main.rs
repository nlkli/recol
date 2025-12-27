use std::path::PathBuf;

use clap::Parser;
mod cli;
mod collection;
mod color;
mod targets;
mod utils;

const DEFAULT_NVIM_CONFIG_PATH: &str = ".config/nvim/init.lua";
const DEFAULT_ALACRITTY_CONFIG_PATH: &str = ".config/alacritty/alacritty.toml";

fn al_path() -> PathBuf {
    std::env::home_dir()
        .expect("home dir")
        .join(DEFAULT_ALACRITTY_CONFIG_PATH)
}

fn nvim_path() -> PathBuf {
    std::env::home_dir()
        .expect("home dir")
        .join(DEFAULT_NVIM_CONFIG_PATH)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    if args.font_list {}

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
        loop {
            if args.show {
                println!("{}", theme.name);
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
            println!("{}", theme.name);
            let home_dir = std::env::home_dir().unwrap();
            let path = home_dir.join(DEFAULT_ALACRITTY_CONFIG_PATH);
            if path.exists() && path.is_file() {
                targets::alacritty::write_theme_into_config(&path, theme)?;
            }
            let path = home_dir.join(DEFAULT_NVIM_CONFIG_PATH);
            if path.exists() && path.is_file() {
                targets::nvim::write_theme_into_config(&path, theme)?;
            }
            break;
        }
    }

    Ok(())
}
