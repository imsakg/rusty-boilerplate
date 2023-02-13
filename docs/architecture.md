# Scaffold architecture

The original root package remains a separate async experiment. `scaffold/` is a
standalone Cargo workspace with a small library and the `rusty` binary.

## Boundaries

- `cli`: declarative clap argument types, subcommands and constrained choices.
- `app`: command dispatch, configuration discovery, reporting and opt-in local Git.
- `config`: strict TOML overlays, supported environment keys, semantic validation,
  and TOML-safe metadata application to each package manifest.
- `name`: conservative portable package/file identifier checks.
- `template`: built-in catalog and pure in-memory project rendering.
- `render`: single-pass named substitution without recursive evaluation.
- `local`: trusted UTF-8 local-directory loading, suffix removal and size limits.
- `generate`: source/test/example plans and nearest-package lookup.
- `plan`: path normalization, duplicate/prefix validation, collision preflight,
  exclusive creation and directory-capability-relative output writes.
- `main`: stdout/stderr separation, JSON envelopes and exit/pipe handling.

Rendering never creates files. Config application mutates only an in-memory plan.
`Plan::files` is inspectable/editable by library callers, so writes revalidate all
paths rather than trusting construction alone. Filesystem operations intentionally
do not promise transactional rollback. Template sources are data, not executed hooks.

## Quality gates

```sh
cargo fmt --manifest-path scaffold/Cargo.toml --check
cargo clippy --manifest-path scaffold/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path scaffold/Cargo.toml --locked
sh scripts/smoke.sh
```

The generated-build integration test runs Cargo test, rustfmt and Clippy for
all four built-in templates, plus generated tests/examples. Its Cargo invocations
use `--offline` after the generator has populated the dependency cache. CLI tests
use isolated directories/process environments rather than mutating global env.
Unix-specific tests cover non-Unicode env values, symlink rejection and a controlled
output-directory symlink swap. A configured CI matrix covers Linux/macOS/Windows;
local execution evidence does not claim those remote jobs have run.
