use std::path::PathBuf;
pub mod alacritty;
pub mod ghostty;
pub mod nvim;

pub enum Target {
    Alacritty,
    Ghostty,
    Nvim,
}

#[inline(always)]
fn home_dir() -> std::path::PathBuf {
    std::env::home_dir().unwrap()
}

pub fn config_path(target: Target) -> Option<PathBuf> {
    let prefix = match std::env::var("XDG_CONFIG_HOME").ok() {
        Some(p) => PathBuf::from(p),
        None => home_dir().join(".config"),
    };
    match target {
        Target::Alacritty => {
            let path = prefix.join("alacritty/alacritty.toml");
            if path.exists() {
                return Some(path);
            }
            let path = prefix.join("alacritty.toml");
            if path.exists() {
                return Some(path);
            }
            let path = home_dir().join(".alacritty.toml");
            if path.exists() {
                return Some(path);
            }
            let path = PathBuf::from("/etc/alacritty/alacritty.toml");
            if path.exists() {
                return Some(path);
            }
            None
        }
        Target::Ghostty => {
            let path = prefix.join("ghostty/config.ghostty");
            if path.exists() {
                return Some(path);
            }
            let path = prefix.join("ghostty/config");
            if path.exists() {
                return Some(path);
            }
            #[cfg(target_os = "macos")]
            {
                let path = home_dir()
                    .join("Library/Application Support/com.mitchellh.ghostty/config.ghostty");
                if path.exists() {
                    return Some(path);
                }
                let path =
                    home_dir().join("Library/Application Support/com.mitchellh.ghostty/config");
                if path.exists() {
                    return Some(path);
                }
            }
            None
        }
        Target::Nvim => {
            let path = prefix.join("nvim/init.lua");
            if path.exists() {
                return Some(path);
            }
            None
        }
    }
}
