# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-04-30

### Changed
- Migrate from deprecated `ErrorOwe` to `ErrorOweBase` + explicit
  `ConfIOReason`/`UvsReason` in error handling
- Replace deprecated `ErrorWith::with()`/`want()` with `doing()`

[Unreleased]: https://github.com/wp-labs/wp-log/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/wp-labs/wp-log/releases/tag/v0.3.0
