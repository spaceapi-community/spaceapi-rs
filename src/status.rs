use std::collections::BTreeMap;
use std::fmt;

use serde::ser::{Serializer, Serialize, SerializeMap};
use serde::de::{self, Deserializer, Deserialize, Visitor, MapAccess};
use serde_json::value::Value;

pub use crate::sensors::SensorTemplate;
pub use crate::sensors::Sensors;
pub use crate::sensors::TemperatureSensor;
pub use crate::sensors::PeopleNowPresentSensor;

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
    #[serde(rename = "type")]
    pub type_: String,
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
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
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
    #[serde(rename = "type")]
    pub type_: String,
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueReportChannel {
    Email,
    IssueMail,
    Twitter,
    Ml,
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
    pub issue_report_channels: Vec<IssueReportChannel>,

    // Mutable data
    pub state: State,
    pub sensors: Option<Sensors>,

    // Custom extensions are allowed and will be prefixed with `ext_`.
    pub extensions: Extensions,
}

impl Status {
    /// Create a new Status object with only the absolutely required fields.
    #[deprecated(since="0.5.0",
                 note="Please use the `StatusBuilder` or a struct expression instead")]
    pub fn new<S: Into<String>>(space: S, logo: S, url: S, location: Location, contact: Contact,
                                issue_report_channels: Vec<IssueReportChannel>) -> Status {
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
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {

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
        field_count += self.extensions.len();

        // Serialize fields
        let mut state = serializer.serialize_map(Some(field_count))?;
        macro_rules! serialize {
            ($field:expr, $field_name:expr) => {
                state.serialize_entry($field_name, &$field)?;
            };
        }
        macro_rules! maybe_serialize {
            ($field:expr, $field_name:expr) => {
                if let Some(ref val) = $field {
                    state.serialize_entry($field_name, &val)?;
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

        // Serialize extensions
        for (name, value) in self.extensions.iter() {
            state.serialize_entry(&format!("ext_{}", name), &value)?;
        }

        // Finalize
        state.end()
    }
}


impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
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
            IssueReportChannels,
            State,
            Sensors,
            Extension(String),
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where D: Deserializer<'de>
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        write!(formatter, "a valid field name")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where E: de::Error
                    {
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
                            "issue_report_channels" => Ok(Field::IssueReportChannels),
                            "state" => Ok(Field::State),
                            "sensors" => Ok(Field::Sensors),
                            _ => {
                                if value.starts_with("ext_") {
                                    Ok(Field::Extension(
                                        value.trim_left_matches("ext_").to_owned()
                                    ))
                                } else {
                                    Err(de::Error::unknown_field(value, &FIELDS))
                                }
                            }
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct StatusVisitor;

        impl<'de> Visitor<'de> for StatusVisitor {
            type Value = Status;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a valid status object")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Status, V::Error>
                where V: MapAccess<'de>
            {
                macro_rules! visit_map_field {
                    ($field:ident, $field_name:expr) => {
                        {
                            if $field.is_some() {
                                return Err(de::Error::duplicate_field($field_name));
                            }
                            $field = Some(map.next_value()?);
                        }
                    };
                }
                macro_rules! process_map_field {
                    ($field:ident, $field_name:expr) => {
                        match $field {
                            Some(val) => val,
                            None => return Err(de::Error::missing_field($field_name)),
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
                let mut issue_report_channels: Option<Vec<IssueReportChannel>> = None;
                let mut state: Option<State> = None;
                let mut sensors: Option<Option<Sensors>> = None;
                let mut extensions = Extensions::new();
                while let Some(key) = map.next_key()? {
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
                        Field::IssueReportChannels => visit_map_field!(issue_report_channels, "issue_report_channels"),
                        Field::State => visit_map_field!(state, "state"),
                        Field::Sensors => visit_map_field!(sensors, "sensors"),
                        Field::Extension(name) => {
                            let value: Value = map.next_value()?;
                            extensions.insert(name, value);
                        }
                    }
                }
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
                    extensions: extensions,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "api", "space", "logo", "url", "location", "contact", "spacefed",
            "projects", "cam", "feeds", "events", "radio_show", "cache",
            "issue_report_channels", "state", "sensors", "extensions",
        ];
        deserializer.deserialize_struct("Status", FIELDS, StatusVisitor)
    }
}

/// Builder for the `Status` object.
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
    issue_report_channels: Vec<IssueReportChannel>,
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
        self.events.get_or_insert(vec![]).push(event);
        self
    }

    pub fn add_cam<S: Into<String>>(mut self, cam: S) -> Self {
        self.cam.get_or_insert(vec![]).push(cam.into());
        self
    }

    pub fn feeds(mut self, feeds: Feeds) -> Self {
        self.feeds = Some(feeds);
        self
    }

    pub fn add_radio_show(mut self, radio_show: RadioShow) -> Self {
        self.radio_show.get_or_insert(vec![]).push(radio_show);
        self
    }

    pub fn add_project<S: Into<String>>(mut self, project: S) -> Self {
        self.projects.get_or_insert(vec![]).push(project.into());
        self
    }

    pub fn add_issue_report_channel(mut self, report_channel: IssueReportChannel) -> Self {
        self.issue_report_channels.push(report_channel);
        self
    }

    /// Add an extension to the `Status` object.
    ///
    /// The prefix `ext_` will automatically be prepended to the name during
    /// serialization, if not already present.
    pub fn add_extension<V: Into<Value>>(mut self, name: &str, value: V) -> Self {
        self.extensions.insert(name.trim_left_matches("ext_").to_owned(), value.into());
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
            .add_issue_report_channel(IssueReportChannel::Email)
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
        assert_eq!(status.issue_report_channels, vec![IssueReportChannel::Email]);
    }

    #[test]
    fn serialize_skip_none() {
        let f1 = Feed {
            type_: Some("rss".to_string()),
            url: "https://some/rss.xml".to_string(),
        };
        let f2 = Feed {
            type_: None,
            url: "https://some/rss.xml".to_string(),
        };
        assert_eq!(to_string(&f1).unwrap(),
            "{\"type\":\"rss\",\"url\":\"https://some/rss.xml\"}".to_string());
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
            .add_extension("aaa", Value::Array(vec![Value::Null, Value::from(42)]))
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
            .add_extension("bbb", Value::Array(vec![Value::Null, Value::from(42)]))
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
    fn deserialize_status() {
        let data = "{\"api\":\"0.13\",\"space\":\"a\",\"logo\":\"b\",\"url\":\"c\",\
            \"location\":{\"lat\":0.0,\"lon\":0.0},\"contact\":{},\"issue_report_channels\":[],\
            \"state\":{\"open\":null},\"ext_aaa\":\"xxx\",\"ext_bbb\":[null,42]}";
        let deserialized: Status = from_str(&data).unwrap();
        assert_eq!(&deserialized.api, "0.13");
        let keys: Vec<_> = deserialized.extensions.keys().collect();
        assert_eq!(keys.len(), 2)
    }

    mod serialize {
        use super::*;

        /// Macro to generate serialization test code
        macro_rules! test_serialize {
            ($test_name:ident, $value:expr, $expected:expr) => {
                #[test]
                fn $test_name() {
                    let serialized = to_string(&$value).unwrap();
                    assert_eq!(serialized, $expected);
                }
            }
        }

        test_serialize!(issue_report_channel_email, IssueReportChannel::Email, "\"email\"");
        test_serialize!(issue_report_channel_issue_mail, IssueReportChannel::IssueMail, "\"issue_mail\"");
        test_serialize!(issue_report_channel_twitter, IssueReportChannel::Twitter, "\"twitter\"");
        test_serialize!(issue_report_channel_ml, IssueReportChannel::Ml, "\"ml\"");
    }

    mod deserialize {
        use super::*;

        /// Macro to generate deserialization test code
        macro_rules! test_deserialize {
            ($test_name:ident, $value:expr, $type:ty, $expected:expr) => {
                #[test]
                fn $test_name() {
                    let deserialized = from_str::<$type>(&$value).unwrap();
                    assert_eq!(deserialized, $expected);
                }
            }
        }

        test_deserialize!(issue_report_channel_email, "\"email\"", IssueReportChannel, IssueReportChannel::Email);
        test_deserialize!(issue_report_channel_issue_mail, "\"issue_mail\"", IssueReportChannel, IssueReportChannel::IssueMail);
        test_deserialize!(issue_report_channel_twitter, "\"twitter\"", IssueReportChannel, IssueReportChannel::Twitter);
        test_deserialize!(issue_report_channel_ml, "\"ml\"", IssueReportChannel, IssueReportChannel::Ml);
    }
}
