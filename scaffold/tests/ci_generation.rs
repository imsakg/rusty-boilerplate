use std::process::Command;
#[test]
fn ci_is_generated_only_when_requested() {
    let dir = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_rusty"))
            .current_dir(dir.path())
            .args(args)
            .output()
            .unwrap()
    };
    assert!(run(&["new", "plain"]).status.success());
    assert!(!dir.path().join("plain/.github").exists());
    let output = run(&["new", "checked", "--ci"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let workflow =
        std::fs::read_to_string(dir.path().join("checked/.github/workflows/ci.yml")).unwrap();
    for command in ["cargo fmt", "cargo clippy", "cargo test"] {
        assert!(workflow.contains(command));
    }
}
