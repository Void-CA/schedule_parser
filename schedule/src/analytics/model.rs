use std::collections::HashMap;

use crate::domain::models::{Class, Day, Major};

pub struct ScheduleAnalytics {
    classes: Vec<Class>,
}

impl ScheduleAnalytics {
    pub fn new(classes: Vec<Class>) -> Self {
        Self { classes }
    }

    pub fn classes_per_day(&self) -> HashMap<Day, usize> {
        let mut map = HashMap::new();

        for class in &self.classes {
            *map.entry(class.day).or_insert(0) += 1;
        }

        map
    }

    pub fn classes_per_major(&self) -> HashMap<Major, usize> {
        let mut map = HashMap::new();

        for class in &self.classes {
            for major in &class.majors {
                *map.entry(*major).or_insert(0) += 1;
            }
        }

        map
    }

    pub fn room_usage(&self) -> HashMap<String, usize> {
        let mut map = HashMap::new();

        for class in &self.classes {
            *map.entry(class.room.clone()).or_insert(0) += 1;
        }

        map
    }

    pub fn detect_conflicts(&self) -> Vec<(&Class, &Class)> {
        let mut conflicts = Vec::new();

        for i in 0..self.classes.len() {
            for j in (i + 1)..self.classes.len() {
                let a = &self.classes[i];
                let b = &self.classes[j];

                if a.day == b.day
                    && a.start_block == b.start_block
                    && a.room == b.room
                {
                    conflicts.push((a, b));
                }
            }
        }

        conflicts
    }

    pub fn summary(&self) {
        println!("Total clases: {}", self.classes.len());

        println!("\nClases por día:");
        for (day, count) in self.classes_per_day() {
            println!("{:?}: {}", day, count);
        }

        println!("\nAulas más usadas:");
        for (room, count) in self.room_usage() {
            println!("{}: {}", room, count);
        }
    }
}