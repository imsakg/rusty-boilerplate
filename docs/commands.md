# Command reference

Examples assume `cargo install --path scaffold --locked` has installed `rusty`.
Run any command with `--help` for the clap-generated argument reference.

## Global options

- `-h`, `--help`: print command help.
- `--version`: print the CLI version.
- `-v`, `--verbose`: emit diagnostics on stderr; repeatable.
- `--config PATH`: use one exact TOML file instead of user/project discovery.
- `--json`: emit one JSON object on stdout for application success or failure.

JSON success: `{"ok":true,"message":"..."}`. JSON application failure:
`{"ok":false,"error":"..."}`. The message is human-readable text, not a stable
file-list schema. Invalid command syntax is handled by clap before application
dispatch and remains normal clap stderr, even with `--json`.

Exit codes: 0 for success/help/version (also a closed output pipe); 1 for an
application or output failure; 2 for invalid CLI syntax. Some write failures can
leave created files, so inspect the error and destination before retrying.

## Projects

```sh
rusty new NAME [--path PATH] [PROJECT_OPTIONS]
rusty init [PATH] [--name NAME] [PROJECT_OPTIONS]
```

`new` requires a destination that does not exist, even if it would be empty.
`init` requires an existing directory and infers its basename unless `--name`
is given. Neither overwrites existing files. Project names must be portable
Cargo names, not paths; use `--path` to select a destination outside the cwd.
Names start with an ASCII letter and continue with ASCII letters, digits,
underscores or hyphens; Rust keywords, device names and repeated hyphens are rejected.

Project options:

- `-t`, `--template bin|lib|cli|workspace`: select a built-in starter.
- `--template-dir PATH`: read a trusted local template (conflicts with `--template`).
- `--edition 2018|2021`: override the configured edition.
- `--author TEXT`: replace configured authors; repeat for multiple authors.
- `--license MIT|Apache-2.0|'MIT OR Apache-2.0'`: set SPDX metadata and license texts.
- `--git`: initialize local Git after writing files; no commit, remote or push.
- `--ci`: include `.github/workflows/ci.yml`; does not execute CI.
- `--dry-run`: validate inputs/destinations and print paths without writing anything.

The workspace template creates `app/` and `core/`; run its app with
`cargo run -p NAME-app`. Library projects use `cargo test` rather than `cargo run`.

## File generation

```sh
rusty generate module NAME [--path PATH] [--dry-run]
rusty generate test NAME [--path PATH] [--dry-run]
rusty generate example NAME [--path PATH] [--dry-run]
```

`--path` defaults to `.`. The nearest ancestor Cargo **package** is selected;
a virtual workspace alone is not a package. Outputs are `src/NAME.rs`,
`tests/NAME.rs`, or `examples/NAME.rs`. File names must be Rust identifiers,
so use underscores instead of hyphens. Existing Rust declarations are never
rewritten: register generated modules manually in `main.rs` or `lib.rs`.

## Inspection and configuration

```sh
rusty templates [NAME]
rusty config show
rusty config init [PATH]
rusty doctor
```

`templates` does not read project configuration. `config show` prints effective
TOML; with `--json`, that TOML is the envelope's `message` string. `config init`
uses built-in defaults and exclusively creates its destination (default
`.rusty.toml`), independent of discovered defaults. Its parent must already exist.
`doctor` invokes `rustc --version`, `cargo --version` and `git --version` locally.
Rustc/Cargo are required; missing Git is reported but does not fail the command.
No tooling is installed or changed.
