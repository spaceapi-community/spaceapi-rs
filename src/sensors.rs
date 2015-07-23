//! Module defining all sensor related structures.

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};
use utils::Optional;


//--- Templates ---//

/// An enum of all possible sensor templates.
#[derive(Debug, Clone)]
pub enum SensorTemplate {
    PeopleNowPresentSensorTemplate {
        location: Optional<String>,
        name: Optional<String>,
        names: Optional<Vec<String>>,
        description: Optional<String>,
    },
    TemperatureSensorTemplate {
        unit: String,
        location: String,
        name: Optional<String>,
        description: Optional<String>,
    },
}


//--- Structures ---//

#[derive(Debug, Clone)]
pub struct PeopleNowPresentSensor {
    pub location: Optional<String>,
    pub name: Optional<String>,
    pub names: Optional<Vec<String>>,
    pub description: Optional<String>,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct TemperatureSensor {
    pub unit: String,
    pub location: String,
    pub name: Optional<String>,
    pub description: Optional<String>,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct Sensors {
    pub people_now_present: Optional<Vec<PeopleNowPresentSensor>>,
    pub temperature: Optional<Vec<TemperatureSensor>>,
}


//--- Serialization ---//

impl ToJson for Sensors {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        self.people_now_present.as_ref().map_or((), |v| { d.insert("people_now_present".to_string(), v.to_json()); });
        self.temperature.as_ref().map_or((), |v| { d.insert("temperature".to_string(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for PeopleNowPresentSensor {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("value".to_string(), self.value.to_json());
        self.location.as_ref().map_or((), |v| { d.insert("location".to_string(), v.to_json()); });
        self.name.as_ref().map_or((), |v| { d.insert("name".to_string(), v.to_json()); });
        self.names.as_ref().map_or((), |v| { d.insert("names".to_string(), v.to_json()); });
        self.description.as_ref().map_or((), |v| { d.insert("description".to_string(), v.to_json()); });
        Json::Object(d)
    }
}

impl ToJson for TemperatureSensor {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("value".to_string(), self.value.to_json());
        d.insert("unit".to_string(), self.unit.to_json());
        d.insert("location".to_string(), self.location.to_json());
        self.name.as_ref().map_or((), |v| { d.insert("name".to_string(), v.to_json()); });
        self.description.as_ref().map_or((), |v| { d.insert("description".to_string(), v.to_json()); });
        Json::Object(d)
    }
}
