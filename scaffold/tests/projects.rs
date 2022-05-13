use rusty_boilerplate::template::project_files;
#[test]
fn binary_template_has_valid_cargo_metadata() {
    let files = project_files("my-app", "bin").unwrap();
    let manifest: toml::Value = files[std::path::Path::new("Cargo.toml")].parse().unwrap();
    assert_eq!(manifest["package"]["name"].as_str(), Some("my-app"));
    assert!(files[std::path::Path::new("src/main.rs")].contains("fn main()"));
    assert!(project_files("../escape", "bin").is_err());
}
