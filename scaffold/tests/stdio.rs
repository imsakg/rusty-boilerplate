use std::process::{Command, Stdio};
#[cfg(unix)]
#[test]
fn unrelated_non_utf8_environment_does_not_panic() {
    use std::os::unix::ffi::OsStringExt;
    let dir = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .env(
            "UNRELATED_BINARY_VALUE",
            std::ffi::OsString::from_vec(vec![255]),
        )
        .args(["config", "show"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
#[test]
fn verbose_diagnostics_do_not_pollute_json_stdout() {
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .args(["-v", "--json", "templates"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(serde_json::from_slice::<serde_json::Value>(&output.stdout).is_ok());
    assert!(String::from_utf8_lossy(&output.stderr).contains("rusty:"));
}
#[test]
fn closed_stdout_pipe_is_not_a_panic() {
    for _ in 0..5 {
        let mut child = Command::new(env!("CARGO_BIN_EXE_rusty"))
            .args(["templates"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        drop(child.stdout.take());
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
