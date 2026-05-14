#[cfg(test)]
mod tests;

use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    serialization::{Serializer, YamlSerializer},
    utils::file::FileUtils,
};

#[derive(Default)]
pub struct Settings<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    value: T,
}

impl<T> Deref for Settings<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Settings<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Settings<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    pub fn load(file_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = FileUtils::read(file_path)?;
        Ok(Self {
            value: YamlSerializer::deserialize(&content)?,
        })
    }

    pub fn save(&self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = YamlSerializer::serialize(&self.value)?;
        FileUtils::write(file_path, content)?;
        Ok(())
    }
}
