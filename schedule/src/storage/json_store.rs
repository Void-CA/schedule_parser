use std::fs::File;
use std::io::Write;
use serde_json;

use crate::domain::models::Class;

pub struct JsonStore;

impl JsonStore {
    pub fn save(path: &str, data: &Vec<Class>) -> Result<(), String> {
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| e.to_string())?;

        let mut file = File::create(path)
            .map_err(|e| e.to_string())?;

        file.write_all(json.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}