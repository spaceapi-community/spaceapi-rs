//! Module providing account balance sensor functionality.

use super::{FromSensorTemplate, SensorMetadata, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct AccountBalanceSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadata,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct AccountBalanceSensorTemplate {
    pub metadata: SensorMetadata,
    pub unit: String,
}

impl FromSensorTemplate<AccountBalanceSensorTemplate> for AccountBalanceSensor {
    fn try_from(template: &AccountBalanceSensorTemplate, value: &str) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            unit: template.unit.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for AccountBalanceSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .account_balance
            .push(AccountBalanceSensor::try_from(self, value_str)?);
        Ok(())
    }
}
