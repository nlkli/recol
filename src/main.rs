mod cli;
mod font;
mod targets;
mod tmpstore;
mod utils;

use recol_lib as lib;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    tmpstore::init();

    let mut collection = lib::Collection::new();
    let filters = args.filters();

    if args.theme_list {
        collection
            .name_list(&filters)
            .iter()
            .for_each(|n| println!("{n}"));
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
        }

        if let Some(ref query) = args.font {
            let candidates = font_list.iter().map(|f| f.as_str()).collect::<Vec<_>>();
            font_name = lib::fuzzy::search(query, &candidates).map(String::from);
        }

        if let Some(ref font_name) = font_name {
            targets::apply_font(font_name)?;
            tmpstore::append_font_history(font_name);
        }
    }

    let mut theme = None;
    if args.rand {
        let theme_history = tmpstore::read_theme_history(21);
        theme.replace(collection.random(&filters).unwrap().into_theme());
        let mut n: usize = 0;
        while n < 9 {
            if let Some(ref t) = theme {
                if theme_history.contains(&t.name) {
                    theme.replace(collection.random(&filters).unwrap().into_theme());
                    n += 1;
                    continue;
                }
            }
            break;
        }
    }

    if let Some(ref query) = args.theme {
        theme.replace(
            collection
                .fuzzy_search(query, &filters)
                .unwrap_or(collection.random(&filters).unwrap())
                .into_theme(),
        );
    }

    if let Some(ref mut theme) = theme {
        let print_header = || {
            let tmod = if theme.is_light { "LIGHT" } else { "DARK" };
            println!("{} <{tmod}>", theme.name);
        };
        loop {
            if args.show {
                print_header();
                theme.print_palette();
                break;
            }
            if args.json {
                let json_str = serde_json::to_string_pretty(&serde_json::json!({
                    "name": &theme.name,
                    "is_light": &theme.is_light,
                    "colors": &theme.colors.clone().into_advanced(lib::AdvancedColorSchemeParam::default()),
                })).unwrap();
                println!("{}", json_str);
                break;
            }
            print_header();
            targets::apply_theme(&args, theme)?;
            tmpstore::append_theme_history(&theme.name);
            break;
        }
    }

    Ok(())
}
