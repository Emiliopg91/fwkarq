use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::mpsc::{self, SyncSender};
use std::thread::{self, JoinHandle};

use crate::logger::level::Level;
use crate::logger::sink::Sink;

pub struct FileSink {
    level: Level,
    tx: Option<SyncSender<String>>,
    worker: Option<JoinHandle<()>>,
}

impl FileSink {
    pub fn new<T>(path: T, level: Level) -> std::io::Result<Self>
    where
        T: AsRef<Path>,
    {
        let file = OpenOptions::new().create(true).append(true).open(&path)?;

        let (tx, rx) = mpsc::sync_channel(100);
        let tx = Some(tx);

        let path = path.as_ref().to_owned();
        let worker = Some(thread::spawn(move || {
            let mut writer = BufWriter::new(file);
            while let Ok(msg) = rx.recv() {
                if let Err(e) = writeln!(writer, "{}", msg) {
                    eprintln!("Error writing log to {}:\n  {}", path.display(), e)
                } else {
                    let _ = writer.flush();
                }
            }
        }));

        Ok(Self { level, tx, worker })
    }
}

impl Sink for FileSink {
    fn sink_message(&self, msg: &str, level: Level) {
        if level >= self.level
            && let Some(tx) = &self.tx
        {
            let _ = tx.send(msg.to_string());
        }
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            drop(tx);
        }

        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
