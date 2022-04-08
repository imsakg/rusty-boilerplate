use std::process::Command;

#[test]
fn version_is_reported() {
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .arg("--version")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("rusty 0.1.0"));
}

#[test]
fn unknown_flags_fail() {
    assert!(!Command::new(env!("CARGO_BIN_EXE_rusty"))
        .arg("--nonexistent")
        .output()
        .unwrap()
        .status
        .success());
}
