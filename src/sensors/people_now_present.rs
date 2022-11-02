//! Module providing people present sensor functionality.

use super::{FromSensorTemplate, SensorMetadata, SensorTemplate, SensorTemplateError, Sensors};
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

impl FromSensorTemplate<PeopleNowPresentSensorTemplate> for PeopleNowPresentSensor {
    fn try_from(template: &PeopleNowPresentSensorTemplate, value: &str) -> Result<Self, SensorTemplateError> {
        Ok(Self {
            metadata: template.metadata.clone(),
            value: value.parse()?,
            ..Default::default()
        })
    }
}

impl SensorTemplate for PeopleNowPresentSensorTemplate {
    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError> {
        sensors
            .people_now_present
            .push(PeopleNowPresentSensor::try_from(self, value_str)?);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_template() {
        let template = PeopleNowPresentSensorTemplate {
            metadata: SensorMetadata::default(),
            names: None,
        };

        let mut sensors = Sensors::default();
        template.to_sensor("8", &mut sensors);

        assert_eq!(
            "[{\"value\":8}]",
            serde_json::to_string(&sensors.people_now_present).unwrap()
        );
    }

    #[test]
    fn test_template_bad_integer() {
        let template = PeopleNowPresentSensorTemplate {
            metadata: SensorMetadata::default(),
            names: None,
        };

        let mut sensors = Sensors::default();
        let result = template.try_to_sensor("two", &mut sensors);

        assert!(result.is_err());
        assert_eq!(
            "sensor integer value cannot be parsed",
            result.err().unwrap().to_string()
        );
    }
}
