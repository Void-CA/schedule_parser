use std::fs;
use std::path::Path;
use serde_json::{json, Value};
use std::io::{self, Write};

/// Parsea un archivo raw_*.txt y genera el JSON con nombres oficiales
fn parse_raw_file(filepath: &str) -> Vec<String> {
    let content = fs::read_to_string(filepath)
        .expect("Error al leer el archivo raw");
    
    let lines: Vec<&str> = content.lines().collect();
    let mut official_names = Vec::new();
    let mut current_name = String::new();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !current_name.is_empty() {
                official_names.push(current_name.clone());
                current_name.clear();
            }
            continue;
        }
        
        // Si empieza con •, es un nuevo curso
        if trimmed.starts_with('•') {
            if !current_name.is_empty() {
                official_names.push(current_name.clone());
            }
            current_name = trimmed[1..].trim().to_string();
        } else {
            // Es continuación del nombre anterior
            if !current_name.is_empty() {
                current_name.push(' ');
                current_name.push_str(trimmed);
            } else {
                current_name = trimmed.to_string();
            }
        }
    }
    
    // No olvidar el último
    if !current_name.is_empty() {
        official_names.push(current_name);
    }
    
    // Filtrar basura (como "Cuatrimestre I Cuatrimestre II...")
    official_names.retain(|name| {
        !name.starts_with("Cuatrimestre") && 
        !name.starts_with("I AÑO") &&
        !name.starts_with("II AÑO") &&
        !name.starts_with("III AÑO") &&
        !name.starts_with("IV AÑO") &&
        !name.starts_with("V AÑO") &&
        !name.contains("Modalidad") &&
        !name.contains("Prácticas") &&
        name.len() > 5
    });
    
    official_names
}

/// Extrae códigos y nombres sucios del OCR
fn parse_ocr_file(filepath: &str) -> Vec<(String, String)> {
    let content = fs::read_to_string(filepath)
        .expect("Error al leer el archivo OCR");
    
    let lines: Vec<&str> = content.lines().collect();
    let mut results = Vec::new();
    let mut buffer = String::new();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Si empieza con 4 dígitos, es una nueva asignatura
        if trimmed.len() >= 4 && trimmed[..4].chars().all(|c| c.is_digit(10)) {
            if !buffer.is_empty() {
                // Procesar el buffer anterior
                if let Some((code, name)) = extract_code_and_name(&buffer) {
                    results.push((code, name));
                }
                buffer.clear();
            }
            buffer = trimmed.to_string();
        } else {
            // Es continuación
            if !buffer.is_empty() {
                buffer.push(' ');
                buffer.push_str(trimmed);
            }
        }
    }
    
    // No olvidar el último
    if !buffer.is_empty() {
        if let Some((code, name)) = extract_code_and_name(&buffer) {
            results.push((code, name));
        }
    }
    
    results
}

fn extract_code_and_name(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    
    let code = parts[0].to_string();
    if code.len() != 4 || !code.chars().all(|c| c.is_digit(10)) {
        return None;
    }
    
    // El nombre es todo hasta encontrar un número (las horas)
    let mut name_parts = Vec::new();
    for part in &parts[1..] {
        if part.chars().all(|c| c.is_digit(10)) {
            break;
        }
        // Limpiar basura OCR y guardar como String owned
        let cleaned = part.replace(['[', ']', '|', '—', ':', '!', '¡'], "");
        if !cleaned.is_empty() {
            name_parts.push(cleaned);
        }
    }
    
    if name_parts.is_empty() {
        None
    } else {
        Some((code, name_parts.join(" ")))
    }
}
    

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Uso: {} <MAJOR>", args[0]);
        eprintln!("Ejemplo: {} ICE", args[0]);
        std::process::exit(1);
    }
    
    let major = &args[1];
    let year = "2026";
    
    // Archivos
    let raw_file = format!("data/courses/raw_{}.txt", major.to_lowercase());
    let ocr_file = format!("data/processed/plan_{}_{}.txt", major, year);
    let output_file = format!("data/courses/{}_{}.json", major.to_lowercase(), year);
    
    // Verificar que existan
    if !Path::new(&raw_file).exists() {
        eprintln!("❌ No se encontró: {}", raw_file);
        std::process::exit(1);
    }
    
    println!("📖 Procesando: {}", raw_file);
    let official_names = parse_raw_file(&raw_file);
    println!("   ✅ {} nombres oficiales extraídos", official_names.len());
    
    // Si existe el OCR, extraer códigos
    let mut ocr_data = Vec::new();
    if Path::new(&ocr_file).exists() {
        println!("📄 Procesando OCR: {}", ocr_file);
        ocr_data = parse_ocr_file(&ocr_file);
        println!("   ✅ {} entradas del OCR", ocr_data.len());
    }
    
    // Generar JSON
    let mut courses = serde_json::Map::new();
    
    if ocr_data.is_empty() {
        // Solo nombres oficiales (sin códigos)
        for (i, name) in official_names.iter().enumerate() {
            courses.insert(
                format!("course_{:03}", i),
                json!(name)
            );
        }
    } else {
        // Matchear OCR con nombres oficiales (búsqueda simple por similitud)
        for (code, dirty_name) in &ocr_data {
            // Búsqueda simple: ver si el nombre suicio contiene palabras del oficial
            let dirty_lower = dirty_name.to_lowercase();
            let mut best_match = dirty_name.clone();
            
            for official in &official_names {
                let official_lower = official.to_lowercase();
                // Verificar si alguna palabra clave coincide
                let dirty_words: Vec<&str> = dirty_lower.split_whitespace().collect();
                let official_words: Vec<&str> = official_lower.split_whitespace().collect();
                
                let mut matches = 0;
                for dw in &dirty_words {
                    if official_words.contains(dw) {
                        matches += 1;
                    }
                }
                
                // Si más del 50% de las palabras coinciden
                if matches > 0 && matches >= dirty_words.len() / 2 {
                    best_match = official.clone();
                    break;
                }
            }
            
            courses.insert(code.clone(), json!(best_match));
            println!("   {}: '{}' → '{}'", code, dirty_name, best_match);
        }
    }
    
    let output = json!({
        "major": major,
        "year": year,
        "courses": courses
    });
    
    let json_string = serde_json::to_string_pretty(&output)
        .expect("Error al serializar JSON");
    
    fs::write(&output_file, json_string)
        .expect("Error al escribir el archivo JSON");
    
    println!("\n✅ JSON generado: {}", output_file);
    println!("   Total de asignaturas: {}", courses.len());
}
