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
    /// Apply a theme by name (fuzzy matching)
    #[arg(short, long)]
    pub theme: Option<String>,

    /// Apply a random theme
    #[arg(short, long)]
    pub rand: bool,

    /// Filter to dark themes (used with --rand or --theme-list)
    #[arg(short, long)]
    pub dark: bool,

    /// Filter to light themes
    #[arg(short, long)]
    pub light: bool,

    /// List available themes
    #[arg(long)]
    pub theme_list: bool,

    /// Set font family by name (fuzzy matching)
    #[arg(short, long)]
    pub font: Option<String>,

    /// Pick a random Nerd Font
    #[arg(short = 'F', long)]
    pub font_rand: bool,

    /// List available Nerd Fonts
    #[arg(long)]
    pub font_list: bool,

    /// Show the theme color palette without applying it
    #[arg(short, long)]
    pub show: bool,

    /// Output theme as TOML
    #[arg(long)]
    pub show_toml: bool,

    /// Output theme in rustfmt-style format
    #[arg(long)]
    pub show_fmt: bool,

    // /// Alacritty config path
    // #[arg(short, long)]
    // alacritty_path: Option<String>,

    // /// Neovim config path
    // #[arg(short, long)]
    // nvim_path: Option<String>,
}
