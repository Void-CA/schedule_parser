use pdf_extract::extract_text;
use regex::Regex;

mod models;
use crate::models::{Day, AcademicBlock, Encounter, Major};

fn main() {
    let path = "horario_IV.pdf";
    
    let text = match extract_text(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error leyendo el PDF: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = text.lines().collect();
    
    // El estado actual ahora está fuertemente tipado
    let mut current_major: Option<Major> = None;
    
    let re_encabezado = Regex::new(r"Carrera:\s*([A-Z]+)").unwrap();
    let re_clase = Regex::new(r"^(Lu|Ma|Mi|Ju|Vi|Sa)\s+(\d{2}:\d{2}\s*[ap]m)\s+(\d{2}:\d{2}\s*[ap]m)\s+(.+)$").unwrap();
    let re_detalle = Regex::new(r"^(.*?)\s+\*?\s*[A-Z]+(?:,\s*[A-Z]+)*\s+Gpo\s+(\d+)\s+(Ing\.|MSc\.|Lic\.)\s+(.*?)\s+([A-Z]\d{3,4})$").unwrap();

    println!("Generando horario unificado...\n");

    let mut master_schedule: Vec<Encounter> = Vec::new();

    for line in lines {
        let line = line.trim();
        
        // 1. Detectar y validar cambio de carrera
        if let Some(caps) = re_encabezado.captures(line) {
            let abbrev = &caps[1];
            current_major = match Major::from_abbrev(abbrev) {
                Ok(maj) => Some(maj),
                Err(e) => {
                    eprintln!("⚠️ {}", e);
                    None // Invalidamos el estado para no procesar basura
                }
            };
            continue;
        }

        // 2. Procesar clases solo si estamos dentro de una carrera válida
        if let Some(major) = &current_major {
            if let Some(caps) = re_clase.captures(line) {
                let dia_raw = &caps[1];
                let inicio_raw = &caps[2];
                let fin_raw = &caps[3];
                let resto = &caps[4];
                
                if let Some(det_caps) = re_detalle.captures(resto) {
                    let day = Day::from_abbrev(dia_raw).unwrap();
                    let blocks = AcademicBlock::from_time_range(inicio_raw, fin_raw).unwrap();
                    let group = det_caps[2].parse::<u8>().unwrap_or(0);

                    let encounter = Encounter {
                        major: major.clone(),
                        group,
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

    // Ejemplo de uso: Extraer y mostrar el nombre completo de una carrera
    if let Some(first_encounter) = master_schedule.first() {
        println!("Primera clase escaneada pertenece a: {}\n", first_encounter.major.full_name());
    }

    // Iteramos e imprimimos el parseo global
    for encounter in &master_schedule {
        println!("{:#?}", encounter);
    }
}