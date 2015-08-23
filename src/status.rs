use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};
pub use optional::Optional;
pub use sensors::{SensorTemplate, Sensors};
pub use sensors::{TemperatureSensor, PeopleNowPresentSensor};


#[derive(Debug, Clone)]
pub struct Location {
    pub address: Optional<String>,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone)]
pub struct Spacefed {
    pub spacenet: bool,
    pub spacesaml: bool,
    pub spacephone: bool,
}

#[derive(Debug, Clone)]
pub struct Icon {
    pub open: String,
    pub close: String,
}

#[derive(Debug, Clone)]
pub struct State {
    pub open: Option<bool>,
    pub lastchange: Optional<u64>,
    pub trigger_person: Optional<String>,
    pub message: Optional<String>,
    pub icon: Optional<Icon>,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub _type: String,
    pub timestamp: u64,
    pub extra: Optional<String>,
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub irc: Optional<String>,
    pub twitter: Optional<String>,
    pub foursquare: Optional<String>,
    pub email: Optional<String>,
}

#[derive(Debug, Clone)]
pub struct Feed {
    pub _type: Optional<String>,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Feeds {
    pub blog: Optional<Feed>,
    pub wiki: Optional<Feed>,
    pub calendar: Optional<Feed>,
    pub flickr: Optional<Feed>,
}

#[derive(Debug, Clone)]
pub struct Cache {
    pub schedule: String,
}

#[derive(Debug, Clone)]
pub struct RadioShow {
    pub name: String,
    pub url: String,
    pub _type: String,
    pub start: String,
    pub end: String,
}

/// The main Space API status object.
#[derive(Debug, Clone)]
pub struct Status {

    // Hackerspace properties
    pub api: &'static str,
    pub space: String,
    pub logo: String,
    pub url: String,
    pub location: Location,
    pub contact: Contact,

    // Hackerspace features / projects
    pub spacefed: Optional<Spacefed>,
    pub projects: Optional<Vec<String>>,
    pub cam: Optional<Vec<String>>,
    pub feeds: Optional<Feeds>,
    pub events: Optional<Vec<Event>>,
    pub radio_show: Optional<Vec<RadioShow>>,

    // SpaceAPI internal usage
    pub cache: Optional<Cache>,
    pub issue_report_channels: Vec<String>,

    // Mutable data
    pub state: State,
    pub sensors: Optional<Sensors>,
}

impl Status {

    /// Create a new Status object with only the minimum amount of fields.
    pub fn new<S: Into<String>>(space: S, logo: S, url: S, location: Location, contact: Contact, issue_report_channels: Vec<String>) -> Status {
        Status {
            api: "0.13",
            space: space.into(),
            logo: logo.into(),
            url: url.into(),
            location: location,
            contact: contact,

            spacefed: Optional::Absent,
            projects: Optional::Absent,
            cam: Optional::Absent,
            feeds: Optional::Absent,
            events: Optional::Absent,
            radio_show: Optional::Absent,

            cache: Optional::Absent,
            issue_report_channels: issue_report_channels,

            state: State {
                open: None,
                lastchange: Optional::Absent,
                trigger_person: Optional::Absent,
                message: Optional::Absent,
                icon: Optional::Absent,
            },
            sensors: Optional::Absent,
        }
    }

}

impl ToJson for Status {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();

        d.insert("api".to_string(), self.api.to_json());
        d.insert("space".to_string(), self.space.to_json());
        d.insert("logo".to_string(), self.logo.to_json());
        d.insert("url".to_string(), self.url.to_json());
        d.insert("location".to_string(), self.location.to_json());
        d.insert("contact".to_string(), self.contact.to_json());

        self.spacefed.as_ref().map_or((), |v| { d.insert("spacefed".to_string(), v.to_json()); } );
        self.projects.as_ref().map_or((), |v| { d.insert("projects".to_string(), v.to_json()); });
        self.cam.as_ref().map_or((), |v| { d.insert("cam".to_string(), v.to_json()); });
        self.feeds.as_ref().map_or((), |v| { d.insert("feeds".to_string(), v.to_json()); });
        self.events.as_ref().map_or((), |v| { d.insert("events".to_string(), v.to_json()); });
        self.radio_show.as_ref().map_or((), |v| { d.insert("radio_show".to_string(), v.to_json()); });

        self.cache.as_ref().map_or((), |v| { d.insert("cache".to_string(), v.to_json()); });
        d.insert("issue_report_channels".to_string(), self.issue_report_channels.to_json());

        d.insert("state".to_string(), self.state.to_json());
        self.sensors.as_ref().map_or((), |v| { d.insert("sensors".to_string(), v.to_json()); });

        Json::Object(d)
    }
}

impl ToJson for Location {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("lat".to_string(), self.lat.to_json());
        d.insert("lon".to_string(), self.lon.to_json());
        match self.address {
            Optional::Value(ref address) => {
                d.insert("address".to_string(), address.to_json());
            },
            Optional::Absent => (),
        };
        Json::Object(d)
    }
}

impl ToJson for Spacefed {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("spacenet".to_string(), self.spacenet.to_json());
        d.insert("spacesaml".to_string(), self.spacesaml.to_json());
        d.insert("spacephone".to_string(), self.spacephone.to_json());
        Json::Object(d)
    }
}

impl ToJson for Icon {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("open".to_string(), self.open.to_json());
        d.insert("close".to_string(), self.close.to_json());
        Json::Object(d)
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("open".to_string(), self.open.to_json());
        self.lastchange.as_ref().map_or((), |v| { d.insert("lastchange".to_string(), v.to_json()); });
        self.trigger_person.as_ref().map_or((), |v| { d.insert("trigger_person".to_string(), v.to_json()); });
        self.message.as_ref().map_or((), |v| { d.insert("message".to_string(), v.to_json()); });
        self.icon.as_ref().map_or((), |v| { d.insert("icon".to_string(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Event {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("name".to_string(), self.name.to_json());
        d.insert("type".to_string(), self._type.to_json());
        d.insert("timestamp".to_string(), self.timestamp.to_json());
        self.extra.as_ref().map_or((), |v| { d.insert("extra".to_string(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Contact {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        self.irc.as_ref().map_or((), |v| { d.insert("irc".to_string(), v.to_json()); });
        self.twitter.as_ref().map_or((), |v| { d.insert("twitter".to_string(), v.to_json()); });
        self.foursquare.as_ref().map_or((), |v| { d.insert("foursquare".to_string(), v.to_json()); });
        self.email.as_ref().map_or((), |v| { d.insert("email".to_string(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Feed {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("url".to_string(), self.url.to_json());
        self._type.as_ref().map_or((), |v| { d.insert("type".to_string(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Feeds {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        self.blog.as_ref().map_or((), |v| { d.insert("blog".to_string(), v.to_json()); });
        self.wiki.as_ref().map_or((), |v| { d.insert("wiki".to_string(), v.to_json()); });
        self.calendar.as_ref().map_or((), |v| { d.insert("calendar".to_string(), v.to_json()); });
        self.flickr.as_ref().map_or((), |v| { d.insert("flickr".to_string(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Cache {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("schedule".to_string(), self.schedule.to_json());
        Json::Object(d)
    }
}

impl ToJson for RadioShow {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("name".to_string(), self.name.to_json());
        d.insert("url".to_string(), self.url.to_json());
        d.insert("type".to_string(), self._type.to_json());
        d.insert("start".to_string(), self.start.to_json()); d.insert("end".to_string(), self.end.to_json());
        Json::Object(d)
    }
}
