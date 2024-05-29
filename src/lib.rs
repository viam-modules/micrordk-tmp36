use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use micro_rdk::DoCommand;
use micro_rdk::common::config::ConfigType;
use micro_rdk::common::status::{Status, StatusError};
use micro_rdk::common::registry::{ComponentRegistry, RegistryError, Dependency};

use micro_rdk::common::sensor::{Sensor, SensorType, Readings, SensorError};


pub fn register_models(registry: &mut ComponentRegistry) -> Result<(), RegistryError> {
    registry.register_sensor("tmp36", &MySensor::from_config);
    Ok(())
}


#[derive(DoCommand)]
pub struct MySensor {
// TODO: required - board dependency
// TODO: later - name of analog sensor scaling config

}

impl MySensor {
    pub fn from_config(cfg: ConfigType, deps: Vec<Dependency>) -> Result<SensorType,SensorError> {
        // TODO: required - extract the board dependecy from deps for the constructor
        // TODO: later - validate analog sensor existence from config

        Ok(Arc::new(Mutex::new(MySensor {})))
    }
}

impl Status for MySensor {
    fn get_status(&self) -> Result<Option<micro_rdk::google::protobuf::Struct>, StatusError> {
        Ok(Some(micro_rdk::google::protobuf::Struct {
            // TODO: required - read value from the analog sensor (don't care which one)
            // TODO: later - compute C and scale the read value
            fields: HashMap::new(),
        }))
    }
}

