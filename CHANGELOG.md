# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### V0.9.0 (2023-05-07)

- [changed] Changed the way sensor types are exported.
  Some sensor types that were previously exported in the `spaceapi` module now need to be imported from the `spaceapi::sensors` module.
- [changed] The `names` field is no longer part of `PeopleNowPresentSensor`.
- [added] Added all missing sensor types.
- [changed] Rewrite parts of the sensor implementation.
  This is a breaking change as it changes the way sensor templates are constructed for `PeopleNowPresentSensor` and `TemperatureSensor`.
- [added] Added the `HumitidySensor` and `PowerConsumptionSensor` sensor types.
  This is a breaking change as it adds the `humidity` and `power_consumption` fields to the `Sensors` struct.
- [added] Added the `timezone` key under `location` section.
- [added] Added the `links` section.
- [added] Added the `membership_plans` section.
- [added] Derive `Eq` as well and not just `PartialEq` where possible.
- [fixed] The name for the "closed icon" field has been corrected to `state.icon.closed` (was `state.icon.close`).
- [changed] Bump MSRV to 1.56
- [changed] Require at least Rust 1.56.

### v0.8.1 (2021-07-06)

 - [changed] Allow `state.open` to be `null` again

### v0.8.0 (2021-05-09)

- [added] The `Sensors`, `PeopleNowPresentSensor` and `TemperatureSensor`
  structs now derive `Default` (#84)
- [added] Basic support for the new v14 API (#85, #87, #89, #90, #91) This is a
  breaking change since it changes the `api` field of `Status` to
  `Option<String>` and the `state` field to `Option<State>`. Includes
  * Add `xmpp` field in `Contact` struct and deprecate the `jabber` field
  * Add `xmpp` field to `Keymaster`
  * Rename jabber to xmpp in contact field
  * Add xmpp to contact/keymasters
  * Deprecate contacts.google, issue\_report\_channels, radio\_show,
    spacefed.spacephone and stream keys
  * Add `mumble`, `matrix`, `mastodon` and `gopher` to `Contact`
  * Add `mastodon` field to `Keymaster`
  * Make `state` field optional and disallow `null` for `state.open`


### v0.7.0 (2019-08-22)

- [changed] Require Rust 1.31+ (Rust 2018 edition) (#76)
- [changed] Use [serde flatten](https://serde.rs/attr-flatten.html) to
  implement extensions. This changes the in memory representation of the
  extensions to include the `ext_` prefix. (#79)
- [added] Add missing stream field (#80)

### v0.6.0 (2018-01-25)

- [changed] Minimal required Rust version 1.20.0 (#68)
- [changed] Update `log` crate to version 0.4 (#70)
- [changed] Use an enum (`IssueReportChannel`) for the `issue_report_channels`
  field (#4)

### v0.5.1 (2017-06-17)

- [fixed] Fix compilation warning (#65)

### v0.5.0 (2017-06-17)

- [changed] Update to serde 1.0
- [added] Support arbitrary extension fields (#34)
- [added] Add more methods to `StatusBuilder` (#54)
- [changed] Rename `_type` fields to `type_` (#58)
- [changed] Fix and improve crate documentation (#55)
- [removed] Get rid of `Status::ext_versions` field, use extensions
  mechanism instead (#48)

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
