use rusty_boilerplate::plan::Plan;
#[test]
fn preflights_all_collisions_before_creating_any_file() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/main.rs"), "user code").unwrap();
    assert!(Plan::project("demo", "bin")
        .unwrap()
        .write(dir.path())
        .is_err());
    assert!(!dir.path().join("Cargo.toml").exists());
    assert!(!dir.path().join(".gitignore").exists());
    assert_eq!(
        std::fs::read_to_string(dir.path().join("src/main.rs")).unwrap(),
        "user code"
    );
}

#[test]
fn creates_nested_files_with_exact_contents() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("demo");
    let plan = Plan::project("demo", "bin").unwrap();
    let written = plan.write(&root).unwrap();
    assert_eq!(written.len(), plan.files.len());
    for (path, contents) in plan.files {
        assert_eq!(std::fs::read_to_string(root.join(path)).unwrap(), contents);
    }
}
