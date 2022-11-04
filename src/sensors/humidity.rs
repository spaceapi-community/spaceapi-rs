//! Module providing humidity sensor functionality.

use super::{FromSensorTemplate, SensorMetadataWithLocation, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct HumiditySensor {
    #[serde(flatten)]
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct HumiditySensorTemplate {
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
}

impl FromSensorTemplate<HumiditySensorTemplate> for HumiditySensor {
    fn try_from(template: &HumiditySensorTemplate, value: &str) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            unit: template.unit.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for HumiditySensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors.humidity.push(HumiditySensor::try_from(self, value_str)?);
        Ok(())
    }
}
