use std::collections::{HashMap, BTreeMap};

use serde::ser::{Serializer, Serialize};
use serde::de::{Deserializer, Deserialize, Visitor, MapVisitor, Error as SerdeError};
use serde_json::value::Value;

pub use sensors::SensorTemplate;
pub use sensors::Sensors;
pub use sensors::TemperatureSensor;
pub use sensors::PeopleNowPresentSensor;

type Extensions = BTreeMap<String, Value>;

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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Status {

    // Hackerspace properties
    pub api: String,
    pub space: String,
    pub logo: String,
    pub url: String,
    pub location: Location,
    pub contact: Contact,

    // Hackerspace features / projects
    pub spacefed: Option<Spacefed>,
    pub projects: Option<Vec<String>>,
    pub cam: Option<Vec<String>>,
    pub feeds: Option<Feeds>,
    pub events: Option<Vec<Event>>,
    pub radio_show: Option<Vec<RadioShow>>,

    // SpaceAPI internal usage
    pub cache: Option<Cache>,
    pub issue_report_channels: Vec<String>,

    // Mutable data
    pub state: State,
    pub sensors: Option<Sensors>,

    // Version extension
    pub ext_versions: Option<HashMap<String, String>>,

    // Custom extensions are allowed and will be prefixed with `ext_`.
    pub extensions: Extensions,
}

impl Status {
    /// Create a new Status object with only the absolutely required fields.
    #[deprecated(since="0.5.0",
                 note="Please use the `StatusBuilder` or a struct expression instead")]
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

impl Serialize for Status {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {

        // Determine number of fields to serialize
        let mut field_count = 8;
        if self.spacefed.is_some() { field_count += 1; }
        if self.projects.is_some() { field_count += 1; }
        if self.cam.is_some() { field_count += 1; }
        if self.feeds.is_some() { field_count += 1; }
        if self.events.is_some() { field_count += 1; }
        if self.radio_show.is_some() { field_count += 1; }
        if self.cache.is_some() { field_count += 1; }
        if self.sensors.is_some() { field_count += 1; }
        if self.ext_versions.is_some() { field_count += 1; }
        field_count += self.extensions.len();

        // Serialize fields
        let mut state = serializer.serialize_map(Some(field_count))?;
        macro_rules! serialize {
            ($field:expr, $field_name:expr) => {
                serializer.serialize_map_key(&mut state, $field_name)?;
                serializer.serialize_map_value(&mut state, &$field)?;
            };
        }
        macro_rules! maybe_serialize {
            ($field:expr, $field_name:expr) => {
                if let Some(ref val) = $field {
                    serializer.serialize_map_key(&mut state, $field_name)?;
                    serializer.serialize_map_value(&mut state, &val)?;
                }
            };
        }
        serialize!(self.api, "api");
        serialize!(self.space, "space");
        serialize!(self.logo, "logo");
        serialize!(self.url, "url");
        serialize!(self.location, "location");
        serialize!(self.contact, "contact");
        maybe_serialize!(self.spacefed, "spacefed");
        maybe_serialize!(self.projects, "projects");
        maybe_serialize!(self.cam, "cam");
        maybe_serialize!(self.feeds, "feeds");
        maybe_serialize!(self.events, "events");
        maybe_serialize!(self.radio_show, "radio_show");
        maybe_serialize!(self.cache, "cache");
        serialize!(self.issue_report_channels, "issue_report_channels");
        serialize!(self.state, "state");
        maybe_serialize!(self.sensors, "sensors");
        maybe_serialize!(self.ext_versions, "ext_versions");

        // Serialize extensions
        for (name, value) in self.extensions.iter() {
            serializer.serialize_map_key(&mut state, format!("ext_{}", name))?;
            serializer.serialize_map_value(&mut state, value)?;
        }

        // Finalize
        serializer.serialize_map_end(state)
    }
}


impl Deserialize for Status {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        enum Field {
            Api,
            Space,
            Logo,
            Url,
            Location,
            Contact,
            Spacefed,
            Projects,
            Cam,
            Feeds,
            Events,
            RadioShow,
            Cache,
            IssueReportChanels,
            State,
            Sensors,
            ExtVersions,
            Extension(String),
        };

