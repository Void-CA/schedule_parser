use std::collections::HashMap;

use crate::{analytics::model::ScheduleAnalytics, domain::models::{AcademicBlock, Day}};

impl ScheduleAnalytics {
    // Horas totales por profesor (útil para nóminas o carga docente)
    pub fn professor_workload(&self) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        for class in &self.classes {
            let duration = class.end_block.id() - class.start_block.id();
            *map.entry(class.professor.clone()).or_insert(0) += duration;
        }
        map
    }

    pub fn peak_load_analysis(&self) -> HashMap<(Day, AcademicBlock), usize> {
        let mut heatmap = HashMap::new();

        for class in &self.classes {
            let start = class.start_block.id();
            let end = class.end_block.id();

            for id in start..end {
                if let Some(block) = AcademicBlock::from_id(id) {
                    *heatmap.entry((class.day, block))
                        .or_insert(0) += 1;
                }
            }
        }

        heatmap
    }

    pub fn room_efficiency(&self) -> HashMap<String, f64> {
        let mut usage: HashMap<String, usize> = HashMap::new();

        for class in &self.classes {
            let duration = class.end_block.id() - class.start_block.id();
            *usage.entry(class.room.clone())
                .or_insert(0) += duration;
        }

        usage.into_iter()
            .map(|(room, used)| {
                let total_possible = 8 * 6; // 8 bloques por día × 6 días
                let efficiency = (used as f64 / total_possible as f64) * 100.0;
                (room, efficiency)
            })
            .collect()
    }
}