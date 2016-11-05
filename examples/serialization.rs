
extern crate serde;
extern crate serde_json;
extern crate spaceapi;

use spaceapi::{Status, Location, Contact};

fn main() {
    let status = Status::new(
        "coredump",
        "https://www.coredump.ch/logo.png",
        "https://www.coredump.ch/",
        Location {
            address: None,
            lat: 47.22936,
            lon: 8.82949,
        },
        Contact {
        phone: None,
            sip: None,
            keymasters: None,
            irc: Some("irc://freenode.net/#coredump".into()),
            twitter: Some("@coredump_ch".into()),
            facebook: None,
            google: None,
            identica: None,
            foursquare: Some("525c20e5498e875d8231b1e5".into()),
            email: Some("danilo@coredump.ch".into()),
            ml: None,
            jabber: None,
            issue_mail: None,
        },
        vec!["email".into(), "twitter".into()],
    );
    let stringstatus = serde_json::to_string(&status).unwrap();
    println!("{}", stringstatus);
}