        impl Deserialize for Field {
            fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Field, D::Error> {
                struct FieldVisitor;

                impl Visitor for FieldVisitor {
                    type Value = Field;

                    fn visit_str<E: SerdeError>(&mut self, value: &str) -> Result<Field, E> {
                        match value {
                            "api" => Ok(Field::Api),
                            "space" => Ok(Field::Space),
                            "logo" => Ok(Field::Logo),
                            "url" => Ok(Field::Url),
                            "location" => Ok(Field::Location),
                            "contact" => Ok(Field::Contact),
                            "spacefed" => Ok(Field::Spacefed),
                            "projects" => Ok(Field::Projects),
                            "cam" => Ok(Field::Cam),
                            "feeds" => Ok(Field::Feeds),
                            "events" => Ok(Field::Events),
                            "radio_show" => Ok(Field::RadioShow),
                            "cache" => Ok(Field::Cache),
                            "issue_report_channels" => Ok(Field::IssueReportChanels),
                            "state" => Ok(Field::State),
                            "sensors" => Ok(Field::Sensors),
                            "ext_versions" => Ok(Field::ExtVersions),
                            _ => {
                                if value.starts_with("ext_") {
                                    Ok(Field::Extension(
                                        value.trim_left_matches("ext_").to_owned()
                                    ))
                                } else {
                                    Err(SerdeError::unknown_field(value))
                                }
                            }
                        }
                    }
                }
                deserializer.deserialize_struct_field(FieldVisitor)
            }
        }

        struct StatusVisitor;

        impl Visitor for StatusVisitor {
            type Value = Status;

            fn visit_map<V: MapVisitor>(&mut self, mut visitor: V) -> Result<Status, V::Error> {
                macro_rules! visit_map_field {
                    ($field:ident, $field_name:expr) => {
                        {
                            if $field.is_some() {
                                return Err(SerdeError::duplicate_field($field_name));
                            }
                            $field = Some(visitor.visit_value()?);
                        }
                    };
                }
                macro_rules! process_map_field {
                    ($field:ident, $field_name:expr) => {
                        match $field {
                            Some(val) => val,
                            None => return Err(SerdeError::missing_field($field_name)),
                        };
                    };
                }
                let mut api: Option<String> = None;
                let mut space: Option<String> = None;
                let mut logo: Option<String> = None;
                let mut url: Option<String> = None;
                let mut location: Option<Location> = None;
                let mut contact: Option<Contact> = None;
                let mut spacefed: Option<Option<Spacefed>> = None;
                let mut projects: Option<Option<Vec<String>>> = None;
                let mut cam: Option<Option<Vec<String>>> = None;
                let mut feeds: Option<Option<Feeds>> = None;
                let mut events: Option<Option<Vec<Event>>> = None;
                let mut radio_show: Option<Option<Vec<RadioShow>>> = None;
                let mut cache: Option<Option<Cache>> = None;
                let mut issue_report_channels: Option<Vec<String>> = None;
                let mut state: Option<State> = None;
                let mut sensors: Option<Option<Sensors>> = None;
                let mut ext_versions: Option<Option<HashMap<String, String>>> = None;
                let mut extensions = Extensions::new();
                while let Some(key) = visitor.visit_key()? {
                    match key {
                        Field::Api => visit_map_field!(api, "api"),
                        Field::Space => visit_map_field!(space, "space"),
                        Field::Logo => visit_map_field!(logo, "logo"),
                        Field::Url => visit_map_field!(url, "url"),
                        Field::Location => visit_map_field!(location, "location"),
                        Field::Contact => visit_map_field!(contact, "contact"),
                        Field::Spacefed => visit_map_field!(spacefed, "spacefed"),
                        Field::Projects => visit_map_field!(projects, "projects"),
                        Field::Cam => visit_map_field!(cam, "cam"),
                        Field::Feeds => visit_map_field!(feeds, "feeds"),
                        Field::Events => visit_map_field!(events, "events"),
                        Field::RadioShow => visit_map_field!(radio_show, "radio_show"),
                        Field::Cache => visit_map_field!(cache, "cache"),
                        Field::IssueReportChanels => visit_map_field!(issue_report_channels, "issue_report_channels"),
                        Field::State => visit_map_field!(state, "state"),
                        Field::Sensors => visit_map_field!(sensors, "sensors"),
                        Field::ExtVersions => visit_map_field!(ext_versions, "ext_versions"),
                        Field::Extension(name) => {
                            let value: Value = visitor.visit_value()?;
                            extensions.insert(name, value);
                        }
                    }
                }
                visitor.end()?;
                Ok(Status {
                    api: process_map_field!(api, "api"),
                    space: process_map_field!(space, "space"),
                    logo: process_map_field!(logo, "logo"),
                    url: process_map_field!(url, "url"),
                    location: process_map_field!(location, "location"),
                    contact: process_map_field!(contact, "contact"),
                    spacefed: spacefed.unwrap_or(None),
                    projects: projects.unwrap_or(None),
                    cam: cam.unwrap_or(None),
                    feeds: feeds.unwrap_or(None),
                    events: events.unwrap_or(None),
                    radio_show: radio_show.unwrap_or(None),
                    cache: cache.unwrap_or(None),
                    issue_report_channels: process_map_field!(issue_report_channels, "issue_report_channels"),
                    state: process_map_field!(state, "state"),
                    sensors: sensors.unwrap_or(None),
                    ext_versions: ext_versions.unwrap_or(None),
                    extensions: extensions,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "api", "space", "logo", "url", "location", "contact", "spacefed",
            "projects", "cam", "feeds", "events", "radio_show", "cache",
            "issue_report_channels", "state", "sensors", "ext_versions", "extensions",
        ];
        deserializer.deserialize_struct("Status", FIELDS, StatusVisitor)
    }
}

#[derive(Default, Debug, Clone)]
pub struct StatusBuilder {
    space: String,
    logo: Option<String>,
    url: Option<String>,
    location: Option<Location>,
    contact: Option<Contact>,
    spacefed: Option<Spacefed>,
    projects: Option<Vec<String>>,
    cam: Option<Vec<String>>,
    feeds: Option<Feeds>,
    events: Option<Vec<Event>>,
    radio_show: Option<Vec<RadioShow>>,
    issue_report_channels: Vec<String>,
    extensions: Extensions,
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

