pub mod error;
pub mod json;
pub mod toml;
pub mod yaml;

use std::any::type_name;

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    serialization::error::{Result, SerializationError},
    utils::file::FileUtils,
};

pub trait Serializer {
    fn serialize<S: Serialize>(obj: &S) -> Result<String>;
    fn deserialize<T: DeserializeOwned>(string: &str) -> Result<T>;

    fn get_type() -> String;

    fn serialize_to_file<P: AsRef<std::path::Path>, S: Serialize>(obj: &S, path: &P) -> Result<()> {
        let content = Self::serialize(obj)?;
        FileUtils::write(path, false, &content)
            .map_err(|e| SerializationError::MarshallError(Self::get_type(), Box::new(e)))
    }
    fn deserialize_from_file<T: DeserializeOwned, P: AsRef<std::path::Path>>(
        path: &P,
    ) -> Result<T> {
        Self::deserialize(&FileUtils::read(path).map_err(|e| {
            SerializationError::UnmarshallError(
                Self::get_type(),
                type_name::<T>().to_string(),
                Box::new(e),
            )
        })?)
    }
}
