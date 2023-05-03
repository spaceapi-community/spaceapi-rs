//! Module providing total member count sensor functionality.

use super::{FromSensorTemplate, SensorMetadata, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct TotalMemberCountSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadata,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub struct TotalMemberCountSensorTemplate {
    pub metadata: SensorMetadata,
}

impl FromSensorTemplate<TotalMemberCountSensorTemplate> for TotalMemberCountSensor {
    fn try_from_template(
        template: &TotalMemberCountSensorTemplate,
        value: &str,
    ) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for TotalMemberCountSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .total_member_count
            .push(TotalMemberCountSensor::try_from_template(self, value_str)?);
        Ok(())
    }
}
