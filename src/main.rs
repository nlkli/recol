mod models;
mod utils;
mod collection;
mod color;
use color::Color;
// mod templ;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let black = color!("#000000");
    let white = color!("#808080");

    println!("{} {}", black.luminance(), white.luminance());

    let mut file = std::fs::File::create("colorschemes.bin")?;
    collection::converter::write_themes_from_alacritty_dir("colorschemes", &mut file)?;
    Ok(())
}
