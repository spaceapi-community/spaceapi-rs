use serde_json::from_str;
use spaceapi::Status;

fn main() {
    let v13_input = r#"{"api":"0.13","space":"coredump","logo":"https://www.coredump.ch/logo.png","url":"https://www.coredump.ch/","location":{"lat":47.22936,"lon":8.82949},"contact":{"irc":"irc://freenode.net/#coredump","twitter":"@coredump_ch","foursquare":"525c20e5498e875d8231b1e5","email":"danilo@coredump.ch"},"issue_report_channels":["email","twitter"],"state":{"open":null},"ext_ccc":"chaostreff"}"#;
    let status: Result<Status, _> = from_str(v13_input);
    match status {
        Ok(status) => println!("{:?}", status),
        Err(err) => println!("Could not parse status: {}", err),
    };

    let v14_input = r#"{"api_compatibility":["14"],"space":"coredump","logo":"https://www.coredump.ch/logo.png","url":"https://www.coredump.ch/","location":{"lat":47.22936,"lon":8.82949},"contact":{"irc":"irc://freenode.net/#coredump","twitter":"@coredump_ch","foursquare":"525c20e5498e875d8231b1e5","email":"danilo@coredump.ch"},"ext_ccc":"chaostreff"}"#;
    let status: Result<Status, _> = from_str(v14_input);
    match status {
        Ok(status) => println!("{:?}", status),
        Err(err) => println!("Could not parse status: {}", err),
    };
}
