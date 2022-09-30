use rusty_boilerplate::template::project_files;
#[test]
fn cli_template_has_a_parser_and_its_dependency() {
    let files = project_files("demo", "cli").unwrap();
    let manifest: toml::Value = files[std::path::Path::new("Cargo.toml")].parse().unwrap();
    assert!(manifest["dependencies"].get("clap").is_some());
    assert!(files[std::path::Path::new("src/main.rs")].contains("Parser"));
}
