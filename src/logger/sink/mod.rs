pub mod file;
pub mod stdout;

use chrono::Local;

use crate::logger::level::Level;

pub trait Sink: Send + Sync {
    fn sink_message(&self, msg: &str, name: &str, level: Level);
    fn format_message(&self, message: &str, name: &str, level: Level) -> String {
        format!(
            "[{}][{:15.15}][{:5.5}] - {}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            name,
            level.to_string(),
            message
        )
    }
}
