include!(concat!(env!("OUT_DIR"), "/status.rs"));

impl Status {
    /// Create a new Status object with only the absolutely required fields.
    pub fn new<S: Into<String>>(space: S, logo: S, url: S, location: Location, contact: Contact,
                                issue_report_channels: Vec<String>) -> Status {
        Status {
            api: "0.13".into(),
            space: space.into(),
            logo: logo.into(),
            url: url.into(),
            location: location,
            contact: contact,

            spacefed: None,
            projects: None,
            cam: None,
            feeds: None,
            events: None,
            radio_show: None,

            cache: None,
            issue_report_channels: issue_report_channels,

            state: State {
                open: None,
                lastchange: None,
                trigger_person: None,
                message: None,
                icon: None,
            },
            sensors: None,

            ext_versions: None,
        }
    }

}

