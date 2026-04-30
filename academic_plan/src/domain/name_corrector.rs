use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use strsim::{levenshtein, normalized_damerau_levenshtein};

/// Estructura para deserializar el archivo JSON de una carrera
#[derive(Debug, Deserialize, Serialize)]
struct CourseData {
    major: String,
    year: String,
    courses: HashMap<String, String>,
}

/// Capa de corrección de nombres usando:
/// 1. Archivo JSON (nombres oficiales por código)
/// 2. Fuzzy matching (búsqueda por similitud si el código falla)
/// 3. Patrones OCR sistemáticos
pub struct NameCorrector {
    /// Diccionario cargado: código -> nombre oficial
    known_courses: HashMap<String, String>,
    /// Lista plana de nombres para fuzzy search
    course_names: Vec<String>,
    /// Patrones OCR comunes
    ocr_fixes: Vec<(String, String)>,
}

impl NameCorrector {
    /// Carga los nombres desde el archivo JSON
    pub fn from_file(major: &str, year: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = format!("data/courses/{}_{}.json", major.to_lowercase(), year);
        let content = fs::read_to_string(&file_path)?;
        let course_data: CourseData = serde_json::from_str(&content)?;
        
        let known_courses = course_data.courses.clone();
        let course_names: Vec<String> = known_courses.values().cloned().collect();
        
        let ocr_fixes = vec![
            (" ll ".to_string(), " II ".to_string()),
            ("ll ".to_string(), "II ".to_string()),
            (" ll".to_string(), " II".to_string()),
            ("l1".to_string(), "II".to_string()),
            ("1l".to_string(), "II".to_string()),
            ("1I".to_string(), "II".to_string()),
            ("Il".to_string(), "II".to_string()),
            ("lI".to_string(), "II".to_string()),
            ("Ill".to_string(), "III".to_string()),
            ("1ll".to_string(), "III".to_string()),
            ("Histonia".to_string(), "Historia".to_string()),
            ("Naconal".to_string(), "Nacional".to_string()),
            ("Redaccion".to_string(), "Redacción".to_string()),
            ("Tecnico".to_string(), "Técnico".to_string()),
            ("Asistido]".to_string(), "Asistido".to_string()),
            ("Programacion".to_string(), "Programación".to_string()),
            ("Comunicacion".to_string(), "Comunicación".to_string()),
            ("[".to_string(), "".to_string()),
            ("]".to_string(), "".to_string()),
            ("|".to_string(), "".to_string()),
        ];

        Ok(Self {
            known_courses,
            course_names,
            ocr_fixes,
        })
    }

    /// Corrige el nombre usando múltiples estrategias
    pub fn correct(&self, code: &str, raw_name: &str) -> String {
        // Paso 1: Aplicar correcciones OCR básicas
        let cleaned_name = self.apply_ocr_fixes(raw_name);

        // Paso 2: Fuzzy matching (búsqueda por similitud)
        // IGNORAMOS el código porque cambia entre planes
        if let Some(best_match) = self.find_best_match(&cleaned_name) {
            return best_match;
        }

        // Paso 3: Si nada funciona, devolver el nombre limpio del OCR
        cleaned_name
    }

    /// Aplica correcciones de patrones OCR sistemáticos
    fn apply_ocr_fixes(&self, name: &str) -> String {
        let mut result = format!(" {} ", name);
        for (wrong, right) in &self.ocr_fixes {
            result = result.replace(wrong, right);
        }
        result.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    /// Encuentra el nombre más similar usando distancia de Damerau-Levenshtein
    fn find_best_match(&self, name: &str) -> Option<String> {
        let name_lower = name.to_lowercase();
        let mut best_score = 0.0;
        let mut best_match = None;

        for official_name in &self.course_names {
            let official_lower = official_name.to_lowercase();
            let score = normalized_damerau_levenshtein(&name_lower, &official_lower);
            
            if score > best_score && score > 0.75 { // Umbral de similitud 75%
                best_score = score;
                best_match = Some(official_name.clone());
            }
        }

        best_match
    }

    /// Verifica si un código existe en el JSON (para filtrar basura)
    pub fn is_valid_code(&self, code: &str) -> bool {
        self.known_courses.contains_key(code)
    }
}
