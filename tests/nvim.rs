use std::{fs, process::Command};

#[test]
fn nvim_respects_nvim_appname() {
    let temp = tempfile::tempdir().unwrap();

    let config_dir = temp.path().join("config");
    let custom_vim_dir = config_dir.join("customvim");

    fs::create_dir_all(&custom_vim_dir).unwrap();
    fs::write(
        custom_vim_dir.join("init.lua"),
        r#"
-- recol:start
-- old
-- recol:end
"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_recol"))
        .env("XDG_CONFIG_HOME", &config_dir)
        .env("NVIM_APPNAME", "customvim")
        .args(["--target", "nvim", "gruvbox"])
        .output()
        .unwrap();

    assert!(output.status.success());

    let config = fs::read_to_string(custom_vim_dir.join("init.lua")).unwrap();

    assert!(!config.contains("-- old"));
}

#[test]
fn nvim_defaults_to_nvim_appname() {
    let temp = tempfile::tempdir().unwrap();

    let config_dir = temp.path().join("config");
    let nvim_dir = config_dir.join("nvim");

    fs::create_dir_all(&nvim_dir).unwrap();
    fs::write(
        nvim_dir.join("init.lua"),
        r#"
-- recol:start
-- old
-- recol:end
"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_recol"))
        .env("XDG_CONFIG_HOME", &config_dir)
        .env_remove("NVIM_APPNAME")
        .args(["--target", "nvim", "gruvbox"])
        .output()
        .unwrap();

    assert!(output.status.success());

    let config = fs::read_to_string(nvim_dir.join("init.lua")).unwrap();

    assert!(!config.contains("-- old"));
}
