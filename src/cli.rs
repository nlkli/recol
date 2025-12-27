use clap::Parser;

#[derive(Parser)]
#[command(
    name = "recol",
    version,
    about = "Quickly change your terminal theme",
    long_about = r#"Quickly change your terminal theme.
Supported targets:
  - Alacritty
  - Neovim
Examples:
    tvibe -t <query> -f <query> # set a specific theme and font (fuzzy match)
    tvibe -rdF                  # random dark theme and random Nerd Font"#
)]
pub struct Args {
    /// Apply theme by name (supports fuzzy matching)
    #[arg(short, long)]
    pub theme: Option<String>,

    /// Apply a random theme
    #[arg(short, long)]
    pub rand: bool,

    /// When used with --rand or --theme-list, filters to dark themes
    #[arg(short, long)]
    pub dark: bool,

    /// Filter to light themes
    #[arg(short, long)]
    pub light: bool,

    /// List of available colorschemes
    #[arg(long)]
    pub theme_list: bool,

    /// Set font family by name (supports fuzzy matching)
    #[arg(short, long)]
    pub font: Option<String>,

    /// Pick a random Nerd Font
    #[arg(short = 'F', long)]
    pub font_rand: bool,

    /// List of available Nerd Fonts
    #[arg(long)]
    pub font_list: bool,

    /// Display the theme's color palette in the terminal without applying it
    #[arg(short, long)]
    pub show: bool,

    /// TOML format
    #[arg(long)]
    pub show_toml: bool,

    /// Rust fmt format
    #[arg(long)]
    pub show_fmt: bool,
    // /// Alacritty config path
    // #[arg(short, long)]
    // alacritty_path: Option<String>,

    // /// Neovim config path
    // #[arg(short, long)]
    // nvim_path: Option<String>,
}
