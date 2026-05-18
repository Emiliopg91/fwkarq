#[cfg(test)]
mod tests;

pub mod error;

use std::{collections::HashMap, env, iter, path::Path};

use crate::{
    serialization::{Serializer, yaml::YamlSerializer},
    translator::error::{Result, TranslationError},
};

pub struct Translator {
    msg_map: HashMap<String, String>,
}

impl Translator {
    const DEFAULT_LANG: &str = "en";

    pub fn new<T>(file_path: T) -> Result<Self>
    where
        T: AsRef<Path>,
    {
        let mut lang = env::var("LANG").unwrap_or(Self::DEFAULT_LANG.to_string());
        if lang == "C" {
            lang = Self::DEFAULT_LANG.to_string();
        }

        let content_mapped = YamlSerializer::deserialize_from_file::<
            HashMap<String, HashMap<String, String>>,
            _,
        >(&file_path)
        .map_err(|e| {
            TranslationError::ErrorLoadingFile(file_path.as_ref().display().to_string(), e)
        })?;
        let mut msg_map: HashMap<String, String> = HashMap::new();

        for key in content_mapped.keys() {
            let entry = content_mapped.get(key).unwrap();
            let mut literal = entry.get(&lang);
            if literal.is_none() {
                literal = entry.get(Self::DEFAULT_LANG);
                if literal.is_none() {
                    literal = Some(key);
                }
            }
            msg_map.insert(key.clone(), literal.unwrap().clone());
        }

        Ok(Self { msg_map })
    }

    pub fn translate<K: AsRef<str>>(&self, key: K) -> String {
        self.translate_with_data(key, iter::empty::<String>())
    }

    pub fn translate_with_data<K, D, I>(&self, key: K, data: I) -> String
    where
        K: AsRef<str>,
        I: IntoIterator<Item = D>,
        D: AsRef<str>,
    {
        let key = key.as_ref();

        if !self.msg_map.contains_key(key) {
            return key.to_string();
        }

        let mut result = self.msg_map.get(key).unwrap_or(&key.to_string()).to_owned();

        for item in data {
            result = result.replacen("{}", item.as_ref(), 1);
        }

        result
    }
}
