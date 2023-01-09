# Configuration examples

The complete precedence contract is in the root README. Configuration is data:
unknown keys, unsupported editions and invalid author/license values are errors.
Validation happens at each layer; a later flag does not hide an invalid earlier layer.

## Per-project library defaults

```toml
template = "lib"
edition = "2021"
authors = ["Ada Example"]
license = "MIT"
```

## One-off overrides

```sh
RUSTY_EDITION=2018 rusty new compatible --template bin
RUSTY_AUTHORS='Ada;Grace' rusty new shared --license 'MIT OR Apache-2.0'
rusty --config defaults.toml new tool --author 'Explicit Author'
rusty config show
```

An empty author list is supported in TOML. A present but empty `RUSTY_AUTHORS`
value is rejected instead of silently clearing defaults. Authors are serialized
with TOML escaping, so quotation marks do not corrupt generated manifests.
Environment keys unrelated to the documented `RUSTY_*` settings are ignored.

Configuration files do not support `include`, shell expansion, executable values,
remote URLs, or secrets. Do not store credentials in project defaults.
