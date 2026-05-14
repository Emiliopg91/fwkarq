#[cfg(test)]
mod tests;

pub mod errors;

use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    serialization::{Serializer, yaml::YamlSerializer},
    settings::errors::{Result, SettingsError},
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
    pub fn load(file_path: &PathBuf) -> Result<Self> {
        let content = FileUtils::read(file_path)
            .map_err(|e| SettingsError::LoadError(file_path.to_path_buf(), Box::new(e)))?;
        Ok(Self {
            value: YamlSerializer::deserialize(&content)
                .map_err(|e| SettingsError::LoadError(file_path.to_path_buf(), Box::new(e)))?,
        })
    }

    pub fn save<P>(&self, file_path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let file_path = file_path.as_ref();
        let content = YamlSerializer::serialize(&self.value)
            .map_err(|e| SettingsError::SaveError(file_path.to_path_buf(), Box::new(e)))?;
        FileUtils::write(file_path, content)
            .map_err(|e| SettingsError::SaveError(file_path.to_path_buf(), Box::new(e)))?;
        Ok(())
    }
}
