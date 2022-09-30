//! Module providing humidity sensor functionality.

use super::{LocalisedSensorMetadata, SensorTemplate, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct HumiditySensor {
    #[serde(flatten)]
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct HumiditySensorTemplate {
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
}

impl From<HumiditySensorTemplate> for HumiditySensor {
    fn from(st: HumiditySensorTemplate) -> Self {
        Self {
            metadata: st.metadata,
            unit: st.unit,
            ..Default::default()
        }
    }
}

impl SensorTemplate for HumiditySensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: HumiditySensor = self.clone().into();
        sensor.value = value_str.parse::<f64>()?;
        sensors.humidity.push(sensor);
        Ok(())
    }
}
