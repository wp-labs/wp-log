# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.4.1] - 2026-09-19

### Changed
- Relicense from Elastic License 2.0 to Apache License 2.0: `Cargo.toml` now declares
  `license = "Apache-2.0"`, the repository ships the standard Apache-2.0 text as `LICENSE`
  (it previously had no `LICENSE` file at all, while `Cargo.toml` and the README both claimed
  `Elastic-2.0`), and the README license section now points at that file
- README: add the standard badge set (crates.io, crates.io downloads, License, Rust Edition) and
  switch the CI badge from `workflows/CI/badge.svg` (no link) to the modern
  `actions/workflow/status/.../ci.yml?branch=main` form linked to the Actions workflow
- Bump `strum_macros` from 0.27 to 0.28

### Fixed
- Remove the redundant borrows in the two `format!("{}.{{}}.gz", &file_path)` calls in
  `src/conf.rs` (`clippy::useless_borrows_in_formatting`, a lint introduced by clippy 1.98);
  no behavior change

## [0.4.0] - 2026-05-03

### Changed
- Bump orion-error from 0.7 to 0.8; enable `anyhow` feature
- Bump orion_conf from 0.6 to 0.7
- Bump orion-variate from 0.12 to 0.13 (unify orion-error to 0.8)
- Replace legacy `.owe(reason).doing(ctx)?` with `.source_err(reason, detail)?` (0.8 API)
- Use `ConfIOReason::logic_error()` / `ConfIOReason::resource_error()` delegate constructors instead of `ConfIOReason::from(UnifiedReason::...)`

### Removed
- Remove `UvsReason` and `ErrorOweBase` imports (removed in 0.8)

## [0.3.0] - 2026-04-30

### Changed
- Migrate from deprecated `ErrorOwe` to `ErrorOweBase` + explicit
  `ConfIOReason`/`UvsReason` in error handling
- Replace deprecated `ErrorWith::with()`/`want()` with `doing()`

[Unreleased]: https://github.com/wp-labs/wp-log/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/wp-labs/wp-log/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/wp-labs/wp-log/releases/tag/v0.4.0
[0.3.0]: https://github.com/wp-labs/wp-log/releases/tag/v0.3.0
