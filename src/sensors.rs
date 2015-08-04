//! Module defining all sensor related structures.

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};
use utils::Optional;
use status::Status;


//--- Templates ---//

/// A trait for all possible sensor templates.
/// the SensorTemplate's are capable of registring themself in a Status struct
pub trait SensorTemplate : Send+Sync {
    fn to_sensor(&self, value_str: &str, status: &mut Status);
}

#[derive(Debug, Clone)]
pub struct PeopleNowPresentSensorTemplate {
    pub location: Optional<String>,
    pub name: Optional<String>,
    pub names: Optional<Vec<String>>,
    pub description: Optional<String>,
}

impl SensorTemplate for PeopleNowPresentSensorTemplate {
    fn to_sensor(&self, value_str: &str, status: &mut Status) {
        // TODO error handling
        let value = value_str.parse::<i64>().unwrap();
        let sensor = PeopleNowPresentSensor {
            location: self.location.clone(),
            name: self.name.clone(),
            names: self.names.clone(),
            description: self.description.clone(),
            value: value,
        };
        // TODO handle empty
        status.sensors.as_mut().map_or((), |v| {
            match v.people_now_present {
                Optional::Value(ref mut vec_sensors) => vec_sensors.push(sensor),
                Optional::Absent => v.people_now_present = Optional::Value(vec![sensor]),
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct TemperatureSensorTemplate {
    pub unit: String,
    pub location: String,
    pub name: Optional<String>,
    pub description: Optional<String>,
}

impl SensorTemplate for TemperatureSensorTemplate {
    fn to_sensor(&self, value_str: &str, status: &mut Status) {
        // TODO error handling
        let value = value_str.parse::<f64>().unwrap();
        let sensor = TemperatureSensor {
            unit: self.unit.clone(),
            location: self.location.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            value: value,
        };
        // TODO handle empty
        status.sensors.as_mut().map_or((), |v| {
            match v.temperature {
                Optional::Value(ref mut vec_sensors) => vec_sensors.push(sensor),
                Optional::Absent => v.temperature = Optional::Value(vec![sensor]),
            }
        });
    }
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
