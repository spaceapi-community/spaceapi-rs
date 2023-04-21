//! Module providing radiation sensor functionality.

use super::SensorMetadata;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct RadiationSensors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<Vec<RadiationSensor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta: Option<Vec<RadiationSensor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<Vec<RadiationSensor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta_gamma: Option<Vec<RadiationSensor>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RadiationSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_factor: Option<f64>,
    pub unit: RadiationSensorUnit,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RadiationSensorUnit {
    #[serde(rename = "cpm")]
    CountsPerMinute,
    #[serde(rename = "r/h")]
    RadsPerHour,
    #[serde(rename = "µSv/h")]
    MicroSievertsPerHour,
    #[serde(rename = "µSv/a")]
    MicroSievertsPerYear,
    #[serde(rename = "mSv/h")]
    MilliSievertsPerYear,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        let sensors = RadiationSensors {
            alpha: Some(vec![RadiationSensor {
                metadata: Default::default(),
                dead_time: None,
                conversion_factor: None,
                unit: RadiationSensorUnit::MicroSievertsPerHour,
                value: 0.15,
            }]),
            ..Default::default()
        };

        assert_eq!(
            "{\"alpha\":[{\"unit\":\"µSv/h\",\"value\":0.15}]}",
            serde_json::to_string(&sensors).unwrap()
        );
    }
}
