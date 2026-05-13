use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SerializationError {
    MarshallError(String, Option<Box<dyn Error>>),
    UnmarshallError(String, Option<Box<dyn Error>>),
}

impl Error for SerializationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::MarshallError(_, e) => e.as_deref(),
            Self::UnmarshallError(_, e) => e.as_deref(),
        }
    }
}

impl Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = match self {
            Self::MarshallError(lang, _) => {
                format!("Error marshalling value to {}", lang)
            }
            Self::UnmarshallError(lang, _) => {
                format!("Error unmarshalling value to {}", lang)
            }
        };

        if let Some(err) = self.source() {
            message.push_str(&format!(". Caused by: {}", err));
        }

        write!(f, "{}", message)?;

        Ok(())
    }
}
