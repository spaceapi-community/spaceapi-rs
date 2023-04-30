//! Module providing barometer sensor functionality.

use super::{FromSensorTemplate, SensorMetadataWithLocation, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct BarometerSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct BarometerSensorTemplate {
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
}

impl FromSensorTemplate<BarometerSensorTemplate> for BarometerSensor {
    fn try_from_template(
        template: &BarometerSensorTemplate,
        value: &str,
    ) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            unit: template.unit.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for BarometerSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .barometer
            .push(BarometerSensor::try_from_template(self, value_str)?);
        Ok(())
    }
}
