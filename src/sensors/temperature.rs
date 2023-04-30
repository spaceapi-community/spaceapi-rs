//! Module providing temperature sensor functionality.

use super::{FromSensorTemplate, SensorMetadataWithLocation, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct TemperatureSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct TemperatureSensorTemplate {
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
}

impl FromSensorTemplate<TemperatureSensorTemplate> for TemperatureSensor {
    fn try_from_template(
        template: &TemperatureSensorTemplate,
        value: &str,
    ) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            unit: template.unit.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for TemperatureSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .temperature
            .push(TemperatureSensor::try_from_template(self, value_str)?);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_template() {
        let template = TemperatureSensorTemplate {
            metadata: SensorMetadataWithLocation {
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
            metadata: SensorMetadataWithLocation {
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
