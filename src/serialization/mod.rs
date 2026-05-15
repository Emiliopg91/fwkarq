pub mod error;
pub mod json;
pub mod toml;
pub mod yaml;


use serde::{Serialize, de::DeserializeOwned};

use crate::{
    serialization::error::{SerializationError, Result},
    utils::file::FileUtils,
};

pub trait Serializer {
    fn serialize<S: Serialize>(obj: &S) -> Result<String>;
    fn deserialize<T: DeserializeOwned>(string: &str) -> Result<T>;

    fn get_type() -> String;

    fn serialize_to_file<P: AsRef<std::path::Path>, S: Serialize>(
        obj: &S,
        path: P,
    ) -> Result<()> {
        let content = Self::serialize(obj)?;
        FileUtils::write(path, content)
            .map_err(|e| SerializationError::MarshallError("JSON".to_string(), Box::new(e)))
    }
    fn deserialize_from_file<P: AsRef<std::path::Path>, T: DeserializeOwned>(
        path: P,
    ) -> Result<T> {
        Self::deserialize(
            &FileUtils::read(path).map_err(|e| {
                SerializationError::UnmarshallError("JSON".to_string(), Box::new(e))
            })?,
        )
    }
}
