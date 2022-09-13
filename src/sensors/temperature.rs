//! Module providing temperature sensor functionality.

use super::{LocalisedSensorMetadata, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct TemperatureSensor {
    #[serde(flatten)]
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct TemperatureSensorTemplate {
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
}

impl From<TemperatureSensorTemplate> for TemperatureSensor {
    fn from(template: TemperatureSensorTemplate) -> Self {
        Self {
            metadata: template.metadata,
            unit: template.unit,
            ..Default::default()
        }
    }
}

impl SensorTemplate for TemperatureSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        let mut sensor: TemperatureSensor = self.clone().into();
        sensor.value = value_str.parse::<f64>()?;
        sensors.temperature.push(sensor);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_template() {
        let template = TemperatureSensorTemplate {
            metadata: LocalisedSensorMetadata {
                location: "Main Room".into(),
                description: Some("Centre of main room on ground floor".into()),
                ..Default::default()
            },
            unit: "C".into(),
        };

        let mut sensors = Sensors::default();
        template.to_sensor("24.1", &mut sensors);

        assert_eq!(
            "[{\"location\":\"Main Room\",\"description\":\"Centre of main room on ground floor\",\"unit\":\"C\",\"value\":24.1}]",
            serde_json::to_string(&sensors.temperature).unwrap()
        );
    }

    #[test]
    fn test_template_bad_float() {
        let template = TemperatureSensorTemplate {
            metadata: LocalisedSensorMetadata {
                location: "Main Room".into(),
                description: Some("Centre of main room on ground floor".into()),
                ..Default::default()
            },
            unit: "C".into(),
        };

        let mut sensors = Sensors::default();
        let result = template.try_to_sensor("twenty four point one", &mut sensors);

        assert!(result.is_err());
        assert_eq!(
            "sensor float value cannot be parsed",
            result.err().unwrap().to_string()
        );
    }
}
