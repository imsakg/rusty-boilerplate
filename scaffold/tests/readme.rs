use rusty_boilerplate::template::{catalog, project_files};
#[test]
fn every_starter_documents_how_to_build_and_test() {
    for template in catalog() {
        let files = project_files("demo", template.name).unwrap();
        let readme = &files[std::path::Path::new("README.md")];
        assert!(readme.starts_with("# demo"));
        assert!(readme.contains("cargo test"));
        if template.name == "workspace" {
            assert!(readme.contains("demo-app"));
        }
    }
}
