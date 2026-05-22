#[derive(Clone, Debug, Default)]
pub struct Args {
    /// Apply a theme by name (fuzzy matching)
    pub theme: Option<String>,

    /// Apply a random theme
    pub rand: bool,

    /// Filter to dark themes (used with --rand or --theme-list)
    pub dark: bool,

    /// Filter to light themes
    pub light: bool,

    /// List available themes
    pub theme_list: bool,

    /// Set font family by name (fuzzy matching)
    pub font: Option<String>,

    /// Pick a random Nerd Font
    pub font_rand: bool,

    /// List available Nerd Fonts
    pub font_list: bool,

    /// Show the theme color palette without applying it
    pub show: bool,

    /// Output theme as JSON
    pub show_json: bool,

    /// Output theme as TOML
    pub show_toml: bool,

    /// Output theme in rustfmt-style format
    pub show_fmt: bool,

    /// Alacritty config path
    pub alacritty_config: Option<String>,

    /// Neovim config path
    pub nvim_config: Option<String>,
}

const VERSION: &str = "recol 0.1.5 [https://github.com/nlkli/recol]";
const HELP: &str = r#"
recol — quickly change your terminal theme
https://github.com/nlkli/recol
500+ terminal color schemes:
https://github.com/mbadolato/iTerm2-Color-Schemes
Supported targets: alacritty, neovim.

  recol <TNAME> -f <FNAME> # set a specific theme and font (fuzzy match)
  recol -rdF               # random dark theme and random Nerd Font
  recol -rls               # show random light theme palette

Options:
  [ ], -t, --theme <NAME>
      Apply a theme by name (fuzzy matching)
  -r, --rand
      Apply a random theme
  -d, --dark
  -l, --light
      Filter to dark or light themes 
      (used with --rand, --theme or --theme-list)

  --alacritty_config <PATH>
      default: ~/.config/alacritty/alacritty.toml
  --nvim_config <PATH>
      default: ~/.config/nvim/init.lua

  -f, --font <NAME>
      Set font family by name (fuzzy matching)
  -F, --font-rand   
      Pick a random Nerd Font

  --theme-list  List available themes
  --font-list   List available Nerd Fonts

  -s, --show
    Show the theme color palette without applying it
      --show-json   Output theme as JSON
      --show-toml   Output theme as TOML
      --show-fmt    Output theme in rustfmt-style format

  -h, --help
  -V, --version
"#;

impl Args {
    pub fn parse() -> Self {
        let mut args = Self::default();
        let input = std::env::args();
        let mut last = None;
        for i in input.skip(1) {
            if i.starts_with("--") {
                let key = i.trim_start_matches("--");
                match key {
                    "theme" => {
                        last.replace('t');
                    }
                    "font" => {
                        last.replace('f');
                    }
                    "alacritty_config" => {
                        last.replace('0');
                    }
                    "nvim_config" => {
                        last.replace('1');
                    }
                    "theme-list" => args.theme_list = true,
                    "font-list" => args.font_list = true,
                    "font-rand" => args.font_rand = true,
                    "rand" => args.rand = true,
                    "dark" => args.dark = true,
                    "light" => args.light = true,
                    "show" => args.show = true,
                    "show-json" => args.show_json = true,
                    "show-toml" => args.show_toml = true,
                    "show-fmt" => args.show_fmt = true,
                    "help" => {
                        println!("{}", HELP);
                        std::process::exit(0);
                    }
                    "version" => {
                        println!("{}", VERSION);
                        std::process::exit(0);
                    }
                    _ => (),
                }
            } else if i.starts_with("-") {
                let chars = i.trim_start_matches("-").chars();
                for c in chars {
                    match c {
                        't' => {
                            last.replace(c);
                        }
                        'f' => {
                            last.replace(c);
                        }
                        'T' => {
                            last.replace(c);
                        }
                        'r' => args.rand = true,
                        'd' => args.dark = true,
                        'l' => args.light = true,
                        'F' => args.font_rand = true,
                        's' => args.show = true,
                        'h' => {
                            println!("{}", HELP);
                            std::process::exit(0);
                        }
                        'V' => {
                            println!("{}", VERSION);
                            std::process::exit(0);
                        }
                        _ => (),
                    }
                }
            } else {
                if let Some(c) = last {
                    match c {
                        't' => {
                            args.theme.replace(i);
                        }
                        'f' => {
                            args.font.replace(i);
                        }
                        '0' => {
                            args.alacritty_config.replace(i);
                        }
                        '1' => {
                            args.nvim_config.replace(i);
                        }
                        _ => (),
                    }
                    last = None;
                } else {
                    args.theme.replace(i);
                }
            }
        }
        args
    }
}
