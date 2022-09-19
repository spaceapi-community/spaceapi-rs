use spaceapi::{Contact, IssueReportChannel, Location, State, StatusBuilder};

fn main() {
    let status = StatusBuilder::mixed("coredump")
        .logo("https://www.coredump.ch/logo.png")
        .url("https://www.coredump.ch/")
        .location(Location {
            address: None,
            lat: 47.22936,
            lon: 8.82949,
            ..Default::default()
        })
        .contact(Contact {
            irc: Some("irc://freenode.net/#coredump".into()),
            twitter: Some("@coredump_ch".into()),
            foursquare: Some("525c20e5498e875d8231b1e5".into()),
            email: Some("danilo@coredump.ch".into()),
            ..Default::default()
        })
        .add_issue_report_channel(IssueReportChannel::Email)
        .add_issue_report_channel(IssueReportChannel::Twitter)
        .add_extension("ccc", "chaostreff")
        .state(State::default())
        .build()
        .expect("Creating status failed");
    let stringstatus = serde_json::to_string(&status).unwrap();
    println!("{}", stringstatus);
}
