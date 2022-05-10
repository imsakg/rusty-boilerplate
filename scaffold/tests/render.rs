use rusty_boilerplate::render::render;
use std::collections::BTreeMap;
#[test]
fn rejects_unknown_and_unclosed_placeholders() {
    let vars = BTreeMap::new();
    assert!(render("{{missing}}", &vars).is_err());
    assert!(render("{{name", &vars).is_err());
}

#[test]
fn inserted_values_are_not_evaluated_twice() {
    let vars = BTreeMap::from([("a".into(), "{{b}}".into()), ("b".into(), "secret".into())]);
    assert_eq!(render("{{a}}", &vars).unwrap(), "{{b}}");
}

#[test]
fn replaces_repeated_placeholders_but_preserves_rust_braces() {
    let vars = BTreeMap::from([("name".into(), "demo".into())]);
    assert_eq!(
        render("fn main() { /* {{name}} {{name}} */ }", &vars).unwrap(),
        "fn main() { /* demo demo */ }"
    );
}
