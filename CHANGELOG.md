# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.


### UNRELEASED

- [added] All types now implement `PartialEq` (#22)
- [added] Completed implementation of the `Contact` field (#6)
- [added] `Optional` now implements the `Default` trait (#19)
- [added] Implement `unwrap_or` for `Optional` (#27)
- ...

### v0.2.0 (2015-11-23)

- [added] Support for deserialization (#12)
- [changed] Improved docs (#10)
- [changed] Performance improvements (#11)

### v0.1.1 (2015-11-16)

- [fixed] Fixed documentation URL in `Cargo.toml`

### v0.1.0 (2015-11-14)

- First crates.io release
