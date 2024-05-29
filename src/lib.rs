use micro_rdk::{
    common::{
        analog::AnalogReaderType,
        board::Board,
        config::ConfigType,
        registry::{get_board_from_dependencies, ComponentRegistry, Dependency, RegistryError},
        sensor::{
            GenericReadingsResult, Readings, Sensor, SensorError, SensorResult, SensorT,
            SensorType, TypedReadingsResult,
        },
        status::{Status, StatusError},
    },
    DoCommand,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub fn register_models(registry: &mut ComponentRegistry) -> Result<(), RegistryError> {
    registry.register_sensor("tmp36", &Tmp36Sensor::from_config)?;
    Ok(())
}

#[derive(DoCommand)]
pub struct Tmp36Sensor {
    reader: AnalogReaderType<u16>,
}

impl Tmp36Sensor {
    pub fn from_config(_cfg: ConfigType, deps: Vec<Dependency>) -> Result<SensorType, SensorError> {
        let board = get_board_from_dependencies(deps);
        if board.is_none() {
            return Err(SensorError::ConfigError("sensor missing board attribute"));
        }
        let board_unwrapped = board.unwrap();
        if let Ok(reader) = board_unwrapped.get_analog_reader_by_name("tmp36".to_string()) {
            Ok(Arc::new(Mutex::new(Self { reader })))
        } else {
            Err(SensorError::ConfigError(
                "failed to get analog reader `tmp36`",
            ))
        }
    }
}

impl Status for Tmp36Sensor {
    fn get_status(&self) -> Result<Option<micro_rdk::google::protobuf::Struct>, StatusError> {
        Ok(Some(micro_rdk::google::protobuf::Struct {
            fields: HashMap::new(),
        }))
    }
}

impl Sensor for Tmp36Sensor {}

impl Readings for Tmp36Sensor {
    fn get_generic_readings(&mut self) -> Result<GenericReadingsResult, SensorError> {
        Ok(self
            .get_readings()?
            .into_iter()
            .map(|v| (v.0, SensorResult::<f64> { value: v.1 }.into()))
            .collect())
    }
}

impl SensorT<f64> for Tmp36Sensor {
    fn get_readings(&self) -> Result<TypedReadingsResult<f64>, SensorError> {
        let reading = self
            .reader
            .lock()
            .map_err(|_| SensorError::SensorGenericError("failed to get sensor lock"))?
            .read()?;
        let mut x = HashMap::new();
        x.insert("millivolts".to_string(), reading as f64);
        Ok(x)
    }
}
