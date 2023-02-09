#[cfg(unix)]
#[test]
fn descendant_symlink_swaps_cannot_redirect_writes_outside_the_root() {
    use rusty_boilerplate::plan::Plan;
    use std::{
        os::unix::fs::symlink,
        path::PathBuf,
        time::{Duration, Instant},
    };
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("root");
    let outside = dir.path().join("outside");
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::create_dir(&outside).unwrap();
    let plan = Plan::from_files([
        (PathBuf::from("000-first"), "x".repeat(64 * 1024 * 1024)),
        (PathBuf::from("src/escaped.rs"), "not outside".into()),
    ])
    .unwrap();
    let watch_root = root.clone();
    let watch_outside = outside.clone();
    let watcher = std::thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(10);
        while !watch_root.join("000-first").exists() {
            assert!(Instant::now() < deadline, "writer did not start");
            std::thread::yield_now();
        }
        std::fs::remove_dir(watch_root.join("src")).unwrap();
        symlink(watch_outside, watch_root.join("src")).unwrap();
    });
    let result = plan.write(&root);
    watcher.join().unwrap();
    assert!(result.is_err(), "racing destination must not be followed");
    assert!(!outside.join("escaped.rs").exists());
}
