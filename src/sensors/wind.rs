//! Module providing wind sensor functionality.

use super::SensorMetadataWithLocation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct WindSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadataWithLocation,
    pub properties: WindSensorProperties,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct WindSensorProperties {
    pub speed: WindSensorMeasurement,
    pub gust: WindSensorMeasurement,
    pub direction: WindSensorMeasurement,
    pub elevation: WindSensorMeasurement,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct WindSensorMeasurement {
    pub unit: String,
    pub value: f64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        let sensor = WindSensor::default();

        assert_eq!("{\"location\":\"\",\"properties\":{\"speed\":{\"unit\":\"\",\"value\":0.0},\"gust\":{\"unit\":\"\",\"value\":0.0},\"direction\":{\"unit\":\"\",\"value\":0.0},\"elevation\":{\"unit\":\"\",\"value\":0.0}}}", serde_json::to_string(&sensor).unwrap());
    }
}
