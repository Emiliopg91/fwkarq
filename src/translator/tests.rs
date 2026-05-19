use std::collections::HashMap;

use crate::{
    serialization::{Serializer, yaml::YamlSerializer},
    translator::Translator,
    utils::file::FileUtils,
};

fn initialize() {
    let path = FileUtils::new_tmp_file().unwrap();
    let content: HashMap<&str, HashMap<&str, &str>> = HashMap::from([
        ("lit1", HashMap::from([("en", "Hello")])),
        ("lit2", HashMap::from([("en", "Value: {}-{}")])),
    ]);

    YamlSerializer::serialize_to_file(&content, &path).unwrap();

    Translator::initialize(path).unwrap();
}

#[test]
fn test_01_initialization() {
    println!();
    initialize();
}

#[test]
fn test_02_translate() {
    assert!(Translator::translate("lit1") != "lit1");
}

#[test]
fn test_03_translate_with_data() {
    assert!(Translator::translate_with_data("lit2", ["1", "2"]) == "Value: 1-2");
}
