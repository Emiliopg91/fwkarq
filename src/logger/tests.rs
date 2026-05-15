use crate::logger::{Logger, level::Level};

const NAME: &str = "LoggerName";
const LEVEL: Level = Level::INFO;

fn initialize_logger() -> Logger {
    Logger::new(NAME, LEVEL, "[%d][%n][%l] - %m")
}

#[test]
fn test_01_test_constructor() {
    let logger = initialize_logger();
    assert_eq!(logger.get_level(), LEVEL);
    assert_eq!(logger.get_name(), NAME);
}

#[test]
fn test_02_test_set_level() {
    let new_level = Level::CRITICAL;
    let mut logger = initialize_logger();
    logger.set_level(new_level);
    assert_eq!(logger.get_level(), new_level);
}

#[test]
fn test_03_test_is_level_enabled() {
    let logger = initialize_logger();
    assert!(logger.is_level_enabled(Level::CRITICAL));
    assert!(logger.is_level_enabled(logger.level));
    assert!(!logger.is_level_enabled(Level::DEBUG));
}
