mod cli;
mod font;
mod interactive;
mod targets;
mod tmpstore;
mod utils;

use recol_lib as lib;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[inline]
fn print_theme_header(name: &str, is_light: bool) {
    println!("{name} <{}>", if is_light { "LIGHT" } else { "DARK" });
}

#[inline]
fn theme_as_json(
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

#[inline]
fn print_theme_as_json(name: &str, is_light: bool, colors: &lib::AdvancedColorScheme) {
    let json_str = serde_json::to_string_pretty(&theme_as_json(name, is_light, colors)).unwrap();
    println!("{}", json_str);
}

fn main() -> Result<()> {
    let args = cli::Args::parse();

    tmpstore::init();

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
        if let Some(theme) = tmpstore::read_theme_history(1)
            .first()
            .and_then(|n| collection.by_name(n))
        {
            if args.show {
                print_theme_header(&theme.name, theme.is_light);
                theme.into_theme().print_palette();
            } else if args.json {
                print_theme_as_json(
                    &theme.name,
                    theme.is_light,
                    &theme.into_theme().colors.into_advanced(None),
                );
            } else {
                print_theme_header(theme.name, theme.is_light);
            }
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
            let theme_history = tmpstore::read_theme_history(21);
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
            loop {
                if args.show {
                    print_theme_header(&theme.name, theme.is_light);
                    theme.print_palette();
                    break;
                }
                if args.json {
                    print_theme_as_json(
                        &theme.name,
                        theme.is_light,
                        &theme.colors.clone().into_advanced(None),
                    );
                    break;
                }
                print_theme_header(&theme.name, theme.is_light);
                targets::apply_theme(&args, theme)?;
                tmpstore::append_theme_history(&theme.name);
                break;
            }
        }

        if args.theme_list {
            if theme.is_none() && args.json {
                let json_list = collection
                    .filtered(&filters)
                    .into_iter()
                    .map(|v| {
                        theme_as_json(
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
            let font_history = tmpstore::read_font_history(2);
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
            let font_history = tmpstore::read_font_history(2);
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
            targets::set_font(&args, font_name)?;
            tmpstore::append_font_history(font_name);
        }
    }

    if args.interactive {
        interactive::run(&args)?;
    }

    Ok(())
}
