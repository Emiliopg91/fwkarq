mod tests {
    use std::{env, fs};

    use serde_derive::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::settings::Settings;
    #[derive(Serialize, Deserialize, Default)]
    struct ConfigBean {
        age: u8,
        name: String,
    }

    impl ConfigBean {
        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }

        pub fn age(&self) -> u8 {
            self.age
        }

        pub fn set_age(&mut self, age: u8) {
            self.age = age;
        }
    }

    #[test]
    fn test_01_check_constructor() {
        Settings::<ConfigBean>::default();
    }

    #[test]
    fn test_02_check_load() {
        let tmp = env::temp_dir().join(Uuid::new_v4().to_string());
        let content = r#"age: 18
name: Name
"#;
        fs::write(&tmp, &content).unwrap();
        let settings = Settings::<ConfigBean>::load(&tmp).unwrap();
        assert!(settings.age() == 18);
        assert!(settings.name() == "Name");
    }

    #[test]
    fn test_03_check_saves() {
        let mut settings = Settings::<ConfigBean>::default();
        settings.set_age(18);
        settings.set_name("Name");

        let tmp = env::temp_dir().join(Uuid::new_v4().to_string());
        settings.save(&tmp).unwrap();
    }
}