    pub fn spacefed(mut self, spacefed: Spacefed) -> Self {
        self.spacefed = Some(spacefed);
        self
    }

    pub fn add_event(mut self, event: Event) -> Self {
        if let Some(ref mut events) = self.events {
            events.push(event);
        } else {
            self.events = Some(vec![event]);
        }
        self
    }

    pub fn add_cam<S: Into<String>>(mut self, cam: S) -> Self {
        if let Some(ref mut cams) = self.cam {
            cams.push(cam.into());
        } else {
            self.cam = Some(vec![cam.into()]);
        }
        self
    }

    pub fn feeds(mut self, feeds: Feeds) -> Self {
        self.feeds = Some(feeds);
        self
    }

    pub fn add_radio_show(mut self, radio_show: RadioShow) -> Self {
        if let Some(ref mut radio_shows) = self.radio_show {
            radio_shows.push(radio_show);
        } else {
            self.radio_show = Some(vec![radio_show]);
        }
        self
    }

    pub fn add_project<S: Into<String>>(mut self, project: S) -> Self {
        if let Some(ref mut projects) = self.projects {
            projects.push(project.into());
        } else {
            self.projects = Some(vec![project.into()]);
        }
        self
    }

    pub fn add_issue_report_channel<S: Into<String>>(mut self, report_channel: S) -> Self {
        self.issue_report_channels.push(report_channel.into());
        self
    }

    /// Add an extension to the `Status` object.
    ///
    /// The prefix `ext_` will automatically be prepended to the name during
    /// serialization, if not already present.
    ///
    /// TODO Serde 0.9: Replace `Value` parameter with `V: Into<Value>` and
    /// `update examples/serialization.rs`.
    pub fn add_extension(mut self, name: &str, value: Value) -> Self {
        self.extensions.insert(name.trim_left_matches("ext_").to_owned(), value);
        self
    }

