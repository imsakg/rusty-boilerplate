use std::process::Command;
#[test]
fn explicit_config_controls_generated_manifest() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("defaults.toml"),
        "template = 'lib'\nedition = '2018'\nauthors = ['Ada']\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .args(["--config", "defaults.toml", "new", "demo"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dir.path().join("demo/src/lib.rs").exists());
    let manifest: toml::Value = std::fs::read_to_string(dir.path().join("demo/Cargo.toml"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(manifest["package"]["edition"].as_str(), Some("2018"));
    assert_eq!(manifest["package"]["authors"][0].as_str(), Some("Ada"));
}
