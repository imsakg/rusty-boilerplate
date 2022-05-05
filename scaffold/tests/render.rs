use rusty_boilerplate::render::render;
use std::collections::BTreeMap;
#[test]
fn replaces_repeated_placeholders_but_preserves_rust_braces() {
    let vars = BTreeMap::from([("name".into(), "demo".into())]);
    assert_eq!(
        render("fn main() { /* {{name}} {{name}} */ }", &vars).unwrap(),
        "fn main() { /* demo demo */ }"
    );
}
