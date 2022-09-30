//! Module providing people present sensor functionality.

use super::{SensorMetadata, SensorTemplate, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PeopleNowPresentSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub struct PeopleNowPresentSensorTemplate {
    pub metadata: SensorMetadata,
    pub names: Option<Vec<String>>,
}

impl From<PeopleNowPresentSensorTemplate> for PeopleNowPresentSensor {
    fn from(template: PeopleNowPresentSensorTemplate) -> Self {
        Self {
            metadata: template.metadata,
            ..Default::default()
        }
    }
}

impl SensorTemplate for PeopleNowPresentSensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: PeopleNowPresentSensor = self.clone().into();
        sensor.value = value_str.parse::<u64>()?;
        sensors.people_now_present.push(sensor);
        Ok(())
    }
}
