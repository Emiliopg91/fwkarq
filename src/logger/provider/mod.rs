#[cfg(test)]
mod tests;

use crate::logger::{Logger, level::Level};
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

pub struct Provider {
    default_level: Level,
    default_pattern: String,
    logger_map: HashMap<String, Arc<Logger>>,
}

static PROVIDER: OnceLock<RwLock<Provider>> = OnceLock::new();

impl Provider {
    fn new() -> Self {
        Self {
            default_level: Level::INFO,
            logger_map: HashMap::new(),
            default_pattern: "[%d][%n][%l] - %m".to_string(),
        }
    }

    fn _get_logger(&mut self, logger_name: &str) -> Arc<Logger> {
        self.logger_map
            .entry(logger_name.to_string())
            .or_insert_with(|| {
                Arc::new(Logger::new(
                    logger_name,
                    self.default_level,
                    &self.default_pattern,
                ))
            })
            .clone()
    }

    pub fn get_logger(logger_name: &str) -> Arc<Logger> {
        PROVIDER
            .get_or_init(|| RwLock::new(Provider::new()))
            .write()
            .unwrap()
            ._get_logger(logger_name)
    }

    pub fn get_level() -> Level {
        PROVIDER
            .get_or_init(|| RwLock::new(Provider::new()))
            .write()
            .unwrap()
            .default_level
    }

    pub fn set_level(level: Level) {
        PROVIDER
            .get_or_init(|| RwLock::new(Provider::new()))
            .write()
            .unwrap()
            .default_level = level;
    }

    pub fn set_pattern(pattern: &str) {
        PROVIDER
            .get_or_init(|| RwLock::new(Provider::new()))
            .write()
            .unwrap()
            .default_pattern = pattern.to_string();
    }
}
