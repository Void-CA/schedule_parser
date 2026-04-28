use std::collections::HashMap;

use crate::domain::models::{Class, Day, Major};

pub struct ScheduleAnalytics {
    pub classes: Vec<Class>,
}

impl ScheduleAnalytics {
    pub fn new(classes: Vec<Class>) -> Self {
        Self { classes }
    }

    pub fn total_capacity(&self) -> usize {
        self.classes
            .iter()
            .map(|c| c.day)
            .collect::<std::collections::HashSet<_>>()
            .len()
            * 8
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

    pub fn detect_advanced_conflicts(&self) -> Vec<String> {
        let mut messages = Vec::new();

        for (i, a) in self.classes.iter().enumerate() {
            for b in self.classes.iter().skip(i + 1) {
                // Verificar si los bloques de tiempo se solapan
                let overlaps = a.day == b.day && 
                    (a.start_block < b.end_block && b.start_block < a.end_block);

                if overlaps {
                    // Conflicto de Aula: Dos clases en el mismo sitio
                    if a.room == b.room {
                        messages.push(format!("Conflict: Room {} is double-booked for {} and {}", a.room, a.subject, b.subject));
                    }
                    // Conflicto de Profesor: El profesor no es un electrón, no puede estar en dos sitios
                    if a.professor == b.professor {
                        messages.push(format!("Conflict: Professor {} has simultaneous classes in {} and {}", a.professor, a.room, b.room));
                    }
                    // Conflicto de Grupo: Un mismo grupo/año no puede dividirse
                    if a.group == b.group && a.majors.iter().any(|m| b.majors.contains(m)) {
                        messages.push(format!("Conflict: Group {} ({:?}) has overlapping subjects", a.group, a.majors));
                    }
                }
            }
        }
        messages
    }
}