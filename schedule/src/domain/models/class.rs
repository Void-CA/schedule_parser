use serde::{Deserialize, Serialize};

use super::day::Day;
use super::block::AcademicBlock;
use super::major::Major;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub day: Day,
    pub start_block: AcademicBlock,
    pub end_block: AcademicBlock,
    pub subject: String,
    pub majors: Vec<Major>,
    pub group: u8,
    pub professor: String,
    pub room: String,
}