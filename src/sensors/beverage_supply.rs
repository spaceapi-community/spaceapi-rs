//! Module providing beverage supply sensor functionality.

use super::{FromSensorTemplate, SensorMetadata, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct BeverageSupplySensor {
    #[serde(flatten)]
    pub metadata: SensorMetadata,
    pub unit: String,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub struct BeverageSupplySensorTemplate {
    pub metadata: SensorMetadata,
    pub unit: String,
}

impl FromSensorTemplate<BeverageSupplySensorTemplate> for BeverageSupplySensor {
    fn try_from(template: &BeverageSupplySensorTemplate, value: &str) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            unit: template.unit.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for BeverageSupplySensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .beverage_supply
            .push(BeverageSupplySensor::try_from(self, value_str)?);
        Ok(())
    }
}
