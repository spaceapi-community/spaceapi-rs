extern crate rustc_serialize;

use std::collections::BTreeMap;

use rustc_serialize::json::{Json, ToJson};


/// An Optional value can contain Optional::Some<T> or Optional::Absent.
/// It is similar to an Option, but Optional::Absent means it will be
/// omitted when serialized.
#[derive(Debug, Copy, Clone)]
pub enum Optional<T> {
    Value(T),
    Absent,
}

impl<T> Optional<T> {
    pub fn map_or<U, F: FnOnce(T) -> U>(self, def: U, f: F) -> U {
        match self {
            Optional::Value(v) => f(v),
            Optional::Absent => def,
        }
    }

    pub fn as_mut<'r>(&'r mut self) -> Optional<&'r mut T> {
        match *self {
            Optional::Value(ref mut x) => Optional::Value(x),
            Optional::Absent => Optional::Absent
        }
    }

    pub fn as_ref<'r>(&'r self) -> Optional<&'r T> {
        match *self {
            Optional::Value(ref x) => Optional::Value(x),
            Optional::Absent => Optional::Absent
        }
    }
}

pub struct Location {
    pub address: Optional<String>,
    pub lat: f64,
    pub lon: f64,
}

pub struct Spacefed {
    pub spacenet: bool,
    pub spacesaml: bool,
    pub spacephone: bool,
}

pub struct Icon {
    pub open: String,
    pub close: String,
}

pub struct State {
    pub open: Option<bool>,
    pub lastchange: Optional<u64>,
    pub trigger_person: Optional<String>,
    pub message: Optional<String>,
    pub icon: Optional<Icon>,
}

pub struct Event {
    pub name: String,
    pub _type: String,
    pub timestamp: u64,
    pub extra: Optional<String>,
}

pub struct Contact {
    pub irc: Optional<String>,
    pub twitter: Optional<String>,
    pub foursquare: Optional<String>,
    pub email: Optional<String>,
}

pub struct Feed {
    pub _type: Optional<String>,
    pub url: String,
}

pub struct Feeds {
    pub blog: Optional<Feed>,
    pub wiki: Optional<Feed>,
    pub calendar: Optional<Feed>,
    pub flickr: Optional<Feed>,
}

pub struct Sensors {
    pub people_now_present: Optional<Vec<PeopleNowPresentSensor>>,
    pub temperature: Optional<Vec<TemperatureSensor>>,
}

pub struct PeopleNowPresentSensor {
    pub value: u32,
    pub location: Optional<String>,
    pub name: Optional<String>,
    pub names: Optional<Vec<String>>,
    pub description: Optional<String>,
}

pub struct TemperatureSensor {
    pub value: f32,
    pub unit: String,
    pub location: String,
    pub name: Optional<String>,
    pub description: Optional<String>,
}

pub struct Cache {
    pub schedule: String,
}

pub struct RadioShow {
    pub name: String,
    pub url: String,
    pub _type: String,
    pub start: String,
    pub end: String,
}

pub struct Status {

    // Hackerspace properties
    pub api: String,
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

    /// Create a new Status object with only the minimum amount of fields
    pub fn new(space: String, logo: String, url: String, location: Location, contact: Contact, issue_report_channels: Vec<String>) -> Status {
        Status {
            api: "0.13".to_string(),
            space: space,
            logo: logo,
            url: url,
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

        match self.spacefed {
            Optional::Value(ref spacefed) => { d.insert("spacefed".to_string(), spacefed.to_json()); },
            Optional::Absent => (),
        };
        match self.projects {
            Optional::Value(ref projects) => { d.insert("projects".to_string(), projects.to_json()); },
            Optional::Absent => (),
        };
        match self.cam {
            Optional::Value(ref cam) => { d.insert("cam".to_string(), cam.to_json()); },
            Optional::Absent => (),
        };
        match self.feeds {
            Optional::Value(ref feeds) => { d.insert("feeds".to_string(), feeds.to_json()); },
            Optional::Absent => (),
        };
        match self.events {
            Optional::Value(ref events) => { d.insert("events".to_string(), events.to_json()); },
            Optional::Absent => (),
        };
        match self.radio_show {
            Optional::Value(ref radio_show) => { d.insert("radio_show".to_string(), radio_show.to_json()); },
            Optional::Absent => (),
        };

        match self.cache {
            Optional::Value(ref cache) => { d.insert("cache".to_string(), cache.to_json()); },
            Optional::Absent => (),
        };
        d.insert("issue_report_channels".to_string(), self.issue_report_channels.to_json());

        d.insert("state".to_string(), self.state.to_json());
        match self.sensors {
            Optional::Value(ref sensors) => { d.insert("sensors".to_string(), sensors.to_json()); },
            Optional::Absent => (),
        };

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
        match self.lastchange {
            Optional::Value(ref lastchange) => { d.insert("lastchange".to_string(), lastchange.to_json()); },
            Optional::Absent => (),
        };
        match self.trigger_person {
            Optional::Value(ref trigger_person) => { d.insert("trigger_person".to_string(), trigger_person.to_json()); },
            Optional::Absent => (),
        };
        match self.message {
            Optional::Value(ref message) => { d.insert("message".to_string(), message.to_json()); },
            Optional::Absent => (),
        };
        match self.icon {
            Optional::Value(ref icon) => { d.insert("icon".to_string(), icon.to_json()); },
            Optional::Absent => (),
        };
        Json::Object(d)
    }
}

impl ToJson for Event {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("name".to_string(), self.name.to_json());
        d.insert("type".to_string(), self._type.to_json());
        d.insert("timestamp".to_string(), self.timestamp.to_json());
        match self.extra {
            Optional::Value(ref extra) => { d.insert("extra".to_string(), extra.to_json()); },
            Optional::Absent => (),
        };
        Json::Object(d)
    }
}

