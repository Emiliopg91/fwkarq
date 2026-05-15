pub mod level;
pub mod provider;
pub mod sink;

#[cfg(test)]
mod tests;

use chrono::Local;

use crate::logger::{
    level::Level,
    sink::{Sink, stdout::StdoutSink},
};

type TokenFn = fn(message: &str, name: &str, level: Level) -> String;

pub struct Logger {
    name: String,
    level: Level,
    sinks: Vec<Box<dyn Sink>>,
    pattern: String,
    token_fn: Vec<TokenFn>,
}

impl Logger {
    pub fn new(name: &str, level: Level, pattern: &str) -> Self {
        let mut chars = pattern.chars().peekable();
        let mut token_fn: Vec<TokenFn> = Vec::new();
        let mut pattern = String::new();

        while let Some(c) = chars.next() {
            if c == '%' {
                match chars.peek() {
                    Some('%') => {
                        // Escaped %
                        chars.next();
                        pattern.push('%');
                    }
                    Some(next) => {
                        let func: Option<TokenFn> = match next {
                            'd' => Some(|_, _, _| {
                                Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
                            }),
                            'n' => Some(|_, name, _| format!("{:15.15}", name)),
                            'l' => Some(|_, _, level| level.to_string()),
                            'm' => Some(|msg, _, _| msg.to_string()),
                            _ => None,
                        };
                        match func {
                            Some(f) => {
                                pattern.push_str(&format!("{{{}}}", token_fn.len()));
                                token_fn.push(f);
                                chars.next();
                            }
                            None => pattern.push(*next),
                        }
                    }
                    None => {
                        pattern.push('%');
                    }
                }
            } else {
                pattern.push(c);
            }
        }

        Self {
            name: name.to_string(),
            pattern,
            token_fn,
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

    fn format_message(&self, message: &str, name: &str, level: Level) -> String {
        let mut formatted = self.pattern.clone();
        for (i, token_fn) in self.token_fn.iter().enumerate() {
            formatted = formatted.replace(&format!("{{{}}}", i), &token_fn(message, name, level));
        }
        formatted
    }

    fn log(&self, message: &str, level: Level) {
        if self.is_level_enabled(level) {
            for sink in &self.sinks {
                sink.as_ref()
                    .sink_message(&self.format_message(message, &self.name, level), level);
            }
        }
    }
}
