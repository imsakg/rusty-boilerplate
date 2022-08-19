use std::process::Command;
#[test]
fn config_init_creates_valid_defaults_and_never_clobbers() {
    let dir = tempfile::tempdir().unwrap();
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_rusty"))
            .current_dir(dir.path())
            .args(["config", "init"])
            .output()
            .unwrap()
    };
    assert!(run().status.success());
    let source = std::fs::read_to_string(dir.path().join(".rusty.toml")).unwrap();
    assert!(rusty_boilerplate::config::Settings::parse(&source).is_ok());
    assert!(!run().status.success());
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".rusty.toml")).unwrap(),
        source
    );
}
