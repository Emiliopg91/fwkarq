use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("Error marshalling value to {0}:\n  {1}")]
    MarshallError(String, Box<dyn std::error::Error>),
    #[error("Error unmarshalling value to {0}:\n  {1}")]
    UnmarshallError(String, Box<dyn std::error::Error>),
}

pub type SerializationResult<T> = std::result::Result<T, SerializationError>;
