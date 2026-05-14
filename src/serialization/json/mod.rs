use serde::{Serialize, de::DeserializeOwned};

use crate::serialization::{
    Serializer,
    error::{SerializationError, SerializationResult},
};

pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn deserialize<T: DeserializeOwned>(string: &str) -> SerializationResult<T> {
        serde_json::from_str(string)
            .map_err(|e| SerializationError::UnmarshallError("JSON".to_string(), Some(Box::new(e))))
    }

    fn serialize<T: Serialize>(obj: &T) -> SerializationResult<String> {
        serde_json::to_string_pretty(obj)
            .map_err(|e| SerializationError::MarshallError("JSON".to_string(), Some(Box::new(e))))
    }
}
