use rusty_boilerplate::template::{catalog, lookup};
#[test]
fn lists_unique_templates_in_stable_order() {
    let names: Vec<_> = catalog().iter().map(|t| t.name).collect();
    assert_eq!(names, ["bin", "cli", "lib"]);
    assert!(!lookup("bin").unwrap().description.is_empty());
    assert!(lookup("missing").is_err());
}
