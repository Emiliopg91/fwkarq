use std::{collections::HashMap, path::PathBuf};

use crate::{
    serialization::{Serializer, yaml::YamlSerializer},
    translator::Translator,
    utils::file::FileUtils,
};

fn initialize() -> PathBuf {
    let path = FileUtils::new_tmp_file().unwrap();
    let content: HashMap<&str, HashMap<&str, &str>> = HashMap::from([
        ("lit1", HashMap::from([("en", "Hello")])),
        ("lit2", HashMap::from([("en", "Value: {}-{}")])),
    ]);

    YamlSerializer::serialize_to_file(&content, &path).unwrap();

    path
}

#[test]
fn test_01_initialization() {
    Translator::new(initialize()).unwrap();
}

#[test]
fn test_02_translate() {
    let translator = Translator::new(initialize()).unwrap();
    assert!(translator.translate("lit1") != "lit1");
}

#[test]
fn test_03_translate_with_data() {
    let translator = Translator::new(initialize()).unwrap();
    assert!(translator.translate_with_data("lit2", ["1", "2"]) == "Value: 1-2");
}
