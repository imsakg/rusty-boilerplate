# Rusty scaffold implementation plan

Goal: turn this personal boilerplate into a useful, tested Rust scaffolding tool while retaining the original async example.

Architecture: an independent crate under `scaffold/` owns the CLI and reusable library. Its own workspace boundary keeps the original service dependencies out of the small generator. Rendering is pure; generation plans separate preview from filesystem effects. No template scripts are executed.

## Delivery checkpoints

1. Establish CLI contracts, portable project names and a deterministic template catalog.
2. Render binary/library projects, model file plans and write without overwriting user files.
3. Expose `new`, `init`, `templates` and dry-run commands.
4. Add typed TOML configuration, discovery, environment layering and explicit CLI overrides.
5. Generate modules, tests and examples; add CLI/workspace starters and project metadata.
6. Support local templates with path/symlink protections and machine-readable output.
7. Exercise generated projects with Cargo; document all commands, safety boundaries and contributor gates.
8. Review and fix edge cases, then verify formatting, Clippy, tests and history metadata.

For each behavior, first run a failing focused test, implement the smallest working slice, run the complete scaffold test suite and commit the slice. Documentation/config-only changes use their corresponding parser or executable examples.

Validation commands:

```sh
cargo fmt --manifest-path scaffold/Cargo.toml --check
cargo clippy --manifest-path scaffold/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path scaffold/Cargo.toml
```

The original package remains separate; its pre-existing uncommitted HTTP experiment is not part of these commits.