    pub fn build(self) -> Result<Status, String> {
        Ok(Status {
            api: "0.13".into(), // TODO: Deduplicate
            space: self.space,
            logo: self.logo.ok_or("logo missing")?,
            url: self.url.ok_or("url missing")?,
            location: self.location.ok_or("location missing")?,
            contact: self.contact.ok_or("contact missing")?,
            spacefed: self.spacefed,
            projects: self.projects,
            cam: self.cam,
            feeds: self.feeds,
            events: self.events,
            radio_show: self.radio_show,
            issue_report_channels: self.issue_report_channels,
            extensions: self.extensions,
            ..Default::default()
        })
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
            .location(Location::default())
            .contact(Contact::default())
            .spacefed(Spacefed::default())
            .feeds(Feeds::default())
            .add_project("spaceapi-rs")
            .add_cam("cam1")
            .add_cam("cam2".to_string())
            .add_event(Event::default())
            .add_issue_report_channel("chan")
            .build()
            .unwrap();
        assert_eq!(status.api, "0.13");
        assert_eq!(status.space, "foo");
        assert_eq!(status.logo, "bar");
        assert_eq!(status.url, "foobar");
        assert_eq!(status.location, Location::default());
        assert_eq!(status.contact, Contact::default());
        assert_eq!(status.spacefed, Some(Spacefed::default()));
        assert_eq!(status.feeds, Some(Feeds::default()));
        assert_eq!(status.projects, Some(vec!["spaceapi-rs".to_string()]));
        assert_eq!(status.cam, Some(vec!["cam1".to_string(), "cam2".to_string()]));
        assert_eq!(status.events, Some(vec![Event::default()]));
        assert_eq!(status.issue_report_channels, vec!["chan".to_string()]);
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

    #[test]
    fn serialize_deserialize_full() {
        let status = StatusBuilder::new("foo")
            .logo("bar")
            .url("foobar")
            .location(Location::default())
            .contact(Contact::default())
            .add_extension("aaa", Value::Array(vec![Value::Null, Value::U64(42)]))
            .build()
            .unwrap();
        let serialized = to_string(&status).unwrap();
        let deserialized = from_str::<Status>(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn serialize_extension_fields_empty() {
        let status = StatusBuilder::new("a")
            .logo("b")
            .url("c")
            .location(Location::default())
            .contact(Contact::default())
            .build();
        assert!(status.is_ok());
        assert_eq!(
            &to_string(&status.unwrap()).unwrap(),
            "{\"api\":\"0.13\",\"space\":\"a\",\"logo\":\"b\",\"url\":\"c\",\
            \"location\":{\"lat\":0.0,\"lon\":0.0},\"contact\":{},\"issue_report_channels\":[],\
            \"state\":{\"open\":null}}"
        );
    }

    #[test]
    fn serialize_extension_fields() {
        let status = StatusBuilder::new("a")
            .logo("b")
            .url("c")
            .location(Location::default())
            .contact(Contact::default())
            .add_extension("aaa", Value::String("xxx".into()))
            .add_extension("bbb", Value::Array(vec![Value::Null, Value::U64(42)]))
            .build();
        assert!(status.is_ok());
        assert_eq!(
            &to_string(&status.unwrap()).unwrap(),
            "{\"api\":\"0.13\",\"space\":\"a\",\"logo\":\"b\",\"url\":\"c\",\
            \"location\":{\"lat\":0.0,\"lon\":0.0},\"contact\":{},\"issue_report_channels\":[],\
            \"state\":{\"open\":null},\"ext_aaa\":\"xxx\",\"ext_bbb\":[null,42]}"
        );
    }

    #[test]
    fn serialize_deserialize_full_with_optional() {
        let mut status = StatusBuilder::new("foo")
            .logo("bar")
            .url("foobar")
            .location(Location::default())
            .contact(Contact::default())
            .build()
            .unwrap();
        status.spacefed = Some(Spacefed::default());
        status.feeds = Some(Feeds::default());
        status.projects = Some(vec![]);
        status.cam = Some(vec![]);

        let serialized = to_string(&status).unwrap();
        let deserialized = from_str::<Status>(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    /// Extension field names are automatically prepended with `ext_`.
    /// If two extensions with the same name (before or after prepending) are
    /// added, then the second call will overwrite the first one (standard Map
    /// behavior).
    #[test]
    fn prepend_ext_to_extension_field_names() {
        let status = StatusBuilder::new("a")
            .logo("b")
            .url("c")
            .location(Location::default())
            .contact(Contact::default())
            .add_extension("aaa", Value::String("xxx".into()))
            .add_extension("ext_aaa", Value::String("yyy".into()))
            .add_extension("bbb", Value::Null)
            .add_extension("ext_ccc", Value::Null)
            .build();
        assert!(status.is_ok());
        let serialized = to_string(&status.unwrap()).unwrap();
        assert!(serialized.contains("\"ext_aaa\":\"yyy\""));
        assert!(serialized.contains("\"ext_bbb\":null"));
        assert!(serialized.contains("\"ext_ccc\":null"));
    }

    #[test]
    fn deserialize() {
        let data = "{\"api\":\"0.13\",\"space\":\"a\",\"logo\":\"b\",\"url\":\"c\",\
            \"location\":{\"lat\":0.0,\"lon\":0.0},\"contact\":{},\"issue_report_channels\":[],\
            \"state\":{\"open\":null},\"ext_aaa\":\"xxx\",\"ext_bbb\":[null,42]}";
        let deserialized: Status = from_str(&data).unwrap();
        assert_eq!(&deserialized.api, "0.13");
        let keys: Vec<_> = deserialized.extensions.keys().collect();
        assert_eq!(keys.len(), 2)
    }

}
