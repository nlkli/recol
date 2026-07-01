use recol_lib as lib;

#[derive(Clone, Debug, Default)]
pub struct Args {
    /// Apply a theme by name (fuzzy matching)
    pub theme: Option<String>,

    /// Apply a random theme
    pub rand: bool,

    /// Filter to dark themes
    pub dark: bool,

    /// Filter to light themes
    pub light: bool,

    /// Filter themes by name substring
    pub contains: Option<String>,

    /// Neovim config path
    // pub nvim_config: Option<String>,

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
    pub json: bool,

    /// Run interactive mode
    pub interactive: bool,
}

// Standard ANSI color codes
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";

const VERSION: &str = "recol 0.1.9 [https://github.com/nlkli/recol]";
fn help() -> String {
    format!(
        r#"
CLI utility for changing the color scheme
{magenta}https://github.com/nlkli/recol{reset}
550+ color schemes:
{magenta}https://github.com/mbadolato/iTerm2-Color-Schemes{reset}

{green}Supported targets:{reset} alacritty, ghostty, wezterm, neovim.

{green}Usage:{reset} {blue}recol [OPTIONS] [THEME_NAME]{reset}

{green}Options:{reset}
  {blue}-t{reset}, {blue}--theme <NAME>{reset}
      Apply a theme by name (fuzzy matching)
  {blue}-r{reset}, {blue}--rand{reset}
      Apply a random theme
  {blue}-d{reset}, {blue}--dark{reset}
  {blue}-l{reset}, {blue}--light{reset}
  {blue}-c{reset}, {blue}--contains <STR>{reset}
      Filter themes by dark, light or name substring
      (used with --rand, --theme or --theme-list)
  {blue}-i{reset}, {blue}--interactive{reset}
      Browse and apply themes interactively
  {blue}-f{reset}, {blue}--font <NAME>{reset}
      Set font family by name (fuzzy matching)
  {blue}-F{reset}, {blue}--font-rand{reset}
      Pick a random Nerd Font
  {blue}--theme-list{reset}  List available themes
  {blue}--font-list{reset}   List available Nerd Fonts
  {blue}-s{reset}, {blue}--show{reset}
      Show the theme color palette without applying it
  {blue}-j{reset}, {blue}--json{reset}    Output theme/list as JSON
  {blue}-h{reset}, {blue}--help{reset}
  {blue}-V{reset}, {blue}--version{reset}
"#,
        reset = RESET,
        green = GREEN,
        blue = BLUE,
        magenta = MAGENTA,
    )
}

impl Args {
    pub fn parse() -> Self {
        let mut args = Self::default();
        let mut last: Option<char> = None;

        let mut iter = std::env::args().skip(1);
        while let Some(arg) = iter.next() {
            if let Some(flag) = arg.strip_prefix("--") {
                match flag {
                    "theme" => last = Some('t'),
                    "font" => last = Some('f'),
                    "contains" => last = Some('c'),
                    // "nvim-config" => last = Some('0'),
                    "theme-list" => args.theme_list = true,
                    "font-list" => args.font_list = true,
                    "font-rand" => args.font_rand = true,
                    "rand" => args.rand = true,
                    "dark" => args.dark = true,
                    "light" => args.light = true,
                    "show" => args.show = true,
                    "json" => args.json = true,
                    "interactive" => args.interactive = true,
                    "help" => {
                        println!("{}", help());
                        std::process::exit(0);
                    }
                    "version" => {
                        println!("{}", VERSION);
                        std::process::exit(0);
                    }
                    _ => (),
                }
            } else if let Some(flags) = arg.strip_prefix('-') {
                for c in flags.chars() {
                    match c {
                        't' | 'f' | 'c' => last = Some(c),
                        'r' => args.rand = true,
                        'd' => args.dark = true,
                        'l' => args.light = true,
                        'F' => args.font_rand = true,
                        's' => args.show = true,
                        'j' => args.json = true,
                        'i' => args.interactive = true,
                        'h' => {
                            println!("{}", help());
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
                match last.take() {
                    Some('t') => {
                        args.theme.replace(arg);
                    }
                    Some('f') => {
                        args.font.replace(arg);
                    }
                    Some('c') => {
                        args.contains.replace(arg);
                    }
                    // Some('0') => {
                    //     args.nvim_config.replace(arg);
                    // }
                    _ => {
                        args.theme.replace(arg);
                    }
                }
            }
        }

        args
    }

    pub fn theme_filters(&self) -> Vec<lib::ThemeFilter<'_>> {
        let mut filters = Vec::new();
        if self.light {
            filters.push(lib::ThemeFilter::Light);
        }
        if self.dark {
            filters.push(lib::ThemeFilter::Dark);
        }
        if let Some(s) = &self.contains {
            filters.push(lib::ThemeFilter::Contains(s));
        }
        filters
    }
}
