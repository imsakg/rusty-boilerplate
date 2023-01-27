# End-to-end walkthrough

From the repository root, use the independent generator:

```sh
cargo install --path scaffold --locked
rusty doctor
rusty config init
rusty new example-lib --template lib --license MIT --ci
rusty generate test manifest --path example-lib
rusty generate example hello --path example-lib
cargo test --manifest-path example-lib/Cargo.toml --all-targets
cargo run --manifest-path example-lib/Cargo.toml --example hello
```

`generate module domain --path example-lib` creates `src/domain.rs`; expose it by
adding `pub mod domain;` to `src/lib.rs` yourself. The generator does not silently
edit existing Rust source files or module declarations.

Preview a multi-package project with:

```sh
rusty new my-workspace --template workspace --dry-run
rusty new my-workspace --template workspace
cargo run --manifest-path my-workspace/Cargo.toml -p my-workspace-app
```

`scripts/smoke.sh` executes a self-cleaning walkthrough in a temporary directory.
It uses `TMPDIR` when set by the host and requires Cargo, rustfmt and Clippy.
The generated-project checks run offline after the CLI dependencies are cached.
No command in the smoke initializes a remote, commits, pushes, or runs CI.
