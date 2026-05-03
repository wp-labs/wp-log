# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


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

[Unreleased]: https://github.com/wp-labs/wp-log/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/wp-labs/wp-log/releases/tag/v0.4.0
[0.3.0]: https://github.com/wp-labs/wp-log/releases/tag/v0.3.0
