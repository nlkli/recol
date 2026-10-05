use recol_lib;

fn main() {
    // Build the embedded color schemes binary when enabled.
    if std::env::var("RECOL_BUILD_COLORSCHEMES_BIN").is_ok() {
        let mut output = std::fs::File::create("./recol-lib/src/colorschemes.bin")
            .expect("Failed to create colorschemes.bin");

        // Use RECOL_GHOSSTY_THEMES_DIR to provide a custom themes directory.
        // Otherwise, themes are loaded from ./colorschemes.
        recol_lib::build_colorschemes_bin(
            std::env::var("RECOL_GHOSSTY_THEMES_DIR").unwrap_or_else(|_| "./colorschemes".into()),
            &mut output,
            // Exclude unwanted themes from the built-in collection.
            |name| !["theme_to_exclude"].contains(&name),
            // Normalizes black and white colors across themes that use different color ordering.
            // Light themes expect `black` to be lighter than `white`, while dark themes expect the opposite.
            true,
            // `tmux_fix` - `(enabled, target_wcag_contrast_ratio)`. makes `black` readable on `green` (tmux's default status bar).
            (true, 2.3),
            // `normalize_anomalies` - `(enabled, z_threshold, alpha)`.
            // When enabled, pulls the lightness of palette colors that stand out
            // on both contrast and L* toward the median.
            // `z_threshold` is how far a color must deviate to count as an outlier
            // (lower = more colors get fixed). `alpha` is how far to pull it
            // toward the median (0.0 = not at all, 1.0 = all the way).
            (true, 1., 0.666),
        )
        .expect("Failed to build colorschemes.bin");
    }
}
