use crate::extraction::models::class::RawClass;
use crate::error::ErrorType;

pub struct Validator;

impl Validator {
    pub fn validate(class: &RawClass) -> Vec<(ErrorType, String)> {
        let mut errors = Vec::new();

        // ---------------- SUBJECT ----------------
        if class.subject.trim().is_empty() {
            errors.push((ErrorType::Subject, "Subject vacío".to_string()));
        }

        // ---------------- ROOM ----------------
        let room = class.room.trim();

        if room.is_empty() {
            errors.push((ErrorType::Room, "Aula vacía".to_string()));
        } else if !room.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) {
            errors.push((ErrorType::Room, "Formato de aula inválido".to_string()));
        }

        // ---------------- OFERTA ----------------
        let valid_majors = ["ICE","IME","IMS","IGI","IEE","IEM","LAF","LCM"];

        let has_valid_major = class.oferta_raw
            .split(',')
            .map(|m| m.trim())
            .any(|m| valid_majors.contains(&m));

        if !has_valid_major {
            errors.push((ErrorType::Other, "Oferta inválida".to_string()));
        }

        // ---------------- PROFESOR ----------------
        let name_parts = class.professor.split_whitespace().count();

        if name_parts < 2 {
            errors.push((ErrorType::Other, "Nombre de profesor sospechoso".to_string()));
        }

        // ---------------- GROUP ----------------
        if class.group.parse::<u8>().is_err() {
            errors.push((ErrorType::Group, "Grupo inválido".to_string()));
        }

        // ---------------- TIME RANGE (básico) ----------------
        if class.start_time.trim().is_empty() || class.end_time.trim().is_empty() {
            errors.push((ErrorType::Other, "Horario incompleto".to_string()));
        }

        errors
    }
}