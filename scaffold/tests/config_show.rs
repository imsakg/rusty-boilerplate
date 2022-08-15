use std::process::Command;
#[test]
fn config_show_emits_parseable_effective_settings() {
    let dir = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .env("RUSTY_EDITION", "2018")
        .args(["config", "show"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: toml::Value = std::str::from_utf8(&output.stdout)
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(value["edition"].as_str(), Some("2018"));
}
