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
//! You can create a new ``Status`` instance by using the ``new()`` constructor method. To serialize
//! this to [``Json``](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/enum.Json.html), use
//! the [`ToJson`](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/trait.ToJson.html)
//! trait implementation. You can then create a string from the resulting object.
//!
//!     # extern crate rustc_serialize;
//!     # extern crate spaceapi;
//!     # use spaceapi::{Status, Location, Contact};
//!     # use spaceapi::optional::Optional;
//!     use rustc_serialize::json::ToJson;
//!
//!     # fn main() {
//!     let status = Status::new(
//!         "coredump",
//!         "https://www.coredump.ch/logo.png",
//!         "https://www.coredump.ch/",
//!         Location {
//!             address: Optional::Value("Spinnereistrasse 2, 8640 Rapperswil, Switzerland".into()),
//!             lat: 47.22936,
//!             lon: 8.82949,
//!         },
//!         Contact {
//!             irc: Optional::Value("irc://freenode.net/#coredump".into()),
//!             twitter: Optional::Value("@coredump_ch".into()),
//!             foursquare: Optional::Value("525c20e5498e875d8231b1e5".into()),
//!             email: Optional::Value("danilo@coredump.ch".into()),
//!         },
//!         vec![
//!             "email".into(),
//!             "twitter".into(),
//!         ],
//!     );
//!
//!     let jsonstatus = status.to_json();
//!     let stringstatus = jsonstatus.to_string();
//!     # }

#[macro_use]
extern crate log;
extern crate rustc_serialize;

pub mod optional;
pub mod sensors;
mod status;
pub use status::*;
