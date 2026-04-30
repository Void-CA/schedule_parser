use std::fs;
mod raw;
mod domain;

// Importamos lo necesario de tus módulos
use raw::row::{RowParser};
use domain::auditor::{PlanAuditor};
use domain::{Normalizer, Class, NameCorrector};

fn main() {
    let major = "ICE";
    let year = "2026";
    
    // 1. Carga del archivo procesado por Tesseract
    let content = fs::read_to_string(format!("data/processed/plan_{}_2026.txt", major))
        .expect("Error al leer el archivo de texto");

    let parser = RowParser::new();
    
    // 2. Pre-procesamiento: Unión de líneas fragmentadas
    let sanitized_lines = preprocess_ocr_lines(&content);

    // 3. Inicializar el corrector de nombres desde archivo JSON
    let corrector = NameCorrector::from_file(major, year)
        .expect("Error al cargar el archivo de nombres de asignaturas");

    // 4. Extracción, Corrección y Normalización (Pipeline)
    let final_dataset: Vec<Class> = sanitized_lines
        .iter()
        .filter_map(|line| {
            // Primero el RowParser extrae los grupos del Regex
            let mut raw_row = parser.parse(line, major, year, "1")?;
            
            // Validar que el código sea válido (exista en el JSON de la carrera)
            if !corrector.is_valid_code(&raw_row.course_code) {
                return None; // Descartar líneas con códigos inválidos
            }
            
            // Corregir el nombre usando el archivo JSON y patrones OCR
            let corrected_name = corrector.correct(&raw_row.course_code, &raw_row.course_name);
            raw_row.course_name = corrected_name;
            
            // El Normalizer aplica el self_heal y convierte tipos
            Normalizer::normalize(raw_row)
        })
        .collect();

    // 5. Auditoría de los datos finales
    // El auditor ahora recibe el dataset ya corregido para evaluar la calidad real
    let report = PlanAuditor::run_final(&final_dataset);

    // 6. Resultados
    report.print_summary();

    // Opcional: Imprimir el dataset final para inspección visual
    println!("Dataset Final: {:#?}", final_dataset);
}

fn preprocess_ocr_lines(text: &str) -> Vec<String> {
    let mut merged = Vec::new();
    let mut buffer = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

        // Si la línea empieza con 4 dígitos, es una nueva asignatura
        if trimmed.len() >= 4 && trimmed[..4].chars().all(|c| c.is_digit(10)) {
            if !buffer.is_empty() {
                merged.push(buffer.clone());
                buffer.clear();
            }
            buffer = trimmed.to_string();
        } else {
            // Es una continuación del nombre
            if !buffer.is_empty() {
                buffer.push(' ');
                buffer.push_str(trimmed);
            } else {
                // Texto huérfano antes del código (raro, pero pasa)
                buffer = trimmed.to_string();
            }
        }
    }
    
    if !buffer.is_empty() { merged.push(buffer); }
    merged
}