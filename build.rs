use recol_lib;

fn main() {
    if std::env::var("RECOL_FETCH_GHOSSTY_THEMES").is_ok() {
        let status = std::process::Command::new("./fetch_colorschemes.sh")
            .status()
            .expect("Failed to run fetch_colorschemes.sh");

        assert!(status.success(), "fetch_colorschemes.sh failed");
    }

    let mut output = std::fs::File::create("./recol-lib/src/colorschemes.bin")
        .expect("Failed to create colorschemes.bin");

    recol_lib::build_colorschemes_bin(
        std::env::var("RECOL_GHOSSTY_THEMES_DIR").unwrap_or_else(|_| "./colorschemes".into()),
        &mut output,
        |name| !name.is_empty(),
    )
    .expect("Failed to build colorschemes.bin");
}
