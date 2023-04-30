//! Module providing door lock sensor functionality.

use super::{FromSensorTemplate, SensorMetadataWithLocation, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct DoorLockedSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadataWithLocation,
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct DoorLockedSensorTemplate {
    pub metadata: SensorMetadataWithLocation,
}

impl FromSensorTemplate<DoorLockedSensorTemplate> for DoorLockedSensor {
    fn try_from_template(
        template: &DoorLockedSensorTemplate,
        value: &str,
    ) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for DoorLockedSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .door_locked
            .push(DoorLockedSensor::try_from_template(self, value_str)?);
        Ok(())
    }
}
