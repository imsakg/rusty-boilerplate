# Local template directories

Only use a local template you trust. Rusty reads text and writes files; it never
runs a template hook, shell script, Cargo, or a network fetch during generation.
Generated Rust and Cargo metadata may execute code when **you later build them**.
This is not a sandbox for untrusted Rust projects.

## Minimal template

Create this directory (the `.tpl` suffix is optional and is removed in outputs):

```text
starter/
  Cargo.toml.tpl
  src/
    main.rs.tpl
```

`Cargo.toml.tpl`:

```toml
[package]
name = "{{name}}"
version = "0.1.0"
edition = "{{edition}}"
```

`src/main.rs.tpl`:

```rust
fn main() {
    println!("Hello from {{name}}!");
}
```

Run `rusty new demo --template-dir ./starter --dry-run`, then omit `--dry-run`.
`--template-dir` conflicts with `--template`. Local templates still receive the
configured edition, authors and selected license in their Cargo package manifests.
A virtual workspace root is supported when at least one member package manifest
is present. Manifest package names/dependency paths are your template's responsibility.

## Variables and paths

Available variables: `{{name}}`, `{{crate_name}}` (hyphens become underscores), and
`{{edition}}`. Variables work in UTF-8 file paths and file contents. Substitution
is single-pass: inserted values are not interpreted recursively. Single Rust
braces are preserved; `{{...}}` always denotes a template variable. Unknown or
unclosed variables are errors, including in files that do not end in `.tpl`.

Paths must remain relative, portable and unique after suffix removal/rendering.
Symlinks and special files are rejected. Repository control paths such as `.git/`
and Windows drive syntax are forbidden. Each source file is limited to 1 MiB;
a template may contain at most 1024 files. Binary/non-UTF-8 source files are not
supported. File permissions are not copied and scripts are not marked executable.

## File-write boundary

Generation validates its plan and checks every destination before writing.
Existing destination files are not overwritten, and unrelated files remain in
place when using `init`. There is no `--force` mode. A dry run performs these same
checks but does not create directories, files, Git repositories or workflows.

Writing a multi-file project is not a filesystem transaction: disk-full or other
mid-write errors may leave already-created files/directories. Inspect and remove
only the generated paths before retrying. Git initialization happens after file
creation; a failed `--git` reports that the project files already exist.

Writes use a capability-relative directory handle after the destination root is
opened. Descendant symlink swaps cannot redirect those writes outside that root.
The caller-selected root and template source directory must still be trusted;
this does not defend against another process replacing the root before it is
opened or racing changes to the template source itself. Root/ancestor choices
made explicitly by the caller are outside the template's authority boundary.
