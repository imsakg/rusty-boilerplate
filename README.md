# rusty-boilerplate

A Rust project generator plus the original async-service experiment. The generator
is an independent crate under [`scaffold/`](scaffold/) so its CLI does not pull in
the original Axum/SQLx application dependencies.

## Run or install the CLI

Use a current stable Rust toolchain (the lockfile is verified with Rust 1.99).
No historical MSRV is claimed; see [history provenance](docs/history.md).

```sh
cargo run --manifest-path scaffold/Cargo.toml -- --help
cargo install --path scaffold --locked
rusty doctor
rusty templates
rusty new hello --template bin --dry-run
rusty new hello --template cli --author "Your Name" --license MIT --git --ci
cd hello
cargo run -- --help
cargo test --all-targets
```

The CLI is named `rusty`; its Cargo package is `rusty-boilerplate`. Installation
is local only: the crate is marked `publish = false`.

## Commands

- `new NAME`: create a new directory; `--path PATH` chooses a different destination.
- `init [PATH] --name NAME`: add a project to an existing directory without overwriting files.
- `templates [NAME]`: list starters or describe one. Available: `bin`, `lib`, `cli`, `workspace`.
- `generate module NAME`: create `src/NAME.rs`; add the desired `mod NAME;` declaration yourself.
- `generate test NAME`: create a Cargo integration test.
- `generate example NAME`: create a runnable example.
- `config show`: print effective settings as TOML.
- `config init [PATH]`: create a new config file (default `.rusty.toml`).
- `doctor`: report installed Rust/Git tools without installing or modifying anything.

Use `--dry-run` on project/file generation to check all destinations and preview
paths. `--json` wraps application responses in a machine-readable envelope.
`-v` sends diagnostics to stderr. See the [command reference](docs/commands.md)
for exact details and [custom templates](docs/custom-templates.md) for local sources.

## Configuration

```toml
template = "lib"
edition = "2021"
authors = ["Your Name <you@example.test>"]
license = "MIT OR Apache-2.0"
```

Precedence, low to high:

1. Built-in defaults (`bin`, edition `2021`, no authors or license).
2. `$XDG_CONFIG_HOME/rusty/config.toml`, or `$HOME/.config/rusty/config.toml`.
3. The nearest ancestor `.rusty.toml` (not every ancestor).
4. `RUSTY_TEMPLATE`, `RUSTY_EDITION`, `RUSTY_AUTHORS` (semicolon separated), `RUSTY_LICENSE`.
5. Explicit project flags: `--template`, `--edition`, repeated `--author`, `--license`.

`--config PATH` replaces file discovery with that exact file; environment and CLI
layers still apply. Missing explicit files and invalid layers fail rather than
silently falling back. `init PATH` discovers from its destination; `new` and
`config show` discover from the invoking working directory. Only editions 2018 and
2021 are supported. Unknown TOML keys are errors.

`--git` initializes a local repository without commits or pushes. `--ci` writes a
workflow but does not enable or run a remote service. Review license copyright
placeholders before publishing a generated project.

## Development

```sh
cargo fmt --manifest-path scaffold/Cargo.toml --check
cargo clippy --manifest-path scaffold/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path scaffold/Cargo.toml --locked
```

Tests exercise rendering, configuration, failures, file safety and real Cargo
build/test/lint runs of every generated starter. The CLI's dependency cache must
be populated before the generated-project tests run offline.

The original `Cargo.toml` and `src/` example are retained, not migrated into the
generator. Its old untracked lockfile may need dependency updates on modern Rust;
commands above intentionally scope to the independent generator.
