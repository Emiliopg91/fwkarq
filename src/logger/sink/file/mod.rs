use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::Mutex;

use crate::logger::level::Level;
use crate::logger::sink::Sink;

pub struct FileSink {
    writer: Mutex<BufWriter<std::fs::File>>,
    level: Level,
}

impl FileSink {
    pub fn new(path: &str, level: Level) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Self {
            writer: Mutex::new(BufWriter::new(file)),
            level,
        })
    }

    fn format_message(&self, msg: &str, name: &str, level: Level) -> String {
        format!("[{:?}] {} - {}", level, name, msg)
    }
}

impl Sink for FileSink {
    fn sink_message(&self, msg: &str, name: &str, level: Level) {
        if level >= self.level {
            let line = self.format_message(msg, name, level);

            if let Ok(mut writer) = self.writer.lock() {
                let _ = writeln!(writer, "{}", line);
                let _ = writer.flush();
            }
        }
    }
}
