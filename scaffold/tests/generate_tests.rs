use rusty_boilerplate::{
    generate::{file_plan, Kind},
    plan::Plan,
};
#[test]
fn generates_integration_tests_in_the_tests_directory() {
    let dir = tempfile::tempdir().unwrap();
    Plan::project("demo", "lib")
        .unwrap()
        .write(dir.path())
        .unwrap();
    file_plan(Kind::Test, "manifest")
        .unwrap()
        .write(dir.path())
        .unwrap();
    let source = std::fs::read_to_string(dir.path().join("tests/manifest.rs")).unwrap();
    assert!(source.contains("#[test]"));
    assert!(source.contains("CARGO_MANIFEST_DIR"));
}
