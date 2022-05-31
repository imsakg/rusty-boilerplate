use rusty_boilerplate::plan::Plan;
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
