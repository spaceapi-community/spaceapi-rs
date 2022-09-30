//! Module providing power consumption sensor functionality.

use super::{LocalisedSensorMetadata, SensorTemplate, Sensors};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PowerConsumptionSensor {
    #[serde(flatten)]
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct PowerConsumptionSensorTemplate {
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
}

impl From<PowerConsumptionSensorTemplate> for PowerConsumptionSensor {
    fn from(template: PowerConsumptionSensorTemplate) -> Self {
        Self {
            metadata: template.metadata,
            unit: template.unit,
            ..Default::default()
        }
    }
}

impl SensorTemplate for PowerConsumptionSensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: PowerConsumptionSensor = self.clone().try_into()?;
        sensor.value = value_str.parse::<f64>()?;
        sensors.power_consumption.push(sensor);
        Ok(())
    }
}
