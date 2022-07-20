use rusty_boilerplate::config::Settings;
#[test]
fn defaults_and_toml_overrides_are_typed() {
    let config = Settings::parse(
        "edition = '2018'\ntemplate = 'lib'\nauthors = ['Ada <ada@example.test>']\n",
    )
    .unwrap();
    assert_eq!(config.edition, "2018");
    assert_eq!(config.template, "lib");
    assert_eq!(config.authors.len(), 1);
    assert_eq!(Settings::default().edition, "2021");
}
#[test]
fn rejects_unknown_keys_and_unsupported_editions() {
    assert!(Settings::parse("edtion = '2021'").is_err());
    assert!(Settings::parse("edition = '1900'").is_err());
    assert!(Settings::parse("template = 'typo'").is_err());
}
