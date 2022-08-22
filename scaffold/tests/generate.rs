use std::process::Command;
#[test]
fn generates_a_module_only_in_a_cargo_project() {
    let dir = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_rusty"))
            .current_dir(dir.path())
            .args(args)
            .output()
            .unwrap()
    };
    assert!(!run(&["generate", "module", "widget"]).status.success());
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname='demo'\nversion='0.1.0'",
    )
    .unwrap();
    let output = run(&["generate", "module", "widget"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dir.path().join("src/widget.rs").is_file());
    assert!(!run(&["generate", "module", "../bad"]).status.success());
    assert!(!run(&["generate", "module", "my-module"]).status.success());
}
