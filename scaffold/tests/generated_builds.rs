use rusty_boilerplate::{
    config::Settings,
    generate::{file_plan, Kind},
    plan::Plan,
    template::catalog,
};
use std::process::Command;

#[test]
fn every_generated_starter_builds_and_runs_its_tests() {
    let dir = tempfile::tempdir().unwrap();
    for template in catalog() {
        let name = format!("demo_{}", template.name);
        let root = dir.path().join(&name);
        let mut plan = Plan::project(&name, template.name).unwrap();
        Settings::default().apply(&mut plan).unwrap();
        plan.write(&root).unwrap();
        if template.name != "workspace" {
            for (kind, name) in [(Kind::Test, "manifest"), (Kind::Example, "hello")] {
                file_plan(kind, name).unwrap().write(&root).unwrap();
            }
        }
        for args in [
            vec!["test", "--offline", "--all-targets"],
            vec!["fmt", "--all", "--check"],
            vec![
                "clippy",
                "--offline",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ] {
            let output = Command::new("cargo")
                .current_dir(&root)
                .args(args)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}: {}{}",
                template.name,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
