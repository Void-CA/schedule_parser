use regex::Regex;
use wasm_bindgen::prelude::*;

mod models;
use crate::models::{Day, AcademicBlock, Encounter, Major};

// Esta macro es el puente entre Rust y JS
#[wasm_bindgen]
pub fn parse_schedule(raw_text: &str) -> Result<JsValue, JsValue> {
    let lines: Vec<&str> = raw_text.lines().collect();
    let mut current_major: Option<Major> = None;
    let mut master_schedule: Vec<Encounter> = Vec::new();

    let re_encabezado = Regex::new(r"Carrera:\s*([A-Z]+)").unwrap();
    let re_clase = Regex::new(r"^(Lu|Ma|Mi|Ju|Vi|Sa)\s+(\d{2}:\d{2}\s*[ap]m)\s+(\d{2}:\d{2}\s*[ap]m)\s+(.+)$").unwrap();
    let re_detalle = Regex::new(r"^(.*?)\s+\*?\s*[A-Z]+(?:,\s*[A-Z]+)*\s+Gpo\s+(\d+)\s+(Ing\.|MSc\.|Lic\.)\s+(.*?)\s+([A-Z]\d{3,4})$").unwrap();

    for line in lines {
        let line = line.trim();
        
        if let Some(caps) = re_encabezado.captures(line) {
            let abbrev = &caps[1];
            current_major = Major::from_abbrev(abbrev).ok();
            continue;
        }

        if let Some(major) = &current_major {
            if let Some(caps) = re_clase.captures(line) {
                let dia_raw = &caps[1];
                let inicio_raw = &caps[2];
                let fin_raw = &caps[3];
                let resto = &caps[4];
                
                if let Some(det_caps) = re_detalle.captures(resto) {
                    // Ignoramos silenciosamente si la hora o día son inválidos en la vista web
                    if let (Ok(day), Ok(blocks)) = (Day::from_abbrev(dia_raw), AcademicBlock::from_time_range(inicio_raw, fin_raw)) {
                        let encounter = Encounter {
                            major: major.clone(),
                            group: det_caps[2].parse::<u8>().unwrap_or(0),
                            subject: det_caps[1].trim().to_string(),
                            professor: format!("{} {}", &det_caps[3], det_caps[4].trim()), 
                            room: det_caps[5].to_string(),
                            day,
                            blocks,
                        };
                        master_schedule.push(encounter);
                    }
                }
            }
        }
    }

    // Convertimos nuestro Vec<Encounter> de Rust a un Array de Objetos en JS
    serde_wasm_bindgen::to_value(&master_schedule)
        .map_err(|err| JsValue::from_str(&err.to_string()))
}