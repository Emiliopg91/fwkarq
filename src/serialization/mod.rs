#[cfg(test)]
mod tests;

pub mod error;
pub mod json;
pub mod yaml;

use serde::{Serialize, de::DeserializeOwned};

use crate::serialization::error::SerializationResult;

pub trait Serializer {
    fn serialize<T: Serialize>(obj: &T) -> SerializationResult<String>;
    fn deserialize<T: DeserializeOwned>(string: &str) -> SerializationResult<T>;
}
