use std::process::Command;
#[test]
fn dual_license_sets_metadata_and_emits_both_texts() {
    let dir = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .args(["new", "demo", "--license", "MIT OR Apache-2.0"])
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
    assert_eq!(
        manifest["package"]["license"].as_str(),
        Some("MIT OR Apache-2.0")
    );
    assert!(std::fs::read_to_string(dir.path().join("demo/LICENSE-MIT"))
        .unwrap()
        .contains("Permission is hereby granted"));
    assert!(
        std::fs::read_to_string(dir.path().join("demo/LICENSE-APACHE"))
            .unwrap()
            .contains("Grant of Patent License")
    );
}
