//! Module providing carbondioxide sensor functionality.

use super::{FromSensorTemplate, SensorMetadataWithLocation, SensorTemplate, SensorTemplateError, Sensors};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct CarbondioxideSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub struct CarbondioxideSensorTemplate {
    pub metadata: SensorMetadataWithLocation,
    pub unit: String,
}

impl FromSensorTemplate<CarbondioxideSensorTemplate> for CarbondioxideSensor {
    fn try_from_template(
        template: &CarbondioxideSensorTemplate,
        value: &str,
    ) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            unit: template.unit.clone(),
            value: value.parse()?,
        })
    }
}

impl SensorTemplate for CarbondioxideSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .carbondioxide
            .push(CarbondioxideSensor::try_from_template(self, value_str)?);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_template() {
        let template = CarbondioxideSensorTemplate {
            metadata: SensorMetadataWithLocation {
                location: "Main Room".into(),
                description: Some("Centre of main room on ground floor".into()),
                ..Default::default()
            },
            unit: "ppm".into(),
        };

        let mut sensors = Sensors::default();
        template.to_sensor("1234", &mut sensors);

        assert_eq!(
            "[{\"location\":\"Main Room\",\"description\":\"Centre of main room on ground floor\",\"unit\":\"ppm\",\"value\":1234}]",
            serde_json::to_string(&sensors.carbondioxide).unwrap()
        );
    }

    #[test]
    fn test_template_bad_float() {
        let template = CarbondioxideSensorTemplate {
            metadata: SensorMetadataWithLocation {
                location: "Main Room".into(),
                description: Some("Centre of main room on ground floor".into()),
                ..Default::default()
            },
            unit: "ppm".into(),
        };

        let mut sensors = Sensors::default();
        let result = template.try_to_sensor("one thousand two hundred thirty four", &mut sensors);

        assert!(result.is_err());
        assert_eq!(
            "sensor integer value cannot be parsed",
            result.err().unwrap().to_string()
        );
    }
}
