# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### v0.4.2 (2017-01-23)

- [fixed] Get rid of unneeded attribute warning (#42)
- [fixed] Fix bug in Status serializer (#47)
- [changed] Various small code improvements

### v0.4.1 (2017-01-09)

- [added] `StatusBuilder` which implements the builder pattern (#38)
- [changed] Use custom implementation for `Status` serialization (#40)

### v0.4.0 (2016-12-26)

- [changed] Use serde for (de)-serialization (#35)
- [changed] Library now depends on nightly (#36)

### v0.3.1 (2016-08-09)

- [added] Top level `get_version` function (#31)
- [added] Added `Status::ext_versions` field (#31)

### v0.3.0 (2016-06-25)

- [added] All types now implement `PartialEq` (#22)
- [added] Completed implementation of the `Contact` field (#6)
- [added] `Optional` now implements the `Default` trait (#19)
- [added] Implemented `unwrap_or` for `Optional` (#27)

### v0.2.0 (2015-11-23)

- [added] Support for deserialization (#12)
- [changed] Improved docs (#10)
- [changed] Performance improvements (#11)

### v0.1.1 (2015-11-16)

- [fixed] Fixed documentation URL in `Cargo.toml`

### v0.1.0 (2015-11-14)

- First crates.io release
