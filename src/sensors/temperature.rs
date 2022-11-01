//! Module providing temperature sensor functionality.

use super::{LocalisedSensorMetadata, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

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
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        let mut sensor: TemperatureSensor = self.clone().into();
        sensor.value = value_str.parse::<f64>()?;
        sensors.temperature.push(sensor);
        Ok(())
    }
}
