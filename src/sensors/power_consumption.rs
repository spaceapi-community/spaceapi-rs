//! Module providing power consumption sensor functionality.

use super::{FromSensorTemplate, SensorMetadataWithLocation, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PowerConsumptionSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct PowerConsumptionSensorTemplate {
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
}

impl FromSensorTemplate<PowerConsumptionSensorTemplate> for PowerConsumptionSensor {
    fn try_from_template(
        template: &PowerConsumptionSensorTemplate,
        value: &str,
    ) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            unit: template.unit.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for PowerConsumptionSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .power_consumption
            .push(PowerConsumptionSensor::try_from_template(self, value_str)?);
        Ok(())
    }
}
