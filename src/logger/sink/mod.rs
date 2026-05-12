use chrono::Local;

use crate::logger::level::Level;

pub trait Sink {
    fn sink_message(&self, msg: &str, name: &str, level: &Level);
    fn format_message(&self, message: &str, name: &str, level: &Level) -> String {
        format!(
            "[{}][{:15.15}][{:5.5}] - {}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            name,
            level.to_string(),
            message
        )
    }
}

#[derive(Default)]
pub struct StdOutSink {}

impl Sink for StdOutSink {
    fn sink_message(&self, msg: &str, name: &str, level: &Level) {
        if *level < Level::ERROR {
            println!("{}", self.format_message(msg, name, level))
        } else {
            eprintln!("{}", self.format_message(msg, name, level))
        }
    }
}
