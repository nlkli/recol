mod collection;
mod color;
mod models;
mod templ;
use clap::Parser;
use rand::seq::IndexedRandom;
use std::{sync::Once, path::{Path, PathBuf}, fmt::Write, io::BufRead, process::exit};
use strsim::levenshtein;

static INIT: Once = Once::new();
const DEFAULT_NVIM_CONFIG_PATH: &str = ".config/nvim/init.lua";
const DEFAULT_ALACRITTY_CONFIG_PATH: &str = ".config/alacritty/alacritty.toml";

fn supports_truecolor() -> bool {
    matches!(
        std::env::var("COLORTERM").as_deref(),
        Ok("truecolor") | Ok("24bit")
    )
}

#[inline(always)]
fn home_dir() -> PathBuf {
    std::env::home_dir().expect("home_dir")
}

static mut NVIM_CONFIG_PATH: Option<PathBuf> = None;
static mut ALACRITTY_CONFIG_PATH: Option<PathBuf> = None;

fn init(cli: Option<&Cli>) {
    // TODO: cli
    unsafe {
        INIT.call_once(|| {
            let _cli = cli.expect("shoud be some");
            let home_dir = home_dir();
            NVIM_CONFIG_PATH = Some(home_dir.join(DEFAULT_NVIM_CONFIG_PATH));
            ALACRITTY_CONFIG_PATH = Some(home_dir.join(DEFAULT_ALACRITTY_CONFIG_PATH));
        })
    }
}

fn get_nvim_config_path() -> &'static PathBuf {
    unsafe {
        NVIM_CONFIG_PATH.as_ref().expect("shoud be init")
    }
}

fn get_alacritty_config_path() -> &'static PathBuf {
    unsafe {
        ALACRITTY_CONFIG_PATH.as_ref().expect("shoud be init")
    }
}

fn apply_theme_to_nvim(theme: &mut models::Theme) -> Result<(), Box<dyn std::error::Error>> {
    const START_MARK: &str = "-- ====THEMESYNCSTARTBLOCK====";
    const END_MARK: &str = "-- ====THEMESYNCENDBLOCK====";

    let content = templ::nvim(theme);
    let path = get_nvim_config_path();

    let file = std::fs::File::open(&path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut buf = String::new();
    let mut lines = reader.lines();
    let mut inserted = false;

    while let Some(line) = lines.next() {
        let line = line?;
        writeln!(&mut buf, "{}", &line)?;
        if line == START_MARK {
            writeln!(&mut buf, "{}", &content)?;
            inserted = true;
            break;
        }
    }
    if inserted {
        let mut replace_buf = String::new();
        let mut found_end = false;
        while let Some(line) = lines.next() {
            let line = line?;
            writeln!(&mut replace_buf, "{}", &line)?;
            if line == END_MARK {
                found_end = true;
                break;
            }
        }
        if found_end {
            writeln!(&mut buf, "{}", &END_MARK)?;
        } else {
            writeln!(&mut buf, "{}", &replace_buf)?;
        }
        while let Some(line) = lines.next() {
            let line = line?;
            writeln!(&mut buf, "{}", &line)?;
        }
    } else {
        writeln!(&mut buf, "\n{START_MARK}\n{content}\n{END_MARK}")?;
    }

    std::fs::write(&path, &buf)?;
    Ok(())
}

#[inline(always)]
fn load_alacritty_config() -> Result<models::alacritty::Config, Box<dyn std::error::Error>> {
    let buff = std::fs::read_to_string(get_alacritty_config_path())?;
    Ok(toml::from_str::<models::alacritty::Config>(&buff)?)
}

#[inline(always)]
fn save_alacritty_config(
    config: &models::alacritty::Config,
) -> Result<models::alacritty::Config, Box<dyn std::error::Error>> {
    let buff = toml::to_string_pretty(&config)?;
    std::fs::write(get_alacritty_config_path(), &buff)?;
    Ok(toml::from_str::<models::alacritty::Config>(&buff)?)
}

fn apply_theme_to_alacritty(theme: &mut models::Theme) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_alacritty_config()?;
    config.replace_colors_from_theme(theme.get_or_insert_colors());
    save_alacritty_config(&config)?;
    Ok(())
}

fn list_nerd_fonts() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut fonts = Vec::new();

    #[cfg(target_os = "macos")]
    {
        let path = home_path_join("Library/Fonts");
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
                            .replace(" ", " ")
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

fn set_alacritty_font(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fonts = list_nerd_fonts()?;
    if fonts.is_empty() {
        return Err("No Nerd Fonts found on your system".into());
    }

    let query = query.to_lowercase();
    let font = fonts
        .iter()
        .min_by_key(|v| levenshtein(&v.to_lowercase(), &query))
        .ok_or_else(|| format!("No matching font found for query '{}'", query))?;

    let path = home_path_join(DEFAULT_ALACRITTY_CONFIG_PATH);
    let mut config = load_alacritty_config(&path)?;
    config.set_font_family(font);
    save_alacritty_config(path, &config)?;

    Ok(())
}

#[derive(Parser)]
#[command(
    name = "tvibe",
    version,
    about = "Change your terminal theme and font easily.",
    long_about = r#"Examples:
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut is_error = false;
    if !supports_truecolor() {
        println!("Warning: Your terminal does not fully support truecolor");
    }
    let cli = Cli::parse();

    if cli.theme_list {
        if cli.dark {
            collection::DARK_LIST.iter().for_each(|i| println!("{i}"));
        } else if cli.light {
            collection::LIGHT_LIST.iter().for_each(|i| println!("{i}"));
        } else {
            collection::LIST.iter().for_each(|i| println!("{i}"));
        }
    }
    if cli.font_list {
        list_nerd_fonts()?.iter().for_each(|i| println!("{i}"));
    }
    if let Some(query) = cli.font {
        if let Err(e) = set_alacritty_font(&query) {
            is_error = true;
            eprintln!("{}", e);
        }
    } else if cli.font_rand {
        let res = set_alacritty_font(
            list_nerd_fonts()?
                .choose(&mut rand::rng())
                .unwrap_or(&"".into()),
        );
        if let Err(e) = res {
            is_error = true;
            eprintln!("{}", e);
        }
    }
    let theme = if let Some(query) = cli.theme {
        Some(collection::search(&query))
    } else if cli.rand {
        if cli.dark {
            Some(collection::rand_dark())
        } else if cli.light {
            Some(collection::rand_light())
        } else {
            Some(collection::rand())
        }
    } else {
        None
    };
    if let Some(mut theme) = theme {
        println!("{}", theme.name.clone().unwrap_or("unknown".into()));
        theme.prepare()?;
        // theme.validation()?;
        if cli.show {
            let base_colors = theme.get_or_insert_colors().base.to_vec_colors()?;
            color::print_palette(&base_colors);
            toml::to_string_pretty(&theme).unwrap();
        }
        if cli.show_toml {
            println!("{}", toml::to_string_pretty(&theme)?);
        }
        if cli.show_fmt {
            println!("{:#?}", theme);
        }
        if !cli.show && !cli.show_toml && !cli.show_fmt {
            if 
            if let Err(e) = apply_theme_to_nvim(&mut theme) {
                is_error = true;
                eprintln!("{}", e);
            }
            if let Err(e) = apply_theme_to_alacritty(&mut theme) {
                is_error = true;
                eprintln!("{}", e);
            }
        }
    }
    if is_error {
        exit(1);
    }

    Ok(())
}
