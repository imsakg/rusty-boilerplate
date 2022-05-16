use rusty_boilerplate::template::project_files;
#[test]
fn library_template_includes_an_executable_unit_test() {
    let files = project_files("math-lib", "lib").unwrap();
    let source = &files[std::path::Path::new("src/lib.rs")];
    assert!(source.contains("pub fn add"));
    assert!(source.contains("#[test]"));
    assert!(!files.contains_key(std::path::Path::new("src/main.rs")));
}

#[test]
fn binary_template_has_valid_cargo_metadata() {
    let files = project_files("my-app", "bin").unwrap();
    let manifest: toml::Value = files[std::path::Path::new("Cargo.toml")].parse().unwrap();
    assert_eq!(manifest["package"]["name"].as_str(), Some("my-app"));
    assert!(files[std::path::Path::new("src/main.rs")].contains("fn main()"));
    assert!(project_files("../escape", "bin").is_err());
}
