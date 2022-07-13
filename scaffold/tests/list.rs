use std::process::Command;
#[test]
fn lists_templates_and_explains_a_single_starter() {
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_rusty"))
            .args(args)
            .output()
            .unwrap()
    };
    let list = run(&["templates"]);
    assert!(list.status.success());
    assert!(String::from_utf8_lossy(&list.stdout).contains("bin"));
    assert!(String::from_utf8_lossy(&run(&["templates", "lib"]).stdout).contains("unit test"));
    assert!(!run(&["templates", "absent"]).status.success());
}
