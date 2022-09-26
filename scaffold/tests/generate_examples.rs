use rusty_boilerplate::generate::{file_plan, Kind};
#[test]
fn generates_standalone_runnable_examples() {
    let plan = file_plan(Kind::Example, "hello").unwrap();
    let source = &plan.files[std::path::Path::new("examples/hello.rs")];
    assert!(source.contains("fn main()"));
    assert!(source.contains("hello"));
}
