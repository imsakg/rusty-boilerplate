use std::process::Command;
#[test]
fn init_discovers_configuration_from_the_destination_directory() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("destination");
    std::fs::create_dir(&target).unwrap();
    std::fs::write(
        target.join(".rusty.toml"),
        "template='lib'\nedition='2018'\nauthors=['Target']",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .args(["init", "destination", "--name", "demo"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(target.join("src/lib.rs").is_file());
    let manifest: toml::Value = std::fs::read_to_string(target.join("Cargo.toml"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(manifest["package"]["authors"][0].as_str(), Some("Target"));
}
#[test]
fn missing_and_malformed_configs_fail_before_creating_files() {
    let dir = tempfile::tempdir().unwrap();
    for source in [None, Some("unknown = true")] {
        if let Some(source) = source {
            std::fs::write(dir.path().join("bad.toml"), source).unwrap();
        }
        let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
            .current_dir(dir.path())
            .args(["--config", "bad.toml", "new", "demo"])
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(!dir.path().join("demo").exists());
    }
}
