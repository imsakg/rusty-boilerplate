use std::process::Command;
#[test]
fn doctor_reports_real_tool_versions() {
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .args(["doctor"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8_lossy(&output.stdout);
    for tool in ["rustc", "cargo", "git"] {
        assert!(text.contains(tool), "{text}");
    }
}
#[test]
fn doctor_fails_clearly_when_required_tools_are_missing() {
    let dir = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .env("PATH", dir.path())
        .args(["doctor"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("rustc"));
}
