//! Module defining all sensor related structures.

//--- Templates ---//
/// A trait for all possible sensor templates.
///
/// A sensor template is like a sensor struct, but without the actual data in it.
/// A `SensorTemplate` is capable of registering itself in a `Sensors` struct.
pub trait SensorTemplate : Send+Sync {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors);
}

#[derive(Debug, Clone)]
pub struct PeopleNowPresentSensorTemplate {
    pub location: Option<String>,
    pub name: Option<String>,
    pub names: Option<Vec<String>>,
    pub description: Option<String>,
}

impl SensorTemplate for PeopleNowPresentSensorTemplate {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        if value_str.parse::<u64>().map(|value|{
            let sensor = PeopleNowPresentSensor {
                location: self.location.clone(),
                name: self.name.clone(),
                names: self.names.clone(),
                description: self.description.clone(),
                value: value,
            };

            match sensors.people_now_present {
                Some(ref mut vec_sensors) => {
                    vec_sensors.push(sensor)
                }
                None => {
                    sensors.people_now_present = Some(vec![sensor])
                }
            }
        }).is_err() {
            warn!("Could not parse value '{}', omiting PeopleNowPresentSensor", value_str);
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemperatureSensorTemplate {
    pub unit: String,
    pub location: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl SensorTemplate for TemperatureSensorTemplate {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        if value_str.parse::<f64>().map(|value|{
            let sensor = TemperatureSensor {
                unit: self.unit.clone(),
                location: self.location.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                value: value,
            };
            match sensors.temperature {
                Some(ref mut vec_sensors) => {
                    vec_sensors.push(sensor)
                }
                None => {
                    sensors.temperature = Some(vec![sensor])
                }
            }
        }).is_err() {
            warn!("Could not parse value '{}', omiting TemperatureSensor", value_str);
        }
    }
}

//--- Structures ---//

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PeopleNowPresentSensor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TemperatureSensor {
    pub unit: String,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sensors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_now_present: Option<Vec<PeopleNowPresentSensor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Vec<TemperatureSensor>>,
}


