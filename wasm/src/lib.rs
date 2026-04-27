use regex::Regex;
use wasm_bindgen::prelude::*;

mod models;
use crate::models::{Day, AcademicBlock, Encounter, Major};

#[wasm_bindgen]
pub fn parse_schedule(raw_text: &str) -> Result<JsValue, JsValue> {
    let lines: Vec<&str> = raw_text.lines().collect();
    let mut current_section_major: Option<Major> = None;
    let mut master_schedule: Vec<Encounter> = Vec::new();

    // Regex actualizadas
    let re_encabezado = Regex::new(r"Carrera:\s*([A-Z]+)").unwrap();
    let re_clase = Regex::new(r"^(Lu|Ma|Mi|Ju|Vi|Sa)\s+(\d{2}:\d{2}\s*[ap]m)\s+(\d{2}:\d{2}\s*[ap]m)\s+(.+)$").unwrap();
    
    // Capturamos: 1.Asignatura, 2.Oferta(Carreras), 3.Grupo, 4.Título, 5.Docente, 6.Local
    let re_detalle = Regex::new(r"^(.*?)\s+\*?\s*([A-Z]+(?:,\s*[A-Z]+)*)\s+Gpo\s+(\d+)\s+(Ing\.|MSc\.|Lic\.)\s+(.*?)\s+([A-Z]\d{3,4})$").unwrap();

    for line in lines {
        let line = line.trim();
        
        // Detectar sección del PDF
        if let Some(caps) = re_encabezado.captures(line) {
            current_section_major = Major::from_abbrev(&caps[1]).ok();
            continue;
        }

        // Si tenemos una sección activa y la línea parece una clase
        if let (Some(section_major), Some(caps)) = (&current_section_major, re_clase.captures(line)) {
            let dia_raw = &caps[1];
            let inicio_raw = &caps[2];
            let fin_raw = &caps[3];
            let resto = &caps[4];
            
            if let Some(det_caps) = re_detalle.captures(resto) {
                if let (Ok(day), Ok(blocks)) = (Day::from_abbrev(dia_raw), AcademicBlock::from_time_range(inicio_raw, fin_raw)) {
                    
                    // --- LÓGICA DE OFERTA MULTIPLE ---
                    let mut majors_offered = Vec::new();
                    
                    // 1. Siempre incluimos la carrera de la sección donde aparece
                    majors_offered.push(section_major.clone());
                    
                    // 2. Parseamos la columna de Oferta para encontrar otras carreras
                    let oferta_text = &det_caps[2];
                    let majors_abbrevs = ["ICE", "IMS", "IME", "IGI", "IEE", "IEM", "LAF", "LCM"];
                    
                    for abbrev in majors_abbrevs {
                        if oferta_text.contains(abbrev) {
                            if let Ok(m) = Major::from_abbrev(abbrev) {
                                if !majors_offered.contains(&m) {
                                    majors_offered.push(m);
                                }
                            }
                        }
                    }

                    master_schedule.push(Encounter::new(
                        majors_offered,
                        det_caps[1].trim().to_string(),
                        det_caps[5].trim().to_string(),
                        det_caps[6].trim().to_string(),
                        day,
                        blocks,
                        det_caps[3].parse::<u8>().unwrap_or(0), // Grupo
                    ));
                }
            }
        }
    }

    serde_wasm_bindgen::to_value(&master_schedule)
        .map_err(|err| JsValue::from_str(&err.to_string()))
}