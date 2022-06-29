use std::process::Command;
#[test]
fn new_creates_project_and_refuses_an_existing_destination() {
    let dir = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .args(["new", "demo"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dir.path().join("demo/src/main.rs").is_file());
    assert!(!Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .args(["new", "demo"])
        .output()
        .unwrap()
        .status
        .success());
}
