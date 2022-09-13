//! Module defining all sensor related structures.

use log::warn;
use serde::{Deserialize, Serialize};

//--- Templates ---//
/// A trait for all possible sensor templates.
///
/// A sensor template is like a sensor struct, but without the actual data in it.
/// A `SensorTemplate` is capable of registering itself in a `Sensors` struct.
pub trait SensorTemplate: Send + Sync {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors);
}

#[derive(Debug, Clone)]
pub struct PeopleNowPresentSensorTemplate {
    pub location: Option<String>,
    pub name: Option<String>,
    pub names: Option<Vec<String>>,
    pub description: Option<String>,
}

impl SensorTemplate for PeopleNowPresentSensorTemplate {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        let parse_result = value_str.parse::<u64>().map(|value| {
            let sensor = PeopleNowPresentSensor {
                location: self.location.clone(),
                name: self.name.clone(),
                names: self.names.clone(),
                description: self.description.clone(),
                value,
            };
            sensors.people_now_present.push(sensor);
        });
        if parse_result.is_err() {
            warn!(
                "Could not parse value '{}', omiting PeopleNowPresentSensor",
                value_str
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemperatureSensorTemplate {
    pub unit: String,
    pub location: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl SensorTemplate for TemperatureSensorTemplate {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        let parse_result = value_str.parse::<f64>().map(|value| {
            let sensor = TemperatureSensor {
                unit: self.unit.clone(),
                location: self.location.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                value,
            };
            sensors.temperature.push(sensor);
        });
        if parse_result.is_err() {
            warn!("Could not parse value '{}', omiting TemperatureSensor", value_str);
        }
    }
}

#[derive(Debug, Clone)]
pub struct HumiditySensorTemplate {
    pub unit: String,
    pub location: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl SensorTemplate for HumiditySensorTemplate {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        let parse_result = value_str.parse::<f64>().map(|value| {
            let sensor = HumiditySensor {
                unit: self.unit.clone(),
                location: self.location.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                value,
            };
            sensors.humidity.push(sensor);
        });
        if parse_result.is_err() {
            warn!("Could not parse value '{}', omiting HumiditySensor", value_str);
        }
    }
}

#[derive(Debug, Clone)]
pub struct PowerConsumptionSensorTemplate {
    pub unit: String,
    pub location: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl SensorTemplate for PowerConsumptionSensor {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        let parse_result = value_str.parse::<f64>().map(|value| {
            let sensor = PowerConsumptionSensor {
                unit: self.unit.clone(),
                location: self.location.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                value,
            };
            sensors.power_consumption.push(sensor);
        });
        if parse_result.is_err() {
            warn!(
                "Could not parse value '{}', omiting PowerConsumptionSensor",
                value_str
            );
        }
    }
}

//--- Structures ---//

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PeopleNowPresentSensor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct TemperatureSensor {
    pub unit: String,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct HumiditySensor {
    pub unit: String,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PowerConsumptionSensor {
    pub unit: String,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Sensors {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub people_now_present: Vec<PeopleNowPresentSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub temperature: Vec<TemperatureSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub humidity: Vec<HumiditySensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub power_consumption: Vec<PowerConsumptionSensor>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{from_str, to_string};

    #[test]
    fn serialize_deserialize_sensors() {
        let a = Sensors {
            people_now_present: vec![],
            temperature: vec![],
            humidity: vec![],
            power_consumption: vec![],
        };
        let b: Sensors = from_str(&to_string(&a).unwrap()).unwrap();
        assert_eq!(a, b);
    }
}
