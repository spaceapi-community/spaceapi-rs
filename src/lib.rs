//! Space API definitions and serialization.
//!
//! This crate contains all data-related definitions that are present in the Space API
//! (http://spaceapi.net/). It also handles serializing that data to JSON by implementing the
//! [`ToJson`](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/trait.ToJson.html)
//! trait for all structs.
//!
//! The currently supported Space API version is 0.13. It is not yet fully implemented.
//!
//! # Examples
//!
//! ## Serializing
//!
//! You can create a new `Status` instance by using the `new()` constructor method.
//!
//! To serialize the status to
//! [`Json`](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/enum.Json.html), use
//! the [`ToJson`](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/trait.ToJson.html)
//! trait implementation. You can then create a string from the resulting object.
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
//!     let stringstatus = serde_json::to_string(&status).unwrap();
//!     let jsonstatus: Status = serde_json::from_str(&stringstatus).unwrap();
//!
//!     # assert!(&stringstatus[0..1] == "{");
//!     # }
//!
//! ## Deserializing
//!
//! You can deserialize any struct of the Space API through `rustc_serialize::json`:
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
