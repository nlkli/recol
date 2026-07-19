use recol_lib::{self as lib, ThemeAdjustment};

use crate::targets::Target;

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

    pub adjust: Vec<ThemeAdjustment>,

    /// Neovim config path
    pub nvim_config: Option<String>,

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

    /// Apply for specific target
    pub targets: Vec<Target>,

    /// Run interactive mode
    pub interactive: bool,

    /// Interactive mode flag
    pub quit_on_select: bool,

    /// Interactive mode flag
    pub init_input: bool,

    /// Interactive mode flag
    pub init_help: bool,
}

// Standard ANSI color codes
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";

const VERSION: &str = "recol 0.2.1 [https://github.com/nlkli/recol]";
fn help() -> String {
    format!(
        r#"CLI utility for changing the color scheme
{magenta}https://github.com/nlkli/recol{reset}
590+ color schemes:
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
  {blue}-a{reset}, {blue}--adjust <SPEC|PATH>{reset} [env: RECOL_ADJUST]
      Apply color adjustments (see --adjust help)
      Format: "group.adjustment=value,..."
  {blue}-i{reset}, {blue}--interactive{reset}
      Browse and apply themes interactively
  {blue}-f{reset}, {blue}--font <NAME>{reset}
      Set font family by name (fuzzy matching)
  {blue}-F{reset}, {blue}--font-rand{reset}
      Pick a random Nerd Font
  {blue}-T{reset}, {blue}--target <TARGET>{reset}
      Apply for specific target
  {blue}--theme-list{reset}  List available themes
  {blue}--font-list{reset}   List available Nerd Fonts
  {blue}-s{reset}, {blue}--show{reset}
      Show the theme color palette without applying it
  {blue}-j{reset}, {blue}--json{reset}    Output theme/list as JSON
  {blue}-h{reset}, {blue}--help{reset}
  {blue}-V{reset}, {blue}--version{reset}"#,
        reset = RESET,
        green = GREEN,
        blue = BLUE,
        magenta = MAGENTA,
    )
}
fn adjust_help() -> String {
    format!(
        r#"Color adjustments: {blue}--adjust "group.adjustment=value,..."{reset}
  Apply one or more transformations to theme colors.

{green}Quick start:{reset}
  {blue}--adjust "brightness=-10"{reset}  Darken whole theme slightly
  {blue}--adjust "saturation=20"{reset}   Boost all colors
  {blue}--adjust "pal.hue=180"{reset}     Shift palette to complementary hues
  {blue}--adjust "bg.exposure=-15,fg.contrast=10"{reset}  Darker bg, punchier text
  {blue}--adjust "blue.hue=30,saturation=-50"{reset}      Turn blues into muted teals
  {blue}--adjust "sel-bg.brightness=20,cursor.sat=50"{reset} Bright sel bg, vivid cursor

{green}Groups (optional, defaults to All):{reset}
  {blue}u/ui{reset}          All UI colors
  {blue}b/bg{reset}         Backgrounds (base, sel, cursor)
  {blue}f/fg{reset}         Foregrounds (base, sel, cursor)
  {blue}s/sel{reset}        Selection colors
  {blue}c/cur{reset}        Cursor colors
  {blue}bb/base-bg{reset}   Base background only
  {blue}bf/base-fg{reset}   Base foreground only
  {blue}sb/sel-bg{reset}    Selection background
  {blue}sf/sel-fg{reset}    Selection foreground
  {blue}cb/cur-bg{reset}    Cursor background
  {blue}cf/cur-fg{reset}    Cursor foreground
  {blue}p/pal{reset}        All ANSI palette colors
  {blue}t/text{reset}       Foregrounds + palette
  {blue}black{reset}        Black (normal + bright)
  {blue}red{reset}          Red (normal + bright)
  {blue}green{reset}        Green (normal + bright)
  {blue}yellow{reset}       Yellow (normal + bright)
  {blue}blue{reset}         Blue (normal + bright)
  {blue}magenta{reset}      Magenta (normal + bright)
  {blue}cyan{reset}         Cyan (normal + bright)
  {blue}white{reset}        White (normal + bright)
  {blue}orange{reset}       Orange (normal + bright)
  {blue}pink{reset}         Pink (normal + bright)

{green}Adjustments:{reset}
  {blue}b/br/brightness{reset}=N     HSL lightness shift (-100..100)
  {blue}e/exposure{reset}=N          Linear-light scale (-100..100, ±1 stop)
  {blue}c/contrast{reset}=N          HSL contrast (-100..100)
  {blue}cc/channel-contrast{reset}=N RGB channel contrast (-100..100)
  {blue}s/sat/saturation{reset}=N    HSV saturation (-100..100)
  {blue}v/vib/vibrance{reset}=N      Smart saturation (-100..100)
  {blue}h/hue{reset}=N               Hue rotation (-180..180°)
  {blue}t/temp/temperature{reset}=N  Blue↔Orange white balance (-100..100)
  {blue}ti/tint{reset}=N             Green↔Magenta white balance (-100..100)
  {blue}g/gamma{reset}=N             Gamma correction (0.25..4.0)
  {blue}bp/black-point{reset}=N      Lift shadows (-100..100)
  {blue}wp/white-point{reset}=N      Crush highlights (-100..100)
  {blue}i/invert{reset}=1            Invert HSL lightness (value ignored)

{green}More examples:{reset}
  {blue}--adjust "temperature=40,tint=-10"{reset}  Warm amber tint
  {blue}--adjust "pal.gamma=0.8,black.brightness=5"{reset}  Softer palette, lifted blacks
  {blue}--adjust "red.hue=-20,saturation=30,temperature=60"{reset}  Rich warm reds
  {blue}--adjust "preset.txt"{reset}  Load adjustments from file
  {blue}--adjust "_"{reset}           Reset all adjustments"#,
        reset = RESET,
        green = GREEN,
        blue = BLUE,
    )
}

