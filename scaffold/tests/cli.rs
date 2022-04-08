use std::process::Command;

#[test]
fn binary_runs() {
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Rust project scaffolding"));
}
