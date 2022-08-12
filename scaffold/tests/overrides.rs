use std::process::Command;
#[test]
fn cli_values_win_over_file_and_environment_defaults() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("config.toml"),
        "template = 'lib'\nedition = '2018'\nauthors = ['File']",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .env("RUSTY_AUTHORS", "Environment")
        .args([
            "--config",
            "config.toml",
            "new",
            "demo",
            "--template",
            "bin",
            "--edition",
            "2021",
            "--author",
            "A \"quoted\" person",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let manifest: toml::Value = std::fs::read_to_string(dir.path().join("demo/Cargo.toml"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(manifest["package"]["edition"].as_str(), Some("2021"));
    assert_eq!(
        manifest["package"]["authors"][0].as_str(),
        Some("A \"quoted\" person")
    );
    assert!(dir.path().join("demo/src/main.rs").exists());
}
