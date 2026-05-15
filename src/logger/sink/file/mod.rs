use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Mutex;

use crate::logger::level::Level;
use crate::logger::sink::Sink;

pub struct FileSink {
    writer: Mutex<BufWriter<std::fs::File>>,
    level: Level,
}

impl FileSink {
    pub fn new<T>(path: T, level: Level) -> std::io::Result<Self>
    where
        T: AsRef<Path>,
    {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Self {
            writer: Mutex::new(BufWriter::new(file)),
            level,
        })
    }
}

impl Sink for FileSink {
    fn sink_message(&self, msg: &str, level: Level) {
        if level >= self.level
            && let Ok(mut writer) = self.writer.lock()
        {
            let _ = writeln!(writer, "{}", msg);
            let _ = writer.flush();
        }
    }
}
