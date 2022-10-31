use std::process::Command;
#[test]
fn git_initialization_is_opt_in_and_dry_run_does_not_initialize() {
    let dir = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_rusty"))
            .current_dir(dir.path())
            .args(args)
            .output()
            .unwrap()
    };
    assert!(run(&["new", "plain"]).status.success());
    assert!(!dir.path().join("plain/.git").exists());
    let result = run(&["new", "versioned", "--git"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(dir.path().join("versioned/.git").is_dir());
    assert!(run(&["new", "preview", "--git", "--dry-run"])
        .status
        .success());
    assert!(!dir.path().join("preview").exists());
}
