//! Module defining all sensor related structures.

use log::warn;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// Common information describing any sensor.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct SensorMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TryInto<LocalisedSensorMetadata> for SensorMetadata {
    type Error = &'static str;

    fn try_into(self) -> Result<LocalisedSensorMetadata, Self::Error> {
        match self.location {
            Some(location) => Ok(LocalisedSensorMetadata {
                name: self.name,
                location,
                description: self.description,
            }),
            None => Err("No location specified when one is required"),
        }
    }
}

/// Common information describing any sensor which requires a specified location.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct LocalisedSensorMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

//--- Templates ---//
/// A trait for all possible sensor templates.
///
/// A sensor template is like a sensor struct, but without the actual data in it.
/// A `SensorTemplate` is capable of registering itself in a `Sensors` struct.
pub trait SensorTemplate: Send + Sync {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        if let Err(e) = self.try_to_sensor(value_str, sensors) {
            warn!("Omitting sensor. Reason: {}", e);
        }
    }

    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors)
        -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone)]
pub struct PeopleNowPresentSensorTemplate {
    pub metadata: SensorMetadata,
    pub names: Option<Vec<String>>,
}

impl Into<PeopleNowPresentSensor> for PeopleNowPresentSensorTemplate {
    fn into(self) -> PeopleNowPresentSensor {
        PeopleNowPresentSensor {
            metadata: self.metadata,
            ..PeopleNowPresentSensor::default()
        }
    }
}

impl SensorTemplate for PeopleNowPresentSensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: PeopleNowPresentSensor = self.clone().into();
        sensor.value = value_str.parse::<u64>()?;
        sensors.people_now_present.push(sensor);
        Ok(())
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
    #[serde(flatten)]
    pub metadata: SensorMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
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
