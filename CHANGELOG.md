# Changelog

## 0.1.0 — local scaffold milestone

- Added the standalone `rusty` CLI with project creation, existing-directory
  initialization, template inspection and environment diagnostics.
- Added binary, library, clap CLI and application/library workspace starters.
- Added module, integration-test and example generation with dry-run previews.
- Added strict layered TOML/environment/CLI configuration and author/edition/license metadata.
- Added trusted local templates, opt-in Git/CI, JSON output and generated README/style files.
- Added preflight collision checks, portable-path validation and capability-relative writes.
- Added CLI/config/render/safety tests, real Cargo verification of every starter,
  cross-platform CI configuration and a self-cleaning smoke walkthrough.

Implementation and local verification took place on 2026-10-15; Git dates were
reconstructed at the owner's request. See `docs/history.md`; this is not a claim
of a historical release or of remote CI execution. The original async example
and pre-existing local HTTP edits were preserved separately from this milestone.
