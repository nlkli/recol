use recol_lib;

const FETCH_GHOSSTY_THEMES: bool = false;
const BUILD_COLORSCHEMES_BIN: bool = false;

fn main() {
    if std::env::var("RECOL_FETCH_GHOSSTY_THEMES").is_ok() || FETCH_GHOSSTY_THEMES {
        let status = std::process::Command::new("./fetch.sh")
            .status()
            .expect("Failed to run fetch_colorschemes.sh");

        assert!(status.success(), "fetch_colorschemes.sh failed");
    }

    if std::env::var("RECOL_BUILD_COLORSCHEMES_BIN").is_ok() || BUILD_COLORSCHEMES_BIN {
        let mut output = std::fs::File::create("./recol-lib/src/colorschemes.bin")
            .expect("Failed to create colorschemes.bin");

        recol_lib::build_colorschemes_bin(
            std::env::var("RECOL_GHOSSTY_THEMES_DIR").unwrap_or_else(|_| "./colorschemes".into()),
            &mut output,
            |name| !["<skip themes by name>"].contains(&name),
        )
        .expect("Failed to build colorschemes.bin");
    }
}