impl Args {
    pub fn parse() -> Self {
        let mut args = Self::default();
        let mut last: Option<char> = None;

        if let Some(arg) = std::env::var("RECOL_ADJUST").ok() {
            args.adjust_arg(arg);
        }

        let mut iter = std::env::args().skip(1);
        while let Some(arg) = iter.next() {
            if let Some(flag) = arg.strip_prefix("--") {
                match flag {
                    "theme" => last = Some('t'),
                    "font" => last = Some('f'),
                    "contains" => last = Some('c'),
                    "target" => last = Some('c'),
                    "nvim_config" => last = Some('0'),
                    "adjust" => last = Some('a'),
                    "theme-list" => args.theme_list = true,
                    "font-list" => args.font_list = true,
                    "font-rand" => args.font_rand = true,
                    "rand" => args.rand = true,
                    "dark" => args.dark = true,
                    "light" => args.light = true,
                    "show" => args.show = true,
                    "json" => args.json = true,
                    "interactive" => args.interactive = true,
                    "quit-on-select" => args.quit_on_select = true,
                    "init-input" => args.init_input = true,
                    "init-help" => args.init_help = true,
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
                        't' | 'f' | 'c' | 'T' | 'a' => last = Some(c),
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
                    Some('0') => {
                        args.nvim_config.replace(arg);
                    }
                    Some('T') => {
                        if let Ok(t) = arg.parse::<Target>() {
                            if !args.targets.contains(&t) {
                                args.targets.push(t);
                            }
                        }
                    }
                    Some('a') => {
                        args.adjust_arg(arg);
                    }
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

    fn adjust_arg(&mut self, mut arg: String) {
        if arg == "help" {
            println!("{}", adjust_help());
            std::process::exit(0);
        }
        let path = std::path::PathBuf::from(&arg);
        if path.is_file() {
            arg = std::fs::read_to_string(path).expect("read adjust file error");
        }
        if arg == "_" {
            self.adjust.clear();
            self.adjust.push(ThemeAdjustment::None);
            return;
        }
        for adjust_str in arg.split(",") {
            let trimmed = adjust_str.trim();
            match trimmed.parse::<ThemeAdjustment>() {
                Ok(a) => {
                    self.adjust.push(a);
                }
                Err(e) => panic!("{}", e),
            }
        }
    }
}
