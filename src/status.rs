use std::collections::{BTreeMap, HashMap};
use rustc_serialize::json::{Json, ToJson};
pub use optional::Optional;
pub use sensors::SensorTemplate;
pub use sensors::Sensors;
pub use sensors::TemperatureSensor;
pub use sensors::PeopleNowPresentSensor;


#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct Location {
    pub address: Optional<String>,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct Spacefed {
    pub spacenet: bool,
    pub spacesaml: bool,
    pub spacephone: bool,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct Icon {
    pub open: String,
    pub close: String,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct State {
    pub open: Option<bool>,
    pub lastchange: Optional<u64>,
    pub trigger_person: Optional<String>,
    pub message: Optional<String>,
    pub icon: Optional<Icon>,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct Event {
    pub name: String,
    pub _type: String,
    pub timestamp: u64,
    pub extra: Optional<String>,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct Keymaster {
    pub name: Optional<String>,
    pub irc_nick: Optional<String>,
    pub phone: Optional<String>,
    pub email: Optional<String>,
    pub twitter: Optional<String>,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct GoogleContact {
    pub plus: Optional<String>,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct Contact {
    pub phone: Optional<String>,
    pub sip: Optional<String>,
    pub keymasters: Optional<Vec<Keymaster>>,
    pub irc: Optional<String>,
    pub twitter: Optional<String>,
    pub facebook: Optional<String>,
    pub google: Optional<GoogleContact>,
    pub identica: Optional<String>,
    pub foursquare: Optional<String>,
    pub email: Optional<String>,
    pub ml: Optional<String>,
    pub jabber: Optional<String>,
    pub issue_mail: Optional<String>,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct Feed {
    pub _type: Optional<String>,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct Feeds {
    pub blog: Optional<Feed>,
    pub wiki: Optional<Feed>,
    pub calendar: Optional<Feed>,
    pub flickr: Optional<Feed>,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct Cache {
    pub schedule: String,
}

#[derive(Debug, Clone, PartialEq, RustcDecodable)]
pub struct RadioShow {
    pub name: String,
    pub url: String,
    pub _type: String,
    pub start: String,
    pub end: String,
}

/// The main Space API status object.
#[derive(Debug, Clone, PartialEq, RustcDecodable)]
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

    // Version extension
    // TODO: Once we move to serde, maybe we can store this
    // as `HashMap<String, &'static str>`?
    pub ext_versions: Optional<HashMap<String, String>>,
}

impl Status {

    /// Create a new Status object with only the absolutely required fields.
    pub fn new<S: Into<String>>(space: S, logo: S, url: S, location: Location, contact: Contact, issue_report_channels: Vec<String>) -> Status {
        Status {
            api: "0.13".into(),
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

            ext_versions: Optional::Absent,
        }
    }

}

impl ToJson for Status {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();

        d.insert("api".into(), self.api.to_json());
        d.insert("space".into(), self.space.to_json());
        d.insert("logo".into(), self.logo.to_json());
        d.insert("url".into(), self.url.to_json());
        d.insert("location".into(), self.location.to_json());
        d.insert("contact".into(), self.contact.to_json());

        self.spacefed.as_ref().map_or((), |v| { d.insert("spacefed".into(), v.to_json()); } );
        self.projects.as_ref().map_or((), |v| { d.insert("projects".into(), v.to_json()); });
        self.cam.as_ref().map_or((), |v| { d.insert("cam".into(), v.to_json()); });
        self.feeds.as_ref().map_or((), |v| { d.insert("feeds".into(), v.to_json()); });
        self.events.as_ref().map_or((), |v| { d.insert("events".into(), v.to_json()); });
        self.radio_show.as_ref().map_or((), |v| { d.insert("radio_show".into(), v.to_json()); });

        self.cache.as_ref().map_or((), |v| { d.insert("cache".into(), v.to_json()); });
        d.insert("issue_report_channels".into(), self.issue_report_channels.to_json());

        d.insert("state".into(), self.state.to_json());
        self.sensors.as_ref().map_or((), |v| { d.insert("sensors".into(), v.to_json()); });

        if let Optional::Value(ref versions) = self.ext_versions {
            d.insert("ext_versions".into(), versions.to_json());
        }

        Json::Object(d)
    }
}

impl ToJson for Location {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("lat".into(), self.lat.to_json());
        d.insert("lon".into(), self.lon.to_json());
        match self.address {
            Optional::Value(ref address) => {
                d.insert("address".into(), address.to_json());
            },
            Optional::Absent => (),
        };
        Json::Object(d)
    }
}

impl ToJson for Spacefed {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("spacenet".into(), self.spacenet.to_json());
        d.insert("spacesaml".into(), self.spacesaml.to_json());
        d.insert("spacephone".into(), self.spacephone.to_json());
        Json::Object(d)
    }
}

impl ToJson for Icon {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("open".into(), self.open.to_json());
        d.insert("close".into(), self.close.to_json());
        Json::Object(d)
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("open".into(), self.open.to_json());
        self.lastchange.as_ref().map_or((), |v| { d.insert("lastchange".into(), v.to_json()); });
        self.trigger_person.as_ref().map_or((), |v| { d.insert("trigger_person".into(), v.to_json()); });
        self.message.as_ref().map_or((), |v| { d.insert("message".into(), v.to_json()); });
        self.icon.as_ref().map_or((), |v| { d.insert("icon".into(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Event {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("name".into(), self.name.to_json());
        d.insert("type".into(), self._type.to_json());
        d.insert("timestamp".into(), self.timestamp.to_json());
        self.extra.as_ref().map_or((), |v| { d.insert("extra".into(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Keymaster {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        self.name.as_ref().map_or((), |v| { d.insert("name".into(), v.to_json()); });
        self.irc_nick.as_ref().map_or((), |v| { d.insert("irc_nick".into(), v.to_json()); });
        self.phone.as_ref().map_or((), |v| { d.insert("phone".into(), v.to_json()); });
        self.email.as_ref().map_or((), |v| { d.insert("email".into(), v.to_json()); });
        self.twitter.as_ref().map_or((), |v| { d.insert("twitter".into(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for GoogleContact {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        self.plus.as_ref().map_or((), |v| { d.insert("plus".into(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Contact {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        self.phone.as_ref().map_or((), |v| { d.insert("phone".into(), v.to_json()); });
        self.sip.as_ref().map_or((), |v| { d.insert("sip".into(), v.to_json()); });
        self.keymasters.as_ref().map_or((), |v| { d.insert("keymasters".into(), v.to_json()); });
        self.irc.as_ref().map_or((), |v| { d.insert("irc".into(), v.to_json()); });
        self.twitter.as_ref().map_or((), |v| { d.insert("twitter".into(), v.to_json()); });
        self.facebook.as_ref().map_or((), |v| { d.insert("facebook".into(), v.to_json()); });
        self.google.as_ref().map_or((), |v| { d.insert("google".into(), v.to_json()); });
        self.identica.as_ref().map_or((), |v| { d.insert("identica".into(), v.to_json()); });
        self.foursquare.as_ref().map_or((), |v| { d.insert("foursquare".into(), v.to_json()); });
        self.email.as_ref().map_or((), |v| { d.insert("email".into(), v.to_json()); });
        self.ml.as_ref().map_or((), |v| { d.insert("ml".into(), v.to_json()); });
        self.jabber.as_ref().map_or((), |v| { d.insert("jabber".into(), v.to_json()); });
        self.issue_mail.as_ref().map_or((), |v| { d.insert("issue_mail".into(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Feed {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("url".into(), self.url.to_json());
        self._type.as_ref().map_or((), |v| { d.insert("type".into(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Feeds {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        self.blog.as_ref().map_or((), |v| { d.insert("blog".into(), v.to_json()); });
        self.wiki.as_ref().map_or((), |v| { d.insert("wiki".into(), v.to_json()); });
        self.calendar.as_ref().map_or((), |v| { d.insert("calendar".into(), v.to_json()); });
        self.flickr.as_ref().map_or((), |v| { d.insert("flickr".into(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for Cache {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("schedule".into(), self.schedule.to_json());
        Json::Object(d)
    }
}

impl ToJson for RadioShow {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("name".into(), self.name.to_json());
        d.insert("url".into(), self.url.to_json());
        d.insert("type".into(), self._type.to_json());
        d.insert("start".into(), self.start.to_json()); d.insert("end".into(), self.end.to_json());
        Json::Object(d)
    }
}

#[cfg(test)]
mod test {
  use super::*;
  use optional::Optional::*;
  use rustc_serialize::json::{self, ToJson};
  
  #[test]
  fn serialize_deserialize_cache() {
    let a: Cache = Cache { schedule: "bla".into() };
    let b: Cache = json::decode(&a.to_json().to_string()).unwrap();
    
    assert_eq!(a.schedule, b.schedule);
  }
  
  #[test]
  fn serialize_deserialize_simple_contact() {
    let a: Contact = Contact{
      phone: Absent,
      sip: Absent,
      keymasters: Value(vec![
          Keymaster {
              name: Value("Joe".into()),
              irc_nick: Absent,
              phone: Absent,
              email: Value("joe@example.com".into()),
              twitter: Absent,
          },
      ]),
      irc: Value("bla".into()),
      twitter: Absent,
      facebook: Absent,
      google: Value(GoogleContact { plus: Value("http://gplus/profile".into()) }),
      identica: Absent,
      foursquare: Absent,
      email: Value("bli@bla".into()),
      ml: Absent,
      jabber: Absent,
      issue_mail: Absent,
    };
    let b: Contact = json::decode(&a.to_json().to_string()).unwrap();
    
    assert_eq!(a.irc, b.irc);
    assert_eq!(a.twitter, b.twitter);
    assert_eq!(a.foursquare, b.foursquare);
    assert_eq!(a.email, b.email);
    assert_eq!(a.sip, b.sip);
    assert_eq!(a.ml, b.ml);

    assert_eq!(a.google, b.google);
    assert_eq!(a.keymasters, b.keymasters);
  }

}
