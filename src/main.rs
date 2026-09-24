mod cli;
mod font;
mod interactive;
mod state;
mod targets;
mod utils;

use recol_lib as lib;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn print_theme_palette(t: &lib::ThemeEx) {
    lib::print_palette(&t.colors.as_colors_array()[0..14]);
}

#[inline]
pub fn print_theme_header(name: &str, is_light: bool) {
    println!("{name} <{}>", if is_light { "LIGHT" } else { "DARK" });
}

fn theme_as_json_value(
    name: &str,
    is_light: bool,
    colors: &lib::AdvancedColorScheme,
) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "is_light": is_light,
        "colors": colors,
    })
}

pub fn print_theme_as_json(t: lib::ThemeEx) {
    let jv = theme_as_json_value(
        &t.name,
        t.is_light,
        &t.advanced.unwrap_or(t.colors.into_advanced(None)),
    );
    println!("{}", serde_json::to_string_pretty(&jv).unwrap());
}

fn apply_theme(args: &cli::Args, theme: &lib::ThemeEx) -> Result<()> {
    targets::apply_theme(
        targets::with_existing_default_config_path(targets::with_specific_or_for_all(
            &args.targets,
        )),
        theme,
    )?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
enum StdIn {
    Theme(lib::ThemeEx),
    List(Vec<lib::ThemeEx>),
    NotValid,

    #[default]
    None,
}

fn read_stdin(args: &cli::Args) -> Result<StdIn> {
    use std::io::{IsTerminal, Read};

    let stdin = std::io::stdin();
    if stdin.is_terminal() {
        return Ok(StdIn::None);
    }

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let trimmed = input.trim();
    let Some(first) = trimmed.chars().next() else {
        return Ok(StdIn::None);
    };

    Ok(match first {
        '{' => match serde_json::from_str::<lib::AdvancedTheme>(trimmed) {
            Ok(theme) => StdIn::Theme(theme.ex()),
            _ => StdIn::NotValid,
        },
        '[' => match serde_json::from_str::<Vec<lib::AdvancedTheme>>(trimmed) {
            Ok(list) => StdIn::List(list.into_iter().map(|v| v.ex()).collect()),
            _ => StdIn::NotValid,
        },
        _ => {
            let mut c = lib::Collection::new();
            let filters = args.theme_filters();
            let mut list = trimmed
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .filter_map(|l| {
                    c.find(|t| t.name == l)
                        .or(c.fuzzy_search(l, &filters, None))
                })
                .map(|t| t.into_theme().ex())
                .collect::<Vec<_>>();
            if list.is_empty() {
                StdIn::None
            } else if list.len() == 1 {
                StdIn::Theme(list.into_iter().next().unwrap())
            } else {
                list.sort_by(|a, b| a.name.cmp(&b.name));
                list.dedup_by(|a, b| a.name == b.name);
                StdIn::List(list)
            }
        }
    })
}

fn theme_action(args: &cli::Args, mut theme: lib::ThemeEx) -> Result<()> {
    theme.colors.apply_adjustments(&args.adjust);
    if args.show {
        print_theme_header(&theme.name, theme.is_light);
        print_theme_palette(&theme);
    } else if args.json {
        print_theme_as_json(theme);
    } else {
        apply_theme(&args, &theme)?;
        if lib::Collection::new().any(|t| t.name == theme.name) {
            state::append_theme_history(&theme.name);
        }
        print_theme_header(&theme.name, theme.is_light);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let stdin = read_stdin(&args)?;

    match stdin {
        StdIn::Theme(v) => {
            theme_action(&args, v)?;
            return Ok(());
        }
        StdIn::List(list) => {
            if let Some(theme) = if args.rand {
                fastrand::choice(&list)
            } else {
                list.first()
            } {
                theme_action(&args, theme.clone())?;
            }
            return Ok(());
        }
        StdIn::NotValid => {}
        StdIn::None => {}
    }

    if let Some(ref media) = args.media {
        if let Some(max_colors) = args.palettegen {
            let palette = lib::palettegen_from_media(media, max_colors)?;
            if args.show {
                lib::print_palette(&palette);
            } else if args.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &palette.into_iter().map(|c| c.css()).collect::<Vec<_>>()
                    )?
                );
            } else {
                palette.into_iter().for_each(|c| println!("{c}"));
            }
            return Ok(());
        }
        let cs = lib::ColorScheme::from_media(media)?;
        let theme = lib::Theme::new(
            media.file_stem().unwrap().to_str().unwrap(),
            cs.is_light(),
            cs,
        )
        .ex();
        theme_action(&args, theme)?;
        return Ok(());
    }

    let mut collection = lib::Collection::new();

    if args.theme.is_none()
        && !args.rand
        && args.contains.is_none()
        && !args.theme_list
        && args.font.is_none()
        && !args.font_rand
        && !args.font_list
        && !args.interactive
    {
        if let Some(lazy_theme) = state::read_theme_history(1)
            .first()
            .and_then(|n| collection.by_name(n))
        {
            let theme = lazy_theme.into_theme().ex();
            theme_action(&args, theme)?;
        }
        return Ok(());
    }

    if args.theme.is_some() || args.rand || args.theme_list {
        let filters = args.theme_filters();

        let mut theme = None;

        if let Some(ref query) = args.theme {
            theme = theme.or(collection
                .fuzzy_search(query, &filters, None)
                .map(|v| v.into_theme()));
        } else if args.rand {
            let theme_history = state::read_theme_history(21);
            let mut choice = collection.random(&filters);
            let mut n = 0;
            while let Some(ref t) = choice {
                if !theme_history.contains(&t.name.to_string()) || n > 9 {
                    break;
                }
                choice = choice.or(collection.random(&filters));
                n += 1;
            }
            theme = theme.or(choice.map(|v| v.into_theme()));
        }

        if let Some(ref theme) = theme {
            theme_action(&args, theme.clone().ex())?;
        }

        if args.theme_list {
            if theme.is_none() && args.json {
                let json_list = collection
                    .filtered(&filters)
                    .into_iter()
                    .map(|v| {
                        theme_as_json_value(
                            v.name,
                            v.is_light,
                            &v.into_theme().colors.into_advanced(None),
                        )
                    })
                    .collect::<Vec<_>>();
                println!("{}", serde_json::to_string_pretty(&json_list)?);
            } else {
                collection
                    .name_list(&filters)
                    .iter()
                    .for_each(|n| println!("{n}"));
            }
        }
    }

    if args.font_list || args.font_rand || args.font.is_some() {
        let mut font_name = None;

        let font_list = font::list(|_| true)?;
        if args.font_list {
            font_list.iter().for_each(|f| println!("{f}"));
        }

        if args.font_rand {
            let font_history = state::read_font_history(2);
            font_name = fastrand::choice(&font_list).cloned();
            let mut n: usize = 0;
            while n < 5 {
                if let Some(ref f) = font_name {
                    if font_history.contains(f) {
                        font_name = fastrand::choice(&font_list).cloned();
                        n += 1;
                        continue;
                    }
                }
                break;
            }
            let font_history = state::read_font_history(2);
            font_name = fastrand::choice(&font_list).cloned();
            let mut n = 0;
            while let Some(ref f) = font_name {
                if !font_history.contains(f) || n > 3 {
                    break;
                }
                font_name = font_name.or(fastrand::choice(&font_list).cloned());
                n += 1;
            }
        }

        if let Some(ref query) = args.font {
            let candidates = font_list.iter().map(|f| f.as_str()).collect::<Vec<_>>();
            font_name = lib::fuzzy::search(query, &candidates, None).map(String::from);
        }

        if let Some(ref font_name) = font_name {
            targets::set_font(
                targets::with_existing_default_config_path(targets::with_specific_or_for_all(
                    &args.targets,
                )),
                font_name,
            )?;
            state::append_font_history(font_name);
        }
    }

    if args.interactive {
        interactive::run(&args)?;
    }

    Ok(())
}
