use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub enum SettingsError {
    LoadError(PathBuf, Box<dyn std::error::Error>),
    SaveError(PathBuf, Box<dyn std::error::Error>),
}

impl std::error::Error for SettingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::LoadError(_, e) | Self::SaveError(_, e) => Some(&**e),
        }
    }
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = match self {
            Self::LoadError(path, _) => format!("Could not load settings from {}", path.display()),
            Self::SaveError(path, _) => format!("Could not save settings to {}", path.display()),
        };

        if let Some(e) = self.source() {
            message.push_str(&format!(". Caused by: {}", e));
        }

        write!(f, "{}", message)
    }
}

pub type Result<T> = std::result::Result<T, SettingsError>;
