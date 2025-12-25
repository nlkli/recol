mod models;
mod utils;
mod collection;
mod color;
// mod templ;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    collection::converter::create_colorshemes_bin("colorschemes")?;
    let col = collection::Collection::new(collection::COLOR_SCHEMES);
    let theme = col.rand(None).unwrap();
    println!("{:#?}", theme.into_theme());
    Ok(())
}
