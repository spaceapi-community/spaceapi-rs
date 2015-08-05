//! Module defining all sensor related structures.

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};
use utils::Optional;


//--- Templates ---//

/// A trait for all possible sensor templates.
/// the `SensorTemplate`s are capable of registering themselves in a `Sensors` struct
pub trait SensorTemplate : Send+Sync {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors);
}

#[derive(Debug, Clone)]
pub struct PeopleNowPresentSensorTemplate {
    pub location: Optional<String>,
    pub name: Optional<String>,
    pub names: Optional<Vec<String>>,
    pub description: Optional<String>,
}

impl SensorTemplate for PeopleNowPresentSensorTemplate {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        if value_str.parse::<i64>().map(|value|{
            let sensor = PeopleNowPresentSensor {
                location: self.location.clone(),
                name: self.name.clone(),
                names: self.names.clone(),
                description: self.description.clone(),
                value: value,
            };

            match sensors.people_now_present {
                Optional::Value(ref mut vec_sensors) => {
                    vec_sensors.push(sensor)
                }
                Optional::Absent => {
                    sensors.people_now_present = Optional::Value(vec![sensor])
                }
            }
        }).is_err() {
            warn!("Could not parse '{}': omiting the sensor", value_str);
        }
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
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        if value_str.parse::<f64>().map(|value|{
            let sensor = TemperatureSensor {
                unit: self.unit.clone(),
                location: self.location.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                value: value,
            };
            match sensors.temperature {
                Optional::Value(ref mut vec_sensors) => {
                    vec_sensors.push(sensor)
                }
                Optional::Absent => {
                    sensors.temperature = Optional::Value(vec![sensor])
                }
            }
        }).is_err() {
            warn!("Could not parse '{}': omiting the sensor", value_str);
        }
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
