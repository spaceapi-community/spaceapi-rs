//! Module providing temperature sensor functionality.

use super::{LocalisedSensorMetadata, SensorMetadata, SensorTemplate, Sensors};
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
    pub metadata: SensorMetadata,
    pub unit: String,
}

impl TryInto<TemperatureSensor> for TemperatureSensorTemplate {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<TemperatureSensor, Self::Error> {
        Ok(TemperatureSensor {
            metadata: self.metadata.try_into()?,
            unit: self.unit,
            ..TemperatureSensor::default()
        })
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