impl ToJson for Contact {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        match self.irc {
            Optional::Value(ref irc) => { d.insert("irc".to_string(), irc.to_json()); },
            Optional::Absent => (),
        };
        match self.twitter {
            Optional::Value(ref twitter) => { d.insert("twitter".to_string(), twitter.to_json()); },
            Optional::Absent => (),
        };
        match self.foursquare {
            Optional::Value(ref foursquare) => { d.insert("foursquare".to_string(), foursquare.to_json()); },
            Optional::Absent => (),
        };
        match self.email {
            Optional::Value(ref email) => { d.insert("email".to_string(), email.to_json()); },
            Optional::Absent => (),
        };
        Json::Object(d)
    }
}

impl ToJson for Feed {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("url".to_string(), self.url.to_json());
        match self._type {
            Optional::Value(ref _type) => { d.insert("type".to_string(), _type.to_json()); },
            Optional::Absent => (),
        };
        Json::Object(d)
    }
}

impl ToJson for Feeds {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        match self.blog {
            Optional::Value(ref blog) => { d.insert("blog".to_string(), blog.to_json()); },
            Optional::Absent => (),
        };
        match self.wiki {
            Optional::Value(ref wiki) => { d.insert("wiki".to_string(), wiki.to_json()); },
            Optional::Absent => (),
        };
        match self.calendar {
            Optional::Value(ref calendar) => { d.insert("calendar".to_string(), calendar.to_json()); },
            Optional::Absent => (),
        };
        match self.flickr {
            Optional::Value(ref flickr) => { d.insert("flickr".to_string(), flickr.to_json()); },
            Optional::Absent => (),
        };
        Json::Object(d)
    }
}

impl ToJson for Sensors {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        match self.people_now_present {
            Optional::Value(ref people_now_present) => { d.insert("people_now_present".to_string(), people_now_present.to_json()); },
            Optional::Absent => (),
        };
        match self.temperature {
            Optional::Value(ref temperature) => { d.insert("temperature".to_string(), temperature.to_json()); },
            Optional::Absent => (),
        };
        Json::Object(d)
    }
}

impl ToJson for PeopleNowPresentSensor {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("value".to_string(), self.value.to_json());
        match self.location {
            Optional::Value(ref location) => { d.insert("location".to_string(), location.to_json()); },
            Optional::Absent => (),
        };
        match self.name {
            Optional::Value(ref name) => { d.insert("name".to_string(), name.to_json()); },
            Optional::Absent => (),
        };
        match self.names {
            Optional::Value(ref names) => { d.insert("names".to_string(), names.to_json()); },
            Optional::Absent => (),
        };
        match self.description {
            Optional::Value(ref description) => { d.insert("description".to_string(), description.to_json()); },
            Optional::Absent => (),
        };
        Json::Object(d)
    }
}

impl ToJson for TemperatureSensor {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("value".to_string(), self.value.to_json());
        d.insert("unit".to_string(), self.unit.to_json());
        d.insert("location".to_string(), self.location.to_json());
        match self.name {
            Optional::Value(ref name) => { d.insert("name".to_string(), name.to_json()); },
            Optional::Absent => (),
        };
        match self.description {
            Optional::Value(ref description) => { d.insert("description".to_string(), description.to_json()); },
            Optional::Absent => (),
        };
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
        d.insert("start".to_string(), self.start.to_json());
        d.insert("end".to_string(), self.end.to_json());
        Json::Object(d)
    }
}
