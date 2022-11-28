use rusty_boilerplate::template::{catalog, project_files};
#[test]
fn starters_include_stable_formatting_and_lint_configuration() {
    for template in catalog() {
        let files = project_files("demo", template.name).unwrap();
        let format: toml::Value = files[std::path::Path::new("rustfmt.toml")].parse().unwrap();
        assert_eq!(format["max_width"].as_integer(), Some(100));
        assert!(files.contains_key(std::path::Path::new("clippy.toml")));
    }
}
