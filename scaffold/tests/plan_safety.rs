use rusty_boilerplate::{name::validate_name, plan::Plan};
use std::path::PathBuf;
#[test]
fn normalized_aliases_and_file_directory_conflicts_are_rejected() {
    assert!(Plan::from_files([
        (PathBuf::from("a/b"), "first".into()),
        (PathBuf::from("a/./b"), "second".into())
    ])
    .is_err());
    assert!(Plan::from_files([
        (PathBuf::from("a"), "file".into()),
        (PathBuf::from("a/b"), "child".into())
    ])
    .is_err());
    assert!(Plan::from_files([
        (PathBuf::from("a/b"), "child".into()),
        (PathBuf::from("a"), "file".into())
    ])
    .is_err());
}
#[test]
fn mutable_plans_cannot_bypass_write_validation() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("root");
    let mut plan = Plan::project("demo", "bin").unwrap();
    plan.files.insert("../escape".into(), "unsafe".into());
    assert!(plan.write(&root).is_err());
    assert!(!dir.path().join("escape").exists());
    assert!(!root.exists());
}
#[test]
fn rejects_all_reserved_keywords_and_nonportable_file_components() {
    for name in [
        "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof",
        "unsized", "virtual", "yield", "union",
    ] {
        assert!(validate_name(name).is_err(), "{name}");
    }
    for path in ["CON.txt", "src/aux.rs", "name.", "trailing ", "bad\u{1}.rs"] {
        assert!(
            Plan::from_files([(PathBuf::from(path), "source".into())]).is_err(),
            "{path}"
        );
    }
}
