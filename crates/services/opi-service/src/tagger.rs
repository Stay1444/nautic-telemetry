use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const FILE_NAME: &str = "tags.yml";

#[derive(Deserialize, Serialize, Default)]
pub struct Tagger {
    voltimeters: HashMap<u8, String>,
    thermometers: HashMap<u8, String>,
}

impl Tagger {
    pub fn new() -> Self {
        if let Ok(content) = std::fs::read_to_string(FILE_NAME) {
            return serde_yaml::from_str(&content).unwrap_or(Self::default());
        }

        Self::default()
    }

    fn save(&self) {
        let yaml = serde_yaml::to_string(&self).expect("Serialization should not fail");
        _ = std::fs::write(FILE_NAME, yaml);
    }

    pub fn voltimeter(&mut self, id: u8) -> String {
        if let Some(tag) = self.voltimeters.get(&id) {
            return tag.clone();
        };

        self.voltimeters.insert(id, format!("Voltimeter {id}"));

        self.save();

        self.voltimeter(id)
    }

    pub fn thermometer(&mut self, id: u8) -> String {
        if let Some(tag) = self.thermometers.get(&id) {
            return tag.clone();
        };

        self.thermometers.insert(id, format!("Thermometer {id}"));

        self.save();

        self.thermometer(id)
    }
}
