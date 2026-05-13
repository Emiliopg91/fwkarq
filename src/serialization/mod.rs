#[cfg(test)]
mod tests;

pub mod error;

use serde::{Serialize, de::DeserializeOwned};

use crate::serialization::error::SerializationError;

pub trait Serializer {
    fn serialize<T: Serialize>(obj: &T) -> Result<String, SerializationError>;
    fn deserialize<T: DeserializeOwned>(string: &str) -> Result<T, SerializationError>;
}

pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn deserialize<T: DeserializeOwned>(string: &str) -> Result<T, SerializationError> {
        serde_json::from_str(string)
            .map_err(|e| SerializationError::UnmarshallError("JSON".to_string(), Some(Box::new(e))))
    }

    fn serialize<T: Serialize>(obj: &T) -> Result<String, SerializationError> {
        serde_json::to_string_pretty(obj)
            .map_err(|e| SerializationError::MarshallError("JSON".to_string(), Some(Box::new(e))))
    }
}

pub struct YamlSerializer;

impl Serializer for YamlSerializer {
    fn deserialize<T: DeserializeOwned>(string: &str) -> Result<T, SerializationError> {
        serde_yaml::from_str(string)
            .map_err(|e| SerializationError::UnmarshallError("YAML".to_string(), Some(Box::new(e))))
    }

    fn serialize<T: Serialize>(obj: &T) -> Result<String, SerializationError> {
        serde_yaml::to_string(obj)
            .map_err(|e| SerializationError::MarshallError("YAML".to_string(), Some(Box::new(e))))
    }
}
