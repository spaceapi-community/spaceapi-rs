//! Module defining common sensor functionality.

mod account_balance;
mod barometer;
mod beverage_supply;
mod carbondioxide;
mod door_locked;
mod humidity;
mod network_connections;
mod network_traffic;
mod people_now_present;
mod power_consumption;
mod radiation;
mod temperature;
mod total_member_count;
mod wind;

pub use account_balance::{AccountBalanceSensor, AccountBalanceSensorTemplate};
pub use barometer::{BarometerSensor, BarometerSensorTemplate};
pub use beverage_supply::{BeverageSupplySensor, BeverageSupplySensorTemplate};
pub use carbondioxide::{CarbondioxideSensor, CarbondioxideSensorTemplate};
pub use door_locked::{DoorLockedSensor, DoorLockedSensorTemplate};
pub use humidity::{HumiditySensor, HumiditySensorTemplate};
pub use network_connections::{
    NetworkConnectionKind, NetworkConnectionMachine, NetworkConnectionsSensor,
    NetworkConnectionsSensorTemplate,
};
pub use network_traffic::{
    NetworkTrafficBitsPerSecond, NetworkTrafficPacketsPerSecond, NetworkTrafficSensor,
    NetworkTrafficSensorProperties,
};
pub use people_now_present::{PeopleNowPresentSensor, PeopleNowPresentSensorTemplate};
pub use power_consumption::{PowerConsumptionSensor, PowerConsumptionSensorTemplate};
pub use radiation::{RadiationSensor, RadiationSensorUnit, RadiationSensors};
pub use temperature::{TemperatureSensor, TemperatureSensorTemplate};
pub use total_member_count::{TotalMemberCountSensor, TotalMemberCountSensorTemplate};
pub use wind::{WindSensor, WindSensorMeasurement, WindSensorProperties};

use log::warn;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Common information describing any sensor.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct SensorMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastchange: Option<u64>,
}

/// Common information describing any sensor which requires a specified location.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct SensorMetadataWithLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastchange: Option<u64>,
}

/// Describes an error occurring when building a sensor from a `SensorTemplate`.
#[derive(Error, Debug)]
pub enum SensorTemplateError {
    /// Failed when parsing an integer value from the provided value string
    #[error("sensor integer value cannot be parsed")]
    BadInteger(#[from] std::num::ParseIntError),

    /// Failed when parsing a floating point value from the provided value string
    #[error("sensor float value cannot be parsed")]
    BadFloat(#[from] std::num::ParseFloatError),

    /// Failed when parsing a boolean value from the provided value string
    #[error("sensor boolean value cannot be parsed")]
    BadBool(#[from] std::str::ParseBoolError),
}

/// Trait that allows sensors to be created from a template and string value.
pub trait FromSensorTemplate<T> {
    fn try_from_template(template: &T, value: &str) -> Result<Self, SensorTemplateError>
    where
        Self: Sized;
}

/// A trait for all possible sensor templates.
///
/// A sensor template is like a sensor struct, but without the actual data in it.
/// A `SensorTemplate` is capable of registering itself in a `Sensors` struct.
pub trait SensorTemplate: Send + Sync {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        if let Err(e) = self.try_to_sensor(value_str, sensors) {
            warn!("Omitting sensor. Reason: {}", e);
        }
    }

    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors) -> Result<(), SensorTemplateError>;
}

/// Container for instances of all sensor types.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Sensors {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub temperature: Vec<TemperatureSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub door_locked: Vec<DoorLockedSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub barometer: Vec<BarometerSensor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radiation: Option<RadiationSensors>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub humidity: Vec<HumiditySensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beverage_supply: Vec<BeverageSupplySensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub power_consumption: Vec<PowerConsumptionSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wind: Vec<WindSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network_connections: Vec<NetworkConnectionsSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account_balance: Vec<AccountBalanceSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub total_member_count: Vec<TotalMemberCountSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub people_now_present: Vec<PeopleNowPresentSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network_traffic: Vec<NetworkTrafficSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub carbondioxide: Vec<CarbondioxideSensor>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{from_str, to_string};

    #[test]
    fn serialize_deserialize_sensors() {
        let a = Sensors::default();
        let b: Sensors = from_str(&to_string(&a).unwrap()).unwrap();
        assert_eq!(a, b);
    }
}
