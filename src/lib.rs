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
//!     extern crate rustc_serialize;
//!     extern crate spaceapi;
//!
//!     use spaceapi::{Status, Location, Contact};
//!     use spaceapi::optional::Optional::{Absent, Value};
//!     use rustc_serialize::json::ToJson;
//!
//!     # fn main() {
//!     let status = Status::new(
//!         "coredump",
//!         "https://www.coredump.ch/logo.png",
//!         "https://www.coredump.ch/",
//!         Location {
//!             address: Value(
//!                 "Spinnereistrasse 2, 8640 Rapperswil, Switzerland".into()),
//!             lat: 47.22936,
//!             lon: 8.82949,
//!         },
//!         Contact {
//!             phone: Absent,
//!             sip: Absent,
//!             keymasters: Absent,
//!             irc: Value("irc://freenode.net/#coredump".into()),
//!             twitter: Value("@coredump_ch".into()),
//!             facebook: Absent,
//!             google: Absent,
//!             identica: Absent,
//!             foursquare: Value("525c20e5498e875d8231b1e5".into()),
//!             email: Value("danilo@coredump.ch".into()),
//!             ml: Absent,
//!             jabber: Absent,
//!             issue_mail: Absent,
//!         },
//!         vec![
//!             "email".into(),
//!             "twitter".into(),
//!         ],
//!     );
//!
//!     let jsonstatus = status.to_json();
//!     let stringstatus = jsonstatus.to_string();
//!
//!     # assert!(&stringstatus[0..1] == "{");
//!     # }
//!
//! ## Deserializing
//!
//! You can deserialize any struct of the Space API through `rustc_serialize::json`:
//!
//!     extern crate rustc_serialize;
//!     extern crate spaceapi;
//!
//!     use rustc_serialize::json;
//!     use spaceapi::Location;
//!
//!     # fn main() {
//!     let location = "{\"lat\": 47.22936, \"lon\": 8.82949}";
//!     let decoded: Location = json::decode(location).unwrap();
//!     println!("{:?}", decoded);
//!
//!     // Output:
//!     // Location { address: Absent, lat: 47.22936000000001, lon: 8.829490000000002 }
//!     # }

#[macro_use]
extern crate log;
extern crate rustc_serialize;

pub mod optional;
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
