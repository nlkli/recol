mod utils;
mod collection;
mod color;
mod models;
mod templ;
mod converter;
use clap::Parser;
use rand::seq::IndexedRandom;
use std::io::Write as IoWrite;
use std::{fmt::Write, fs::OpenOptions, io::BufRead, path::PathBuf, process::exit};
use strsim::levenshtein;

const DEFAULT_NVIM_CONFIG_PATH: &str = ".config/nvim/init.lua";
const DEFAULT_ALACRITTY_CONFIG_PATH: &str = ".config/alacritty/alacritty.toml";

fn supports_truecolor() -> bool {
    matches!(
        std::env::var("COLORTERM").as_deref(),
        Ok("truecolor") | Ok("24bit")
    )
}

use color::Color;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TermColors {
    black: String,
    blue: String,
    cyan: String,
    green: String,
    magenta: String,
    red: String,
    white: String,
    yellow: String,
}

impl TermColors {
    fn as_colors_arr(&self) -> Result<[Color; 8], String> {
        Ok([
            Color::from_hex_str(&self.black)?,
            Color::from_hex_str(&self.blue)?,
            Color::from_hex_str(&self.cyan)?,
            Color::from_hex_str(&self.green)?,
            Color::from_hex_str(&self.magenta)?,
            Color::from_hex_str(&self.red)?,
            Color::from_hex_str(&self.white)?,
            Color::from_hex_str(&self.yellow)?,
        ])
    }
}

#[derive(Deserialize, Debug)]
pub struct Cursor {
    cursor: String,
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct Primary {
    background: String,
    foreground: String,
}

#[derive(Deserialize, Debug)]
pub struct Selection {
    background: String,
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct Colors {
    primary: Primary,
    normal: TermColors,
    bright: TermColors,
    cursor: Cursor,
    selection: Selection,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    colors: Colors,
}

fn xxx() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = vec![];
    for d in std::fs::read_dir("themes")? {
        files.push(d?.file_name().into_string().unwrap());
    }
    files.sort();
    let mut names_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("names.txt")?;
    let mut themes_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("themes.bin")?;
    for f in files {
        let name = f.trim_end_matches(".toml");
        let tf = std::fs::read_to_string(format!("themes/{name}.toml"))?;
        let theme = toml::from_str::<Theme>(&tf)?;
        let c = theme.colors;
        let nc = c.normal.as_colors_arr()?;
        let mut data = vec![];
        for i in nc {
            let (r, g, b, _) = i.to_rgba();
            data.extend_from_slice(&[r,g,b]);
        }
        themes_file.write_all(&data)?;
        writeln!(&mut names_file, "{}", name)?;
    }

