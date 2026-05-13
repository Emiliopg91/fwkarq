use crate::logger::{level::Level, sink::Sink};

pub struct StdoutSink {
    level: Level,
}

impl StdoutSink {
    pub fn new(level: Level) -> Self {
        Self { level }
    }
}

impl Sink for StdoutSink {
    fn sink_message(&self, msg: &str, name: &str, level: Level) {
        if level >= self.level {
            if level < Level::ERROR {
                println!("{}", self.format_message(msg, name, level))
            } else {
                eprintln!("{}", self.format_message(msg, name, level))
            }
        }
    }
}
