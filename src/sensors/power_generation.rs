//! Module providing power generation sensor functionality.

use super::{FromSensorTemplate, SensorMetadataWithLocation, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PowerGenerationSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct PowerGenerationSensorTemplate {
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
}

impl FromSensorTemplate<PowerGenerationSensorTemplate> for PowerGenerationSensor {
    fn try_from_template(
        template: &PowerGenerationSensorTemplate,
        value: &str,
    ) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            unit: template.unit.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for PowerGenerationSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .power_generation
            .push(PowerGenerationSensor::try_from_template(self, value_str)?);
        Ok(())
    }
}