    Ok(())
}

fn gent() -> Result<(), Box<dyn std::error::Error>> {
    xxx()?;
    const SHADEF: f32 = 0.15;
    const BGS: [f32; 4] = [-4., 6., 12., 23.];
    const FGS: [f32; 3] = [6., -23., -45.];
    const SELS: f32 = 16.;
    let mut themes = vec![];
    for d in std::fs::read_dir("themes")? {
        themes.push(d?.path());
    }
    loop {
        let theme_path = themes.choose(&mut rand::rng());
        println!("{:?}", theme_path);
        let toml_string = std::fs::read_to_string(theme_path.unwrap())?;
        let theme = toml::from_str::<Theme>(&toml_string)?;
        let c = theme.colors;

        let dim = TermColors {
            black: Color::from_hex_str(&c.normal.black)?
                .shade(-SHADEF)
                .to_css(false),
            blue: Color::from_hex_str(&c.normal.blue)?
                .shade(-SHADEF)
                .to_css(false),
            cyan: Color::from_hex_str(&c.normal.cyan)?
                .shade(-SHADEF)
                .to_css(false),
            green: Color::from_hex_str(&c.normal.green)?
                .shade(-SHADEF)
                .to_css(false),
            magenta: Color::from_hex_str(&c.normal.magenta)?
                .shade(-SHADEF)
                .to_css(false),
            red: Color::from_hex_str(&c.normal.red)?
                .shade(-SHADEF)
                .to_css(false),
            white: Color::from_hex_str(&c.normal.white)?
                .shade(-SHADEF)
                .to_css(false),
            yellow: Color::from_hex_str(&c.normal.yellow)?
                .shade(-SHADEF)
                .to_css(false),
        };
        let is_light = Color::from_hex_str(&c.primary.background)?.to_hsl().2 > 50.;
        let m = if is_light { -1. } else { 1. };
        let z = if is_light { 0. } else { 100. };
        let bg = Color::from_hex_str(&c.primary.background)?;
        let bg0 = if (bg.to_hsl().2 + BGS[0] * m - z) * (-m) - 1. < 100. {
            bg.brighten(BGS[0] * m)
        } else {
            bg.brighten(-BGS[0] * m)
        };
        let bg2 = bg.brighten(BGS[1] * m);
        let bg3 = bg.brighten(BGS[2] * m);
        let bg4 = bg.brighten(BGS[3] * m);
        let fg = Color::from_hex_str(&c.primary.foreground)?;
        let fg0 = if (fg.to_hsl().2 + FGS[0] * m - z) * (-m) - 1. > 0. {
            fg.brighten(FGS[0] * m)
        } else {
            fg.brighten(-FGS[0] * m)
        };
        let fg2 = fg.brighten(FGS[1] * m);
        let fg3 = fg.brighten(FGS[2] * m);
        let sel = Color::from_hex_str(&c.selection.background)?;
        let sel1 = sel.brighten(SELS * m);

        let orange_normal =
            Color::from_hex_str(&c.normal.red)?.blend(&Color::from_hex_str(&c.normal.yellow)?, 0.5);
        let pink_normal =
            Color::from_hex_str(&c.normal.red)?.blend(&Color::from_hex_str(&c.normal.white)?, 0.5);
        let orange_bright =
            Color::from_hex_str(&c.bright.red)?.blend(&Color::from_hex_str(&c.bright.yellow)?, 0.5);
        let pink_bright =
            Color::from_hex_str(&c.bright.red)?.blend(&Color::from_hex_str(&c.bright.white)?, 0.5);
        let orange_dim =
            Color::from_hex_str(&dim.red)?.blend(&Color::from_hex_str(&dim.yellow)?, 0.5);
        let pink_dim = Color::from_hex_str(&dim.red)?.blend(&Color::from_hex_str(&dim.white)?, 0.5);
        color::print_palette(&[
            orange_normal,
            pink_normal,
            orange_bright,
            pink_bright,
            orange_dim,
            pink_dim,
        ]);
        color::print_palette(&c.normal.as_colors_arr()?);
        color::print_palette(&c.bright.as_colors_arr()?);
        color::print_palette(&dim.as_colors_arr()?);
        color::print_palette(&[bg0, bg, bg2, bg3, bg4]);
        color::print_palette(&[fg0, fg, fg2, fg3]);
        color::print_palette(&[sel, sel1]);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("\n\n\n\n");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let white = color::Color::from_hex_str("#ffffff")?;
    // let black = color::Color::from_hex_str("#000000")?;

    // color::print_palette(&[black, black.brighten(4.0)]);

    // println!("{} {}", black.to_css(false), black.brighten(4.0).to_css(false));

    gent()?;

    return Ok(());
    let cli = Cli::parse();

    if !supports_truecolor() {
        println!("Warning: Your terminal does not fully support truecolor");
    }

    if cli.theme_list {
        {
            if cli.dark {
                collection::DARK_LIST.iter()
            } else if cli.light {
                collection::LIGHT_LIST.iter()
            } else {
                collection::LIST.iter()
            }
        }
        .for_each(|i| println!("{i}"));
    }
    if cli.font_list {
        list_nerd_fonts()?.iter().for_each(|i| println!("{i}"));
    }

    let mut has_error = false;

    if let Some(query) = cli.font {
        if let Err(e) = set_alacritty_font(&query) {
            has_error = true;
            eprintln!("{}", e);
        }
    } else if cli.font_rand {
        let res = set_alacritty_font(
            list_nerd_fonts()?
                .choose(&mut rand::rng())
                .unwrap_or(&"".into()),
        );
        if let Err(e) = res {
            has_error = true;
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
            if nvim_config_path().exists() {
                if let Err(e) = apply_theme_to_nvim(&mut theme) {
                    has_error = true;
                    eprintln!("{}", e);
                }
            }
            if alacritty_config_path().exists() {
                if let Err(e) = apply_theme_to_alacritty(&mut theme) {
                    has_error = true;
                    eprintln!("{}", e);
                }
            }
        }
    }
    if has_error {
        exit(1);
    }

    Ok(())
}
