use std::process::Command;
#[test]
fn dry_run_lists_planned_paths_without_creating_a_directory() {
    let dir = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .args(["new", "demo", "--dry-run"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("demo/src/main.rs"));
    assert!(!dir.path().join("demo").exists());
}
