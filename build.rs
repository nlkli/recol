use recol_lib;

fn main() {
    // Build the embedded color schemes binary when enabled.
    if std::env::var("RECOL_BUILD_COLORSCHEMES_BIN").is_ok() {
        let mut output = std::fs::File::create("./recol-lib/src/colorschemes.bin")
            .expect("Failed to create colorschemes.bin");

        // Use RECOL_GHOSSTY_THEMES_DIR to provide a custom themes directory.
        // Otherwise, themes are loaded from ./colorschemes.
        recol_lib::build_colorschemes_bin(
            std::env::var("RECOL_GHOSSTY_THEMES_DIR")
                .unwrap_or_else(|_| "./colorschemes".into()),
            &mut output,

            // Exclude unwanted themes from the built-in collection.
            |name| !["theme_to_exclude"].contains(&name),
        )
        .expect("Failed to build colorschemes.bin");
    }
}
