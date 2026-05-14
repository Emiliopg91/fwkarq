#[cfg(test)]
mod tests;

use serde::{Serialize, de::DeserializeOwned};

use crate::serialization::{
    Serializer,
    error::{SerializationError, SerializationResult},
};

pub struct YamlSerializer;

impl Serializer for YamlSerializer {
    fn deserialize<T: DeserializeOwned>(string: &str) -> SerializationResult<T> {
        serde_yaml::from_str(string)
            .map_err(|e| SerializationError::UnmarshallError("YAML".to_string(), Some(Box::new(e))))
    }

    fn serialize<T: Serialize>(obj: &T) -> SerializationResult<String> {
        serde_yaml::to_string(obj)
            .map_err(|e| SerializationError::MarshallError("YAML".to_string(), Some(Box::new(e))))
    }
}
