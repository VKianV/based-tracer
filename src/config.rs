use crate::app_error::ConfigError;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Error},
};

pub struct Config {
    pub map: HashMap<String, String>,
}

impl Config {
    pub fn load_config(path: &str) -> Result<Self, Error> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut map = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // Split at first '='
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();

                map.insert(key, value);
            }
        }

        Ok(Config { map })
    }

    pub fn get_str(&self, key: &str) -> Result<&str, ConfigError> {
        self.map
            .get(key)
            .ok_or(ConfigError::KeyNotFound(key.to_string()))
            .map(|s| s.as_str())
    }

    pub fn get_f64(&self, key: &str) -> Result<f64, ConfigError> {
        self.get_str(key)?.parse::<f64>().map_err(Into::into)
    }

    pub fn get_u32(&self, key: &str) -> Result<u32, ConfigError> {
        self.get_str(key)?.parse::<u32>().map_err(Into::into)
    }

    pub fn get_bool(self, key: &str) -> Result<bool, ConfigError> {
        self.get_str(key)?.parse::<bool>().map_err(Into::into)
    }
}
