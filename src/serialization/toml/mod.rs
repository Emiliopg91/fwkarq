#[cfg(test)]
mod tests;

use serde::{Serialize, de::DeserializeOwned};

use crate::serialization::{
    Serializer,
    error::{SerializationError, SerializationResult},
};

pub struct TomlSerializer;

impl Serializer for TomlSerializer {
    fn deserialize<T: DeserializeOwned>(string: &str) -> SerializationResult<T> {
        toml::from_str(string)
            .map_err(|e| SerializationError::UnmarshallError("TOML".to_string(), Some(Box::new(e))))
    }

    fn serialize<T: Serialize>(obj: &T) -> SerializationResult<String> {
        toml::to_string(obj)
            .map_err(|e| SerializationError::MarshallError("TOML".to_string(), Some(Box::new(e))))
    }
}
