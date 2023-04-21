//! Module providing network traffic sensor functionality.

use super::SensorMetadata;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct NetworkTrafficSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadata,
    pub properties: NetworkTrafficSensorProperties,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct NetworkTrafficSensorProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits_per_second: Option<NetworkTrafficBitsPerSecond>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets_per_second: Option<NetworkTrafficPacketsPerSecond>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct NetworkTrafficBitsPerSecond {
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct NetworkTrafficPacketsPerSecond {
    pub value: f64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        let sensor = NetworkTrafficSensor {
            properties: NetworkTrafficSensorProperties {
                bits_per_second: Some(NetworkTrafficBitsPerSecond {
                    value: 42.0,
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            "{\"properties\":{\"bits_per_second\":{\"value\":42.0}}}",
            serde_json::to_string(&sensor).unwrap()
        );
    }
}
