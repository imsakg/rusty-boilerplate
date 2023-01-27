#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT HUP INT TERM
rusty() {
    cargo run --quiet --manifest-path "$root/scaffold/Cargo.toml" -- "$@"
}
cd "$work"
rusty templates
rusty config init
rusty config show
rusty new demo --template lib --license MIT --ci
rusty generate test manifest --path demo
rusty generate example hello --path demo
rusty new preview --template workspace --dry-run
test ! -e preview
cargo fmt --manifest-path demo/Cargo.toml --all --check
cargo test --offline --manifest-path demo/Cargo.toml --all-targets
cargo clippy --offline --manifest-path demo/Cargo.toml --all-targets -- -D warnings
rusty --json templates
printf '%s\n' 'Scaffold smoke passed.'
