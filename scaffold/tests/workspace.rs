use rusty_boilerplate::{config::Settings, plan::Plan};
#[test]
fn workspace_template_wires_an_app_to_a_local_library() {
    let mut plan = Plan::project("demo", "workspace").unwrap();
    Settings::default().apply(&mut plan).unwrap();
    let root: toml::Value = plan.files[std::path::Path::new("Cargo.toml")]
        .parse()
        .unwrap();
    assert_eq!(root["workspace"]["members"].as_array().unwrap().len(), 2);
    assert!(plan.files[std::path::Path::new("app/Cargo.toml")].contains("../core"));
    assert!(plan.files[std::path::Path::new("app/src/main.rs")].contains("demo_core::add"));
}
#[test]
fn metadata_application_reports_malformed_manifests_without_panicking() {
    for source in ["[dependencies]", "package = 'wrong'"] {
        let mut plan =
            Plan::from_files([(std::path::PathBuf::from("Cargo.toml"), source.into())]).unwrap();
        assert!(Settings::default().apply(&mut plan).is_err());
    }
    let mut plan = Plan::from_files([(
        std::path::PathBuf::from("Cargo.toml"),
        "[package]\nname='demo'\nversion='0.1.0'".into(),
    )])
    .unwrap();
    Settings::default().apply(&mut plan).unwrap();
}
