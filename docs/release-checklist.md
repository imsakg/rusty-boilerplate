# Local release checklist

This milestone is not published to crates.io and does not create a Git tag or push.

Before sharing the CLI:

- Run formatting, Clippy, locked tests, doctests and the shell smoke script.
- Confirm every built-in starter passes its generated Cargo test/format/lint checks.
- Review CLI error, dry-run, collision, custom-template and config precedence behavior.
- Review generated license copyright placeholders and workflow actions before publishing.
- Confirm local history dates are explicitly described as reconstructed metadata.
- Keep unrelated pre-existing worktree edits out of the generated history.
- Review the original async package separately; its old ignored lockfile is not the generator's lockfile.

No MSRV lower than the verified current stable toolchain is promised. The lockfile
contains current dependencies, not a reconstruction of a historical crates.io index.
