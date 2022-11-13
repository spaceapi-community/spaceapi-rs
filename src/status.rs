use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use crate::sensors::Sensors;

type Extensions = BTreeMap<String, Value>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub lat: f64,
    pub lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Spacefed {
    pub spacenet: bool,
    pub spacesaml: bool,
    pub spacephone: Option<bool>,
}

impl Spacefed {
    fn verify(&self, version: StatusBuilderVersion) -> Result<(), String> {
        if version == StatusBuilderVersion::V14 {
            if self.spacephone.is_some() {
                return Err("spacefed.spacephone key was removed".into());
            }
        } else if self.spacephone.is_none() {
            return Err("spacefed.spacephone must be present".into());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Icon {
    pub open: String,
    pub closed: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
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

impl State {
    fn verify(&self, _version: StatusBuilderVersion) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub timestamp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xmpp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastodon: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct GoogleContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plus: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
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
    pub xmpp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_mail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mumble: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastodon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gopher: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Feed {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Cache {
    pub schedule: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Stream {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mjpeg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ustream: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BillingInterval {
    Yearly,
    Monthly,
    Weekly,
    Daily,
    Hourly,
    Other,
}

impl Default for BillingInterval {
    fn default() -> Self {
        BillingInterval::Monthly
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct MembershipPlan {
    pub name: String,
    pub value: f64,
    pub currency: String,
    pub billing_interval: BillingInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ApiVersion {
    #[serde(rename = "14")]
    V14,
}

/// The main SpaceAPI status object.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Status {
    // Hackerspace properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_compatibility: Option<Vec<ApiVersion>>,

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
    pub stream: Option<Stream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeds: Option<Feeds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radio_show: Option<Vec<RadioShow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_plans: Option<Vec<MembershipPlan>>,

    // SpaceAPI internal usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Cache>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub issue_report_channels: Vec<IssueReportChannel>,

    // Mutable data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensors: Option<Sensors>,

    // Custom extensions are allowed and will be prefixed with `ext_`.
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Status {
    /// Create a new Status object with only the absolutely required fields.
    #[deprecated(
        since = "0.5.0",
        note = "Please use the `StatusBuilder` or a struct expression instead"
    )]
    pub fn new<S: Into<String>>(
        space: S,
        logo: S,
        url: S,
        location: Location,
        contact: Contact,
        issue_report_channels: Vec<IssueReportChannel>,
    ) -> Status {
        Status {
            api: Some("0.13".into()),
            space: space.into(),
            logo: logo.into(),
            url: url.into(),
            location,
            contact,
            issue_report_channels,
            ..Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum StatusBuilderVersion {
    V0_13,
    V14,
    Mixed,
}

impl Default for StatusBuilderVersion {
    fn default() -> StatusBuilderVersion {
        StatusBuilderVersion::V0_13
    }
}

/// Builder for the `Status` object.
#[derive(Default, Debug, Clone)]
pub struct StatusBuilder {
    version: StatusBuilderVersion,
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
    links: Option<Vec<Link>>,
    membership_plans: Option<Vec<MembershipPlan>>,
    issue_report_channels: Vec<IssueReportChannel>,
    extensions: Extensions,
    state: Option<State>,
}

impl StatusBuilder {
    pub fn new<S: Into<String>>(space_name: S) -> StatusBuilder {
        StatusBuilder {
            space: space_name.into(),
            ..Default::default()
        }
    }

    pub fn v0_13<S: Into<String>>(space_name: S) -> StatusBuilder {
        StatusBuilder {
            space: space_name.into(),
            version: StatusBuilderVersion::V0_13,
            ..Default::default()
        }
    }

    pub fn v14<S: Into<String>>(space_name: S) -> StatusBuilder {
        StatusBuilder {
            space: space_name.into(),
            version: StatusBuilderVersion::V14,
            ..Default::default()
        }
    }

    pub fn mixed<S: Into<String>>(space_name: S) -> StatusBuilder {
        StatusBuilder {
            space: space_name.into(),
            version: StatusBuilderVersion::Mixed,
            ..Default::default()
        }
    }

    pub fn state(mut self, state: State) -> Self {
        self.state = Some(state);
        self
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

    pub fn add_link(mut self, link: Link) -> Self {
        self.links.get_or_insert(vec![]).push(link);
        self
    }

    pub fn add_membership_plan(mut self, membership_plan: MembershipPlan) -> Self {
        self.membership_plans.get_or_insert(vec![]).push(membership_plan);
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
    /// The prefix `ext_` will automatically be prepended to the name, if not already present.
    pub fn add_extension<V: Into<Value>>(mut self, name: &str, value: V) -> Self {
        if name.starts_with("ext_") {
            self.extensions.insert(name.to_owned(), value.into());
        } else {
            self.extensions.insert(format!("ext_{}", name), value.into());
        }
        self
    }

    pub fn build(self) -> Result<Status, String> {
        let api = match self.version {
            StatusBuilderVersion::V0_13 | StatusBuilderVersion::Mixed => Some("0.13".to_owned()),
            _ => None,
        };
        let api_compatibility = match self.version {
            StatusBuilderVersion::V14 | StatusBuilderVersion::Mixed => Some(vec![ApiVersion::V14]),
            _ => None,
        };

        let contact = self.contact.ok_or("contact missing")?;
        if let Some(spacefed) = &self.spacefed {
            spacefed.verify(self.version)?;
        }
        if let Some(state) = &self.state {
            state.verify(self.version)?;
        }

        if self.version == StatusBuilderVersion::V14 {
            if contact.jabber.is_some() {
                return Err("jabber key under contact was renamed to xmpp".into());
            }
            if contact.google.is_some() {
                return Err("google key under contact was removed".into());
            }
            if self.radio_show.is_some() {
                return Err("radio_show key was removed".into());
            }

            if !self.issue_report_channels.is_empty() {
                return Err("issue_report_channels key was removed".into());
            }
        } else {
            if self.issue_report_channels.is_empty() {
                return Err("issue_report_channels must not be empty".into());
            }
            if self.state.is_none() {
                return Err("state must be present in v0.13".into());
            }
            if let Some(ref location) = self.location {
                if location.timezone.is_some() {
                    return Err("location.timezone is only present in v0.14 and above".into());
                }
            }
            if self.links.is_some() {
                return Err("links is only present in v0.14 and above".into());
            }
            if self.membership_plans.is_some() {
                return Err("membership_plans is only present in v0.14 and above".into());
            }
        }

        Ok(Status {
            api,
            api_compatibility,
            space: self.space,
            logo: self.logo.ok_or("logo missing")?,
            url: self.url.ok_or("url missing")?,
            location: self.location.ok_or("location missing")?,
            contact,
            spacefed: self.spacefed,
            projects: self.projects,
            cam: self.cam,
            feeds: self.feeds,
            events: self.events,
            radio_show: self.radio_show,
            links: self.links,
            membership_plans: self.membership_plans,
            issue_report_channels: self.issue_report_channels,
            state: self.state,
            extensions: self.extensions,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{from_str, to_string};

    #[test]
    fn serialize_deserialize_cache() {
        let a = Cache {
            schedule: "bla".into(),
        };
        let b: Cache = from_str(&to_string(&a).unwrap()).unwrap();
        assert_eq!(a.schedule, b.schedule);
    }

    #[test]
    fn serialize_deserialize_simple_contact() {
        let a: Contact = Contact {
            keymasters: Some(vec![Keymaster {
                name: Some("Joe".into()),
                email: Some("joe@example.com".into()),
                ..Keymaster::default()
            }]),
            irc: Some("bla".into()),
            google: Some(GoogleContact {
                plus: Some("http://gplus/profile".into()),
            }),
            email: Some("bli@bla".into()),
            ..Default::default()
        };
        let b: Contact = from_str(&to_string(&a).unwrap()).unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn test_builder_v13_fail_on_location_timezone() {
        let status = StatusBuilder::v0_13("foo")
            .logo("bar")
            .url("foobar")
            .location(Location {
                timezone: Some("Europe/London".into()),
                ..Default::default()
            })
            .contact(Contact::default())
            .add_issue_report_channel(IssueReportChannel::Email)
            .state(State {
                open: Some(false),
                ..State::default()
            })
            .build();
        assert!(status.is_err());
        assert_eq!(
            status.err().unwrap(),
            "location.timezone is only present in v0.14 and above"
        );
    }

    #[test]
    fn test_builder_v13_fail_on_links() {
        let status = StatusBuilder::v0_13("foo")
            .logo("bar")
            .url("foobar")
            .contact(Contact::default())
            .add_issue_report_channel(IssueReportChannel::Email)
            .state(State {
                open: Some(false),
                ..State::default()
            })
            .add_link(Link::default())
            .build();
        assert!(status.is_err());
        assert_eq!(status.err().unwrap(), "links is only present in v0.14 and above");
    }

    #[test]
    fn test_builder_v13_fail_on_membership_plans() {
        let status = StatusBuilder::v0_13("foo")
            .logo("bar")
            .url("foobar")
            .contact(Contact::default())
            .add_issue_report_channel(IssueReportChannel::Email)
            .state(State {
                open: Some(false),
                ..State::default()
            })
            .add_membership_plan(MembershipPlan::default())
            .build();
        assert!(status.is_err());
        assert_eq!(
            status.err().unwrap(),
            "membership_plans is only present in v0.14 and above"
        );
    }

    #[test]
    fn test_builder_v14() {
        let status = StatusBuilder::v14("foo")
            .logo("bar")
            .url("foobar")
            .location(Location::default())
            .contact(Contact::default())
            .add_link(Link::default())
            .add_membership_plan(MembershipPlan::default())
            .build()
            .unwrap();
        assert_eq!(
            status,
            Status {
                api: None,
                api_compatibility: Some(vec![ApiVersion::V14]),
                space: "foo".into(),
                logo: "bar".into(),
                url: "foobar".into(),
                issue_report_channels: vec![],
                links: Some(vec![Link::default()]),
                membership_plans: Some(vec![MembershipPlan::default()]),
                ..Status::default()
            }
        );
    }

    #[test]
    fn test_builder_v14_fail_on_jabber() {
        let status = StatusBuilder::v14("foo")
            .logo("bar")
            .url("foobar")
            .location(Location::default())
            .contact(Contact {
                jabber: Some("jabber".into()),
                ..Contact::default()
            })
            .build();
        assert!(status.is_err());
    }

    #[test]
    fn test_builder_v14_fail_on_google() {
        let status = StatusBuilder::v14("foo")
            .logo("bar")
            .url("foobar")
            .location(Location::default())
            .contact(Contact {
                google: Some(GoogleContact::default()),
                ..Contact::default()
            })
            .build();
        assert!(status.is_err());
    }

    #[test]
    fn test_builder_v14_fail_on_radio_show() {
        let status = StatusBuilder::v14("foo")
            .logo("bar")
            .url("foobar")
            .location(Location::default())
            .contact(Contact::default())
            .add_radio_show(RadioShow::default())
            .build();
        assert!(status.is_err());
    }

    #[test]
    fn test_builder_v14_fail_on_spacephone() {
        let status = StatusBuilder::v14("foo")
            .logo("bar")
            .url("foobar")
            .location(Location::default())
            .contact(Contact::default())
            .spacefed(Spacefed {
                spacephone: Some(true),
                ..Spacefed::default()
            })
            .build();
        assert!(status.is_err());
    }

    #[test]
    fn test_builder_mixed() {
        let status = StatusBuilder::mixed("foo")
            .logo("bar")
            .url("foobar")
            .state(State::default())
            .location(Location::default())
            .contact(Contact::default())
            .add_issue_report_channel(IssueReportChannel::Email)
            .build()
            .unwrap();
        assert_eq!(
            status,
            Status {
                api: Some("0.13".into()),
                api_compatibility: Some(vec![ApiVersion::V14]),
                space: "foo".into(),
                logo: "bar".into(),
                url: "foobar".into(),
                state: Some(State {
                    open: None,
                    ..State::default()
                }),
                issue_report_channels: vec![IssueReportChannel::Email],
                ..Status::default()
            }
        );
    }

    #[test]
    fn test_builder() {
        let status = StatusBuilder::new("foo")
            .logo("bar")
            .url("foobar")
            .state(State {
                open: Some(false),
                ..State::default()
            })
            .location(Location::default())
            .contact(Contact::default())
            .spacefed(Spacefed {
                spacephone: Some(false),
                ..Spacefed::default()
            })
            .feeds(Feeds::default())
            .add_project("spaceapi-rs")
            .add_cam("cam1")
            .add_cam("cam2".to_string())
            .add_event(Event::default())
            .add_issue_report_channel(IssueReportChannel::Email)
            .build()
            .unwrap();
        assert_eq!(status.api, Some("0.13".into()));
        assert_eq!(status.space, "foo");
        assert_eq!(status.logo, "bar");
        assert_eq!(status.url, "foobar");
        assert_eq!(status.location, Location::default());
        assert_eq!(status.contact, Contact::default());
        assert_eq!(
            status.spacefed,
            Some(Spacefed {
                spacephone: Some(false),
                ..Spacefed::default()
            })
        );
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
        assert_eq!(
            to_string(&f1).unwrap(),
            "{\"type\":\"rss\",\"url\":\"https://some/rss.xml\"}".to_string()
        );
        assert_eq!(
            to_string(&f2).unwrap(),
            "{\"url\":\"https://some/rss.xml\"}".to_string()
        );
    }

    #[test]
    fn serialize_deserialize_full() {
        let status = StatusBuilder::new("foo")
            .logo("bar")
            .url("foobar")
            .state(State {
                open: Some(false),
                ..State::default()
            })
            .location(Location::default())
            .contact(Contact::default())
            .add_extension("aaa", Value::Array(vec![Value::Null, Value::from(42)]))
            .add_issue_report_channel(IssueReportChannel::Email)
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
            .state(State {
                open: Some(false),
                ..State::default()
            })
            .location(Location::default())
            .contact(Contact::default())
            .add_issue_report_channel(IssueReportChannel::Email)
            .build();
        assert!(status.is_ok());
        assert_eq!(
            &to_string(&status.unwrap()).unwrap(),
            "{\"api\":\"0.13\",\"space\":\"a\",\"logo\":\"b\",\"url\":\"c\",\
             \"location\":{\"lat\":0.0,\"lon\":0.0},\"contact\":{},\"issue_report_channels\":[\"email\"],\
             \"state\":{\"open\":false}}"
        );
    }

    #[test]
    fn serialize_extension_fields() {
        let status = StatusBuilder::new("a")
            .logo("b")
            .url("c")
            .state(State {
                open: Some(false),
                ..State::default()
            })
            .location(Location::default())
            .contact(Contact::default())
            .add_issue_report_channel(IssueReportChannel::Email)
            .add_extension("aaa", Value::String("xxx".into()))
            .add_extension("bbb", Value::Array(vec![Value::Null, Value::from(42)]))
            .build();
        assert!(status.is_ok());
        assert_eq!(
            &to_string(&status.unwrap()).unwrap(),
            "{\"api\":\"0.13\",\"space\":\"a\",\"logo\":\"b\",\"url\":\"c\",\
             \"location\":{\"lat\":0.0,\"lon\":0.0},\"contact\":{},\"issue_report_channels\":[\"email\"],\
             \"state\":{\"open\":false},\"ext_aaa\":\"xxx\",\"ext_bbb\":[null,42]}"
        );
    }

    #[test]
    fn serialize_deserialize_full_with_optional() {
        let mut status = StatusBuilder::new("foo")
            .logo("bar")
            .url("foobar")
            .state(State {
                open: Some(false),
                ..State::default()
            })
            .location(Location::default())
            .contact(Contact::default())
            .add_issue_report_channel(IssueReportChannel::Email)
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
            .state(State {
                open: Some(false),
                ..State::default()
            })
            .location(Location::default())
            .contact(Contact::default())
            .add_issue_report_channel(IssueReportChannel::Email)
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
        let deserialized: Status = from_str(data).unwrap();
        assert_eq!(deserialized.api, Some("0.13".into()));
        let keys = deserialized.extensions.keys();
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
            };
        }

        test_serialize!(issue_report_channel_email, IssueReportChannel::Email, "\"email\"");
        test_serialize!(
            issue_report_channel_issue_mail,
            IssueReportChannel::IssueMail,
            "\"issue_mail\""
        );
        test_serialize!(
            issue_report_channel_twitter,
            IssueReportChannel::Twitter,
            "\"twitter\""
        );
        test_serialize!(issue_report_channel_ml, IssueReportChannel::Ml, "\"ml\"");

        test_serialize!(stream_default, Stream::default(), "{}");

        test_serialize!(
            stream_m4,
            Stream {
                m4: Some("http://example.org/stream.mpg".to_string()),
                ..Stream::default()
            },
            r#"{"m4":"http://example.org/stream.mpg"}"#
        );

        test_serialize!(
            stream_mjpeg,
            Stream {
                mjpeg: Some("http://example.org/stream.mjpeg".to_string()),
                ..Stream::default()
            },
            r#"{"mjpeg":"http://example.org/stream.mjpeg"}"#
        );

        test_serialize!(
            stream_ustream,
            Stream {
                ustream: Some("http://www.ustream.tv/channel/hackspsps".to_string()),
                ..Stream::default()
            },
            r#"{"ustream":"http://www.ustream.tv/channel/hackspsps"}"#
        );
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
            };
        }

        test_deserialize!(
            issue_report_channel_email,
            "\"email\"",
            IssueReportChannel,
            IssueReportChannel::Email
        );
        test_deserialize!(
            issue_report_channel_issue_mail,
            "\"issue_mail\"",
            IssueReportChannel,
            IssueReportChannel::IssueMail
        );
        test_deserialize!(
            issue_report_channel_twitter,
            "\"twitter\"",
            IssueReportChannel,
            IssueReportChannel::Twitter
        );
        test_deserialize!(
            issue_report_channel_ml,
            "\"ml\"",
            IssueReportChannel,
            IssueReportChannel::Ml
        );
    }
}
