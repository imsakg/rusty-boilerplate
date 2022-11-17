use std::process::Command;
#[test]
fn json_mode_has_a_stable_success_and_failure_envelope() {
    let dir = tempfile::tempdir().unwrap();
    let run = |name: &str| {
        Command::new(env!("CARGO_BIN_EXE_rusty"))
            .current_dir(dir.path())
            .args(["--json", "new", name])
            .output()
            .unwrap()
    };
    let success = run("demo");
    assert!(success.status.success());
    let value: serde_json::Value = serde_json::from_slice(&success.stdout).unwrap();
    assert_eq!(value["ok"], true);
    let failure = run("demo");
    assert!(!failure.status.success());
    let value: serde_json::Value = serde_json::from_slice(&failure.stdout).unwrap();
    assert_eq!(value["ok"], false);
    assert!(value["error"].as_str().unwrap().contains("destination"));
}
