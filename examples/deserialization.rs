extern crate serde_json;
extern crate spaceapi;

use spaceapi::Status;
use serde_json::from_str;

fn main() {
    let input = r#"{"api":"0.13","space":"coredump","logo":"https://www.coredump.ch/logo.png","url":"https://www.coredump.ch/","location":{"lat":47.22936,"lon":8.82949},"contact":{"irc":"irc://freenode.net/#coredump","twitter":"@coredump_ch","foursquare":"525c20e5498e875d8231b1e5","email":"danilo@coredump.ch"},"issue_report_channels":["email","twitter"],"state":{"open":null},"ext_ccc":"chaostreff"}"#;
    let status: Result<Status, _> = from_str(input);
    match status {
        Ok(status) => println!("{:?}", status),
        Err(err) => println!("Could not parse status: {}", err),
    };
}

