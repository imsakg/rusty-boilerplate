use rusty_boilerplate::config::{discover, Settings};
#[test]
fn finds_nearest_project_file_after_user_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let nested = dir.path().join("project/nested");
    std::fs::create_dir_all(&nested).unwrap();
    let user = dir.path().join("user.toml");
    std::fs::write(&user, "authors = ['Ada']\nedition = '2018'").unwrap();
    std::fs::write(dir.path().join("project/.rusty.toml"), "template = 'lib'").unwrap();
    let paths = discover(&nested, Some(&user));
    assert_eq!(paths, [user, dir.path().join("project/.rusty.toml")]);
    let merged = paths
        .iter()
        .try_fold(Settings::default(), |config, path| {
            config.merge_toml(&std::fs::read_to_string(path).unwrap())
        })
        .unwrap();
    assert_eq!(merged.template, "lib");
    assert_eq!(merged.authors, ["Ada"]);
    assert_eq!(merged.edition, "2018");
}
