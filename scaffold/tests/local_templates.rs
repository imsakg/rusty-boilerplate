use std::process::Command;
#[test]
fn local_templates_render_paths_and_contents_without_running_scripts() {
    let dir = tempfile::tempdir().unwrap();
    let template = dir.path().join("starter");
    std::fs::create_dir_all(template.join("src")).unwrap();
    std::fs::write(
        template.join("Cargo.toml.tpl"),
        "[package]\nname='{{name}}'\nversion='0.1.0'\nedition='{{edition}}'\n",
    )
    .unwrap();
    std::fs::write(
        template.join("src/main.rs.tpl"),
        "fn main() { println!(\"{{name}}\"); }\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rusty"))
        .current_dir(dir.path())
        .args(["new", "demo", "--template-dir", "starter"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(std::fs::read_to_string(dir.path().join("demo/src/main.rs"))
        .unwrap()
        .contains("demo"));
    assert!(!dir.path().join("demo/Cargo.toml.tpl").exists());
}
