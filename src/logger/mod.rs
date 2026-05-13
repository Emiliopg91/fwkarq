pub mod level;
pub mod provider;
pub mod sink;

#[cfg(test)]
mod tests;

use crate::logger::{
    level::Level,
    sink::{Sink, stdout::StdoutSink},
};

pub struct Logger {
    name: String,
    level: Level,
    sinks: Vec<Box<dyn Sink>>,
}

impl Logger {
    pub fn new(name: &str, level: Level) -> Self {
        Self {
            name: name.to_string(),
            level,
            sinks: vec![Box::new(StdoutSink::new(level))],
        }
    }

    pub fn get_sinks(&self) -> &Vec<Box<dyn Sink>> {
        self.sinks.as_ref()
    }

    pub fn set_sinks(&mut self, sinks: Vec<Box<dyn Sink>>) {
        self.sinks = sinks;
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_level(&self) -> Level {
        self.level
    }

    pub fn set_level(&mut self, level: Level) {
        self.level = level
    }

    pub fn is_level_enabled(&self, level: Level) -> bool {
        self.level <= level
    }

    pub fn debug(&self, message: &str) {
        self.log(message, Level::DEBUG);
    }

    pub fn info(&self, message: &str) {
        self.log(message, Level::INFO);
    }

    pub fn warning(&self, message: &str) {
        self.log(message, Level::WARNING);
    }

    pub fn error(&self, message: &str) {
        self.log(message, Level::ERROR);
    }

    pub fn critical(&self, message: &str) {
        self.log(message, Level::CRITICAL);
    }

    fn log(&self, message: &str, level: Level) {
        if self.is_level_enabled(level) {
            for sink in &self.sinks {
                sink.as_ref().sink_message(message, &self.name, level);
            }
        }
    }
}
