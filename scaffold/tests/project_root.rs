use rusty_boilerplate::generate::find_project;
#[test]
fn discovers_nearest_package_from_nested_directories() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "[workspace]\nmembers=[]").unwrap();
    let package = dir.path().join("app");
    std::fs::create_dir_all(package.join("src/nested")).unwrap();
    std::fs::write(
        package.join("Cargo.toml"),
        "[package]\nname='app'\nversion='0.1.0'",
    )
    .unwrap();
    assert_eq!(
        find_project(&package.join("src/nested")).unwrap(),
        package.canonicalize().unwrap()
    );
    assert!(find_project(dir.path()).is_err());
}
