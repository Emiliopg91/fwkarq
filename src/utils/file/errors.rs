use std::{error::Error, fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum FileError {
    FileDeletionError(PathBuf, std::io::Error),
    FileTouchError(PathBuf, std::io::Error),
    MakeDirError(PathBuf, std::io::Error),
    FileWriteError(PathBuf, std::io::Error),
    FileReadError(PathBuf, std::io::Error),
}

impl Error for FileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FileError::FileDeletionError(_, e)
            | FileError::FileTouchError(_, e)
            | FileError::MakeDirError(_, e)
            | FileError::FileWriteError(_, e)
            | FileError::FileReadError(_, e) => Some(e),
        }
    }
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = match self {
            FileError::FileDeletionError(path, _) => {
                format!("Could not delete {}", path.display())
            }
            FileError::FileTouchError(path, _) => {
                format!("Could not touch {}", path.display())
            }
            FileError::MakeDirError(path, _) => {
                format!("Could not create directory {}", path.display())
            }
            FileError::FileWriteError(path, _) => {
                format!("Could not write content to {}", path.display())
            }
            FileError::FileReadError(path, _) => {
                format!("Could not read content from {}", path.display())
            }
        };

        if let Some(e) = self.source() {
            msg.push_str(&format!(". Caused by: {}", e));
        }

        write!(f, "{}", msg)
    }
}

pub type Result<T> = std::result::Result<T, FileError>;
