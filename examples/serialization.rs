
extern crate serde;
extern crate serde_json;
extern crate spaceapi;

use spaceapi::{StatusBuilder, Location, Contact};

fn main() {
    let status = StatusBuilder::new("coredump")
        .logo("https://www.coredump.ch/logo.png")
        .url("https://www.coredump.ch/")
        .location(
            Location {
                address: None,
                lat: 47.22936,
                lon: 8.82949,
            })
        .contact(
            Contact {
                irc: Some("irc://freenode.net/#coredump".into()),
                twitter: Some("@coredump_ch".into()),
                foursquare: Some("525c20e5498e875d8231b1e5".into()),
                email: Some("danilo@coredump.ch".into()),
                ..Default::default()
            })
        .add_issue_report_channel("email")
        .add_issue_report_channel("twitter")
        .add_extension("ccc", serde_json::to_value("chaostreff"))
        .build()
        .expect("Creating status failed");
    let stringstatus = serde_json::to_string(&status).unwrap();
    println!("{}", stringstatus);
}
