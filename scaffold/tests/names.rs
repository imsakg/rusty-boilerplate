use rusty_boilerplate::name::validate_name;

#[test]
fn accepts_portable_cargo_names() {
    for name in ["demo", "my-app", "my_lib", "a2"] {
        assert!(validate_name(name).is_ok(), "{name}");
    }
}

#[test]
fn rejects_paths_and_nonportable_names() {
    for name in [
        "",
        "../demo",
        "a/b",
        "a\\b",
        "2app",
        "space name",
        "café",
        "a.b",
        "-bad",
    ] {
        assert!(validate_name(name).is_err(), "{name}");
    }
}
