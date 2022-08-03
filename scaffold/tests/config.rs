use rusty_boilerplate::config::Settings;
#[test]
fn environment_overrides_are_explicit_and_validated() {
    let settings = Settings::default()
        .merge_env([
            ("RUSTY_TEMPLATE", "lib"),
            ("RUSTY_EDITION", "2018"),
            ("RUSTY_AUTHORS", "Ada;Grace"),
        ])
        .unwrap();
    assert_eq!(settings.template, "lib");
    assert_eq!(settings.authors, ["Ada", "Grace"]);
    assert!(Settings::default()
        .merge_env([("RUSTY_EDITION", "invalid")])
        .is_err());
}

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
