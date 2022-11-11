use rusty_boilerplate::{local, plan::Plan};
use std::path::PathBuf;
#[test]
fn prevents_repository_control_files_and_windows_drive_paths() {
    for path in [
        ".git/config",
        "nested/.git/hooks/post-checkout",
        "C:/escape",
        "a:b",
    ] {
        assert!(
            Plan::from_files([(PathBuf::from(path), "payload".into())]).is_err(),
            "{path}"
        );
    }
}
#[test]
fn bounds_template_file_sizes() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("huge.tpl"), "x".repeat(1024 * 1024 + 1)).unwrap();
    assert!(local::load(dir.path(), "demo", "2021").is_err());
}
#[cfg(unix)]
#[test]
fn refuses_template_symlinks() {
    let dir = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink("/etc/passwd", dir.path().join("leak.tpl")).unwrap();
    assert!(local::load(dir.path(), "demo", "2021").is_err());
}
