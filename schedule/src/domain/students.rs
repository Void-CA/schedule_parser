use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::models::Class;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentSchedule {
    pub by_year: HashMap<u8, Vec<Class>>,
}

impl StudentSchedule {
    pub fn new() -> Self {
        Self {
            by_year: HashMap::new(),
        }
    }

    pub fn add_year(&mut self, year: u8, classes: Vec<Class>) {
        self.by_year.insert(year, classes);
    }
}