use std::path::PathBuf;

use clap::Parser;

mod collection;
mod color;
mod targets;
mod utils;

#[derive(Parser)]
#[command(
    name = "recol",
    version,
    about = "Change your terminal theme and font easily",
    long_about = r#"Change your terminal theme and font easily

Examples:
    tvibe -t <query> -f <query> # set specific theme and font
    tvibe -rdF                  # set rand dark theme and rand font"#
)]
struct Cli {
    /// Apply theme by name (supports fuzzy matching)
    #[arg(short, long)]
    theme: Option<String>,

    /// Apply a random theme
    #[arg(short, long)]
    rand: bool,

    /// When used with --rand or --theme-list, filters to dark themes
    #[arg(short, long)]
    dark: bool,

    /// Filter to light themes
    #[arg(short, long)]
    light: bool,

    /// List available Nerd Fonts
    #[arg(long)]
    theme_list: bool,

    /// Set font family by name (supports fuzzy matching)
    #[arg(short, long)]
    font: Option<String>,

    /// Pick a random Nerd Font
    #[arg(short = 'F', long)]
    font_rand: bool,

    /// List available Nerd Fonts
    #[arg(long)]
    font_list: bool,

    /// Display the theme's color palette in the terminal without applying it
    #[arg(short, long)]
    show: bool,

    /// TOML format
    #[arg(long)]
    show_toml: bool,

    /// Rust fmt format
    #[arg(long)]
    show_fmt: bool,
    // /// Alacritty config path
    // #[arg(short, long)]
    // alacritty_path: Option<String>,

    // /// Neovim config path
    // #[arg(short, long)]
    // nvim_path: Option<String>,
}

const DEFAULT_NVIM_CONFIG_PATH: &str = ".config/nvim/init.lua";
const DEFAULT_ALACRITTY_CONFIG_PATH: &str = ".config/alacritty/alacritty.toml";

fn al_path() -> PathBuf {
    std::env::home_dir().expect("home dir").join(DEFAULT_ALACRITTY_CONFIG_PATH)
}

fn nvim_path() -> PathBuf {
    std::env::home_dir().expect("home dir").join(DEFAULT_NVIM_CONFIG_PATH)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // #[cfg(debug_assertions)]
    // collection::build::create_colorshemes_bin("colorschemes")?;

    let cli = Cli::parse();
    let col = collection::Collection::new(collection::COLOR_SCHEMES);

    if cli.rand {
        let mut theme = col.rand(None).unwrap().into_theme();
        println!("{}", theme.name);
        targets::alacritty::write_theme_into_config(al_path(), &mut theme)?;
        targets::nvim::write_theme_into_config(nvim_path(), &mut theme)?;
    }

    // println!("{:#?}", theme.into_theme());
    Ok(())
}
