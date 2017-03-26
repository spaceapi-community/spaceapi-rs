//! Space API definitions and serialization.
//!
//! This crate contains all data-related definitions that are present in the
//! [Space API](https://spacedirectory.org/). It also handles serializing and
//! deserializing of that data from/to JSON by implementing the Serde
//! [`Serialize`](https://docs.serde.rs/serde/ser/trait.Serialize.html) and
//! [`Deserialize`](https://docs.serde.rs/serde/de/trait.Deserialize.html)
//! traits for all structs.
//!
//! The currently supported Space API version is 0.13. It is not yet fully
//! implemented.
//!
//! If you want to implement a Space API server on top of these types, you
//! might want to take a look at the [`spaceapi_server`
//! crate](https://github.com/coredump-ch/spaceapi-server-rs).
//!
//! This library requires Rust 1.15 or newer.
//!
//! # Examples
//!
//! ## Serializing
//!
//! You can create a new `Status` instance by using the `StatusBuilder`.
//!
//!     extern crate serde;
//!     extern crate serde_json;
//!     extern crate spaceapi;
//!
//!     use spaceapi::{Status, StatusBuilder, Location, Contact};
//!
//!     # fn main() {
//!     let status = StatusBuilder::new("coredump")
//!         .logo("https://www.coredump.ch/logo.png")
//!         .url("https://www.coredump.ch/")
//!         .location(
//!             Location {
//!                 address: None,
//!                 lat: 47.22936,
//!                 lon: 8.82949,
//!             })
//!         .contact(
//!             Contact {
//!                 irc: Some("irc://freenode.net/#coredump".into()),
//!                 twitter: Some("@coredump_ch".into()),
//!                 foursquare: Some("525c20e5498e875d8231b1e5".into()),
//!                 email: Some("danilo@coredump.ch".into()),
//!                 ..Default::default()
//!             })
//!         .add_issue_report_channel("email")
//!         .add_issue_report_channel("twitter")
//!         .build()
//!         .expect("Creating status failed");
//!     let serialized = serde_json::to_string(&status).unwrap();
//!     let deserialized: Status = serde_json::from_str(&serialized).unwrap();
//!
//!     # assert!(&serialized[0..1] == "{");
//!     # assert_eq!(deserialized, status);
//!     # }
//!
//! ## Deserializing
//!
//! You can deserialize any struct of the Space API through Serde:
//!
//!     extern crate serde;
//!     extern crate serde_json;
//!     extern crate spaceapi;
//!
//!     use spaceapi::Location;
//!
//!     # fn main() {
//!     let location = "{\"lat\": 47.22936, \"lon\": 8.82949}";
//!     let decoded: Location = serde_json::from_str(location).unwrap();
//!     println!("{:?}", decoded);
//!
//!     // Output:
//!     // Location { address: None, lat: 47.22936000000001, lon: 8.829490000000002 }
//!     # }

#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod sensors;
mod status;
pub use status::*;

/// Return own crate version. Used in API responses.
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod test {
    use super::get_version;

    #[test]
    fn test_get_version() {
        let version = get_version();
        assert_eq!(3, version.split('.').count());
    }
}
