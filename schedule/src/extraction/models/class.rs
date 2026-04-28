use crate::extraction::models::{detail::RawDetail, row::RawRow};

pub struct RawClass {
    pub day: String,
    pub start_time: String,
    pub end_time: String,
    pub subject: String,
    pub oferta_raw: String,
    pub group: String,
    pub professor: String,
    pub room: String,
}

impl RawClass {
    pub fn from_parts(row: RawRow, detail: RawDetail) -> Self {
        Self {
            day: row.day,
            start_time: row.start_time,
            end_time: row.end_time,
            subject: detail.subject,
            oferta_raw: detail.oferta_raw,
            group: detail.group,
            professor: detail.professor_name,
            room: detail.room,
        }
    }
}