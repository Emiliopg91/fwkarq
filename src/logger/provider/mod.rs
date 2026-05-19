#[cfg(test)]
mod tests;

use crate::logger::{
    Logger,
    level::Level,
    sink::{Sink, stdout::StdoutSink},
};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

pub struct Provider {
    default_level: Level,
    default_pattern: String,
    default_sinks: Vec<Arc<dyn Sink + Send + Sync>>,
    logger_map: HashMap<String, Arc<Logger>>,
}

static PROVIDER: LazyLock<RwLock<Provider>> = LazyLock::new(|| RwLock::new(Provider::new()));

impl Provider {
    fn new() -> Self {
        Self {
            default_level: Level::INFO,
            logger_map: HashMap::new(),
            default_pattern: "[%d][%n][%l] - %m".to_string(),
            default_sinks: vec![Arc::new(StdoutSink::new(Level::INFO))],
        }
    }

    pub fn get_logger(logger_name: &str) -> Arc<Logger> {
        {
            let r_prov = PROVIDER.read().unwrap();
            if let Some(logger) = r_prov.logger_map.get(logger_name) {
                return logger.clone();
            }
        }

        let mut w_prov = PROVIDER.write().unwrap();

        if let Some(logger) = w_prov.logger_map.get(logger_name) {
            return logger.clone();
        }

        let inst = Arc::new(Logger::new(
            logger_name,
            w_prov.default_level,
            &w_prov.default_pattern,
        ));

        w_prov
            .logger_map
            .insert(logger_name.to_string(), inst.clone());

        inst
    }

    pub fn get_sinks() -> Vec<Arc<dyn Sink + Send + Sync>> {
        PROVIDER.read().unwrap().default_sinks.clone()
    }

    pub fn set_sinks(sinks: Vec<Arc<dyn Sink + Send + Sync>>) {
        PROVIDER.write().unwrap().default_sinks = sinks;
    }

    pub fn get_level() -> Level {
        PROVIDER.read().unwrap().default_level
    }

    pub fn set_level(level: Level) {
        PROVIDER.write().unwrap().default_level = level;
    }

    pub fn get_pattern() -> String {
        PROVIDER.read().unwrap().default_pattern.clone()
    }

    pub fn set_pattern(pattern: &str) {
        PROVIDER.write().unwrap().default_pattern = pattern.to_string();
    }
}
