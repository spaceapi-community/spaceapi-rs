//! Module providing network connections sensor functionality.

use super::{FromSensorTemplate, SensorMetadata, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct NetworkConnectionsSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machines: Option<Vec<NetworkConnectionMachine>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub kind: Option<NetworkConnectionKind>,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkConnectionKind {
    Wifi,
    Cable,
    Spacenet,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct NetworkConnectionMachine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub mac: String,
}

#[derive(Debug, Clone)]
pub struct NetworkConnectionsSensorTemplate {
    pub metadata: SensorMetadata,
    pub kind: Option<NetworkConnectionKind>,
}

impl FromSensorTemplate<NetworkConnectionsSensorTemplate> for NetworkConnectionsSensor {
    fn try_from_template(
        template: &NetworkConnectionsSensorTemplate,
        value: &str,
    ) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            kind: template.kind.clone(),
            value: value.parse()?,
            ..Default::default()
        })
    }
}

impl SensorTemplate for NetworkConnectionsSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .network_connections
            .push(NetworkConnectionsSensor::try_from_template(self, value_str)?);
        Ok(())
    }
}
