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
    thread, time,
};

pub fn register_models(registry: &mut ComponentRegistry) -> Result<(), RegistryError> {
    registry.register_sensor("tmp36", &Tmp36Sensor::from_config)?;
    Ok(())
}

#[derive(DoCommand)]
pub struct Tmp36Sensor {
    reader: AnalogReaderType<u16>,
    offset: f64,
    num_readings: i32,
}

impl Tmp36Sensor {
    pub fn from_config(_cfg: ConfigType, deps: Vec<Dependency>) -> Result<SensorType, SensorError> {
        let board = get_board_from_dependencies(deps);
        if board.is_none() {
            return Err(SensorError::ConfigError("sensor missing board attribute"));
        }
        let board_unwrapped = board.unwrap();

        let offset = _cfg.get_attribute::<f64>("offset").unwrap_or(0.0);
        let num_readings = _cfg.get_attribute::<i32>("num_readings").unwrap_or(1);

        if num_readings < 1 {
            return Err(SensorError::ConfigError(
                "num_readings must be an integer greater than 1",
            ));
        }

        if let Ok(analog_reader_name) = _cfg.get_attribute::<String>("analog_reader") {
            if let Ok(reader) = board_unwrapped.get_analog_reader_by_name(analog_reader_name) {
                Ok(Arc::new(Mutex::new(Self {
                    reader,
                    offset,
                    num_readings,
                })))
            } else {
                Err(SensorError::ConfigError("failed to get analog reader"))
            }
        } else {
            Err(SensorError::ConfigError(
                "failed to get 'analog_reader' value from config",
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
        // TODO: deal with noise -- read X times and return the median

        let mut readings = Vec::new();
        for i in 0..self.num_readings {
            let reading = self
                .reader
                .lock()
                .map_err(|_| SensorError::SensorGenericError("failed to get sensor lock"))?
                .read()?;
            let readingf = reading as i16;
            readings.push(readingf);

            if i < self.num_readings - 1 {
                thread::sleep(time::Duration::from_millis(1));
            }
        }

        // calculate median
        readings.sort();
        let mid = readings.len() / 2;
        let median_reading = readings[mid];

        let mut x = HashMap::new();
        x.insert(
            "temperature_c".to_string(),
            (median_reading - 500) as f64 / 10.0 + self.offset,
        ); // calculated final temp result

        // debugging fields
        x.insert("temperature_raw".to_string(), (median_reading - 500) as f64 / 10.0); // temp pre-offset
        x.insert("milliv".to_string(), median_reading as f64); // raw reading from sensor
        x.insert("num_readings".to_string(), readings.len() as f64); // number of readings read

        Ok(x)
    }
}
