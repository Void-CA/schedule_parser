use crate::extraction::models::class::RawClass;
use crate::domain::models::{Class, Day, Major, AcademicBlock};

pub struct Normalizer;

impl Normalizer {
    pub fn normalize(raw: RawClass) -> Option<Class> {
        let day = Day::from_abbrev(&raw.day)?;

        let group = raw.group.parse::<u8>().ok()?;

        let majors = raw.oferta_raw
            .split(',')
            .filter_map(|m| Major::from_str(m.trim()))
            .collect::<Vec<_>>();

        let (start_block, end_block) =
            Self::map_time_to_blocks(&raw.start_time, &raw.end_time)?;

        Some(Class {
            day,
            start_block,
            end_block,
            subject: raw.subject,
            majors,
            group,
            professor: raw.professor,
            room: raw.room,
        })
    }

    fn map_time_to_blocks(
        start: &str,
        end: &str
    ) -> Option<(AcademicBlock, AcademicBlock)> {
        match (start, end) {
            ("08:00 am", "09:40 am") => Some((AcademicBlock::Morning1, AcademicBlock::Morning2)),
            ("08:50 am", "09:40 am") => Some((AcademicBlock::Morning2, AcademicBlock::Morning2)),
            ("10:00 am", "11:40 am") => Some((AcademicBlock::Morning3, AcademicBlock::Morning4)),
            ("01:00 pm", "02:40 pm") => Some((AcademicBlock::Afternoon1, AcademicBlock::Afternoon2)),
            ("03:00 pm", "04:40 pm") => Some((AcademicBlock::Afternoon3, AcademicBlock::Afternoon4)),
            _ => None,
        }
    }
}