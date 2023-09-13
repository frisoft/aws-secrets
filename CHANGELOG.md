# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## [Unreleased]

<!--
### Features
- Added a new struct `MyStruct` with the following methods:
  - `my_method()`
  - `other_method()`
-->

## v0.1.2 (2023-09-13)

- Update aws-config dependency.

## v0.1.1 (2022-08-26)

- Update keywords in `Cargo.toml`.
- Update examples.
- Rename `Error::SetTags` to `Error::SetTag`.
- Make `Error` non-exhaustive, as it might be a good idea
  when more error variants are added.

[crates.io]: https://crates.io/crates/aws-secrets

## v0.1.0 (2022-08-26)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/aws-secrets
