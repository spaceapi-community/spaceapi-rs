//! Space API definitions and serialization.
//!
//! This crate contains all data-related definitions that are present in the Space API
//! (http://spaceapi.net/). It also handles serializing that data to JSON by implementing the
//! [`ToJson`](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/trait.ToJson.html)
//! trait for all structs.
//!
//! # Examples
//!
//! You can create a new ``Status`` instance by using the ``new()`` constructor method. To serialize
//! this to [``Json``](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/enum.Json.html), use
//! the [`ToJson`](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/trait.ToJson.html)
//! trait implementation. You can then create a string from the resulting object.
//!
//!     # extern crate rustc_serialize;
//!     # extern crate spaceapi;
//!     # use spaceapi::{Status, Location, Contact};
//!     # use spaceapi::utils::Optional;
//!     use rustc_serialize::json::ToJson;
//!
//!     # fn main() {
//!     let status = Status::new(
//!         "coredump".to_string(),
//!         "https://www.coredump.ch/logo.png".to_string(),
//!         "https://www.coredump.ch/".to_string(),
//!         Location {
//!             address: Optional::Value("Spinnereistrasse 2, 8640 Rapperswil, Switzerland".to_string()),
//!             lat: 47.22936,
//!             lon: 8.82949,
//!         },
//!         Contact {
//!             irc: Optional::Value("irc://freenode.net/#coredump".to_string()),
//!             twitter: Optional::Value("@coredump_ch".to_string()),
//!             foursquare: Optional::Value("525c20e5498e875d8231b1e5".to_string()),
//!             email: Optional::Value("danilo@coredump.ch".to_string()),
//!         },
//!         vec![
//!             "email".to_string(),
//!             "twitter".to_string(),
//!         ],
//!     );
//!
//!     let jsonstatus = status.to_json();
//!     let stringstatus = jsonstatus.to_string();
//!     # }

extern crate rustc_serialize;

pub mod utils;
pub mod sensors;
mod status;
pub use status::*;

