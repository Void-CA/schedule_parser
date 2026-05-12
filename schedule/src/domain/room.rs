use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::models::{Class, Day};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSchedule {
    pub by_day: HashMap<Day, Vec<Class>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSchedules {
    pub map: HashMap<String, RoomSchedule>,
}

impl RoomSchedules {
    pub fn build(classes: &[Class]) -> Self {
        let mut temp: HashMap<String, HashMap<Day, Vec<Class>>> = HashMap::new();

        for class in classes {
            temp.entry(class.room.clone())
                .or_default()
                .entry(class.day)
                .or_default()
                .push(class.clone());
        }

        let map = temp
            .into_iter()
            .map(|(room, by_day)| (room, RoomSchedule { by_day }))
            .collect();

        Self { map }
    }
}