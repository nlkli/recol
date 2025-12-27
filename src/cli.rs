
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

    /// Output theme as TOML
    pub show_toml: bool,

    /// Output theme in rustfmt-style format
    pub show_fmt: bool,
}

const VERSION: &str = "recol 0.1.4";
const HELP: &str = r#"Quickly change your terminal theme.
Over 425 terminal colorschemes.
https://github.com/mbadolato/iTerm2-Color-Schemes
Supported targets: Alacritty, Neovim.

  tvibe <TNAME> -f <FNAME> # set a specific theme and font (fuzzy match)
  tvibe -rdF               # random dark theme and random Nerd Font
  tvibe -rls               # show random light theme palette

Options:
  [ ], -t, --theme <NAME>
          Apply a theme by name (fuzzy matching)
  -r, --rand
          Apply a random theme
  -d, --dark
    Filter to dark themes (used with --rand, --theme or --theme-list)
  -l, --light   Filter to light themes

  -f, --font <NAME>
          Set font family by name (fuzzy matching)
      -F, --font-rand   Pick a random Nerd Font

  --theme-list  List available themes
  --font-list   List available Nerd Fonts

  -s, --show
    Show the theme color palette without applying it
      --show-toml   Output theme as TOML
      --show-fmt    Output theme in rustfmt-style format

  -h, --help    Print help
  -V, --version Print version"#;

impl Args {
    pub fn parse() -> Self {
        let mut args = Self::default();
        let input = std::env::args();
        let mut last = None;
        for i in input.skip(1) {
            if i.starts_with("--") {
                let key = i.trim_start_matches("--");
                match key {
                    "theme" => { last.replace('t'); },
                    "font" => { last.replace('f'); },
                    "theme-list" => args.theme_list = true,
                    "font-list" => args.font_list = true,
                    "font-rand" => args.font_rand = true,
                    "rand" => args.rand = true,
                    "dark" => args.dark = true,
                    "light" => args.light = true,
                    "show" => args.show = true,
                    "show-toml" => args.show_toml = true,
                    "show-fmt" => args.show_fmt = true,
                    "help" => {
                        println!("{}", HELP);
                        std::process::exit(0);
                    },
                    "version" => {
                        println!("{}", VERSION);
                        std::process::exit(0);
                    },
                    _ => (),
                }
            } else if i.starts_with("-") {
                let chars = i.trim_start_matches("-").chars();
                for c in chars {
                    match c {
                        't' => { last.replace(c); },
                        'f' => { last.replace(c); },
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
                        't' => { args.theme.replace(i); },
                        'f' => { args.font.replace(i); },
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
