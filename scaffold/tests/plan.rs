use rusty_boilerplate::plan::Plan;
use std::path::PathBuf;
#[test]
fn project_plans_are_sorted_and_pure() {
    let plan = Plan::project("demo", "bin").unwrap();
    assert_eq!(
        plan.files.keys().next().unwrap(),
        &PathBuf::from(".gitignore")
    );
    assert!(plan.files.contains_key(&PathBuf::from("Cargo.toml")));
}
#[test]
fn plans_reject_escaping_and_ambiguous_paths() {
    for path in ["../bad", "/absolute", "a/../bad", "a\\b", ""] {
        assert!(
            Plan::from_files([(PathBuf::from(path), "data".into())]).is_err(),
            "{path}"
        );
    }
    assert!(Plan::from_files([
        (PathBuf::from("a"), "1".into()),
        (PathBuf::from("A"), "2".into())
    ])
    .is_err());
}
