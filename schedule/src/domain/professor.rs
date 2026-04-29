use std::collections::HashMap;

use serde::Serialize;

use crate::{domain::models::{Class, Day}};

#[derive(Debug, Clone, Serialize)]
pub struct ProfessorSchedule {
    pub by_day: HashMap<Day, Vec<Class>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfessorSchedules {
    pub map: HashMap<String, ProfessorSchedule>,
}

impl ProfessorSchedules {
    pub fn build(classes: &[Class]) -> Self {
        let mut map: HashMap<String, HashMap<Day, Vec<Class>>> = HashMap::new();

        for class in classes {
            map.entry(class.professor.clone())
                .or_default()
                .entry(class.day)
                .or_default()
                .push(class.clone());
        }

        let map = map
            .into_iter()
            .map(|(prof, by_day)| {
                (
                    prof,
                    ProfessorSchedule { by_day }
                )
            })
            .collect();

        Self { map }
    }
}