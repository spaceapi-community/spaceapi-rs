pub use sensors::SensorTemplate;
pub use sensors::Sensors;
pub use sensors::TemperatureSensor;
pub use sensors::PeopleNowPresentSensor;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Spacefed {
    pub spacenet: bool,
    pub spacesaml: bool,
    pub spacephone: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Icon {
    pub open: String,
    pub close: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct State {
    pub open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastchange: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_person: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Event {
    pub name: String,
    pub _type: String,
    pub timestamp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Keymaster {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irc_nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct GoogleContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plus: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keymasters: Option<Vec<Keymaster>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google: Option<GoogleContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identica: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jabber: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_mail: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Feed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Feeds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog: Option<Feed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki: Option<Feed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar: Option<Feed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flickr: Option<Feed>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Cache {
    pub schedule: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct RadioShow {
    pub name: String,
    pub url: String,
    pub _type: String,
    pub start: String,
    pub end: String,
}

/// The main Space API status object.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Status {

    // Hackerspace properties
    pub api: String,
    pub space: String,
    pub logo: String,
    pub url: String,
    pub location: Location,
    pub contact: Contact,

    // Hackerspace features / projects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacefed: Option<Spacefed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cam: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeds: Option<Feeds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radio_show: Option<Vec<RadioShow>>,

    // SpaceAPI internal usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Cache>,
    pub issue_report_channels: Vec<String>,

    // Mutable data
    pub state: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensors: Option<Sensors>,

    // Version extension
    // TODO: Once we move to serde, maybe we can store this
    // as `HashMap<String, &'static str>`?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_versions: Option<HashMap<String, String>>,
}

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
            issue_report_channels: issue_report_channels,
            ..Default::default()
        }
    }

}

#[derive(Default, Debug, Clone)]
pub struct StatusBuilder {
    space: String,
    logo: Option<String>,
    url: Option<String>,
    location: Option<Location>,
    contact: Option<Contact>,
    issue_report_channels: Vec<String>,
}

impl StatusBuilder {
    pub fn new<S: Into<String>>(space_name: S) -> StatusBuilder {
        StatusBuilder {
            space: space_name.into(),
            ..Default::default()
        }
    }

    pub fn logo<S: Into<String>>(mut self, logo: S) -> Self {
        self.logo = Some(logo.into());
        self
    }

    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn contact(mut self, contact: Contact) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn add_issue_report_channel<S: Into<String>>(mut self, report_channel: S) -> Self {
        self.issue_report_channels.push(report_channel.into());
        self
    }

    pub fn build(self) -> Result<Status, String> {
        Ok(Status::new(
            self.space,
            self.logo.ok_or("logo missing")?,
            self.url.ok_or("url missing")?,
            self.location.ok_or("location missing")?,
            self.contact.ok_or("contact missing")?,
            self.issue_report_channels,
            ))
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
        let a: Contact = Contact {
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
            google: Some(GoogleContact { plus: Some("http://gplus/profile".into()) }),
            email: Some("bli@bla".into()),
            ..Default::default()
        };
        let b: Contact = from_str(&to_string(&a).unwrap()).unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn test_builder() {
        let status = StatusBuilder::new("foo")
            .logo("bar")
            .url("foobar")
            .location(Location{
                address: None,
                lat: 0.0,
                lon: 0.0,
            })
        .contact(Contact{
            ..Default::default()
        })
        .build();
        assert!(status.is_ok());
    }

    #[test]
    fn serialize_skip_none() {
        let f1 = Feed {
            _type: Some("rss".to_string()),
            url: "https://some/rss.xml".to_string(),
        };
        let f2 = Feed {
            _type: None,
            url: "https://some/rss.xml".to_string(),
        };
        assert_eq!(to_string(&f1).unwrap(),
            "{\"_type\":\"rss\",\"url\":\"https://some/rss.xml\"}".to_string());
        assert_eq!(to_string(&f2).unwrap(),
            "{\"url\":\"https://some/rss.xml\"}".to_string());
    }

}

