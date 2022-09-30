//! Module providing temperature sensor functionality.

use super::{LocalisedSensorMetadata, SensorTemplate, Sensors};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct TemperatureSensor {
    #[serde(flatten)]
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct TemperatureSensorTemplate {
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
}

impl From<TemperatureSensorTemplate> for TemperatureSensor {
    fn from(template: TemperatureSensorTemplate) -> Self {
        Self {
            metadata: template.metadata,
            unit: template.unit,
            ..Default::default()
        }
    }
}

impl SensorTemplate for TemperatureSensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: TemperatureSensor = self.clone().try_into()?;
        sensor.value = value_str.parse::<f64>()?;
        sensors.temperature.push(sensor);
        Ok(())
    }
}
