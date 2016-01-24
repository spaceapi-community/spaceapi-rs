pub use sensors::SensorTemplate;
pub use sensors::Sensors;
pub use sensors::TemperatureSensor;
pub use sensors::PeopleNowPresentSensor;

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

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{to_string, from_str};

    #[test]
    fn serialize_deserialize_cache() {
        let a = Cache { schedule: "bla".into() };
        let b: Cache = from_str(&to_string(&a).unwrap()).unwrap();
        assert_eq!(a.schedule, b.schedule);
    }

    #[test]
    fn serialize_deserialize_simple_contact() {
        let a: Contact = Contact{
            phone: None,
            sip: None,
            keymasters: Some(vec![
                              Keymaster {
                                  name: Some("Joe".into()),
                                  irc_nick: None,
                                  phone: None,
                                  email: Some("joe@example.com".into()),
                                  twitter: None,
                              },
            ]),
            irc: Some("bla".into()),
            twitter: None,
            facebook: None,
            google: Some(GoogleContact { plus: Some("http://gplus/profile".into()) }),
            identica: None,
            foursquare: None,
            email: Some("bli@bla".into()),
            ml: None,
            jabber: None,
            issue_mail: None,
        };
        let b: Contact = from_str(&to_string(&a).unwrap()).unwrap();

        assert_eq!(a, b);
    }

}

