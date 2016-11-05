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
                Option::Some(ref mut vec_sensors) => {
                    vec_sensors.push(sensor)
                }
                Option::None => {
                    sensors.people_now_present = Option::Some(vec![sensor])
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
                Option::Some(ref mut vec_sensors) => {
                    vec_sensors.push(sensor)
                }
                Option::None => {
                    sensors.temperature = Option::Some(vec![sensor])
                }
            }
        }).is_err() {
            warn!("Could not parse value '{}', omiting TemperatureSensor", value_str);
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/sensors.rs"));
