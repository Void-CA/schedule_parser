use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

/// Estructura para el nuevo formato JSON (array de nombres oficiales)
#[derive(Debug, Deserialize, Serialize)]
struct CourseData {
    major: String,
    year: String,
    #[serde(default)]
    courses: HashMap<String, String>,
    #[serde(default)]
    official_names: Vec<String>,
}

/// Capa de corrección simple: JSON + OCR fixes
pub struct NameCorrector {
    /// Diccionario: código OCR -> nombre oficial
    known_courses: HashMap<String, String>,
    /// Lista de nombres para búsqueda rápida
    ocr_fixes: Vec<(String, String)>,
}

impl NameCorrector {
    pub fn from_file(major: &str, year: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = format!("data/courses/{}_{}.json", major.to_lowercase(), year);
        let content = fs::read_to_string(&file_path)?;
        let course_data: CourseData = serde_json::from_str(&content)?;
        
        let mut ocr_fixes = vec![
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
            known_courses: course_data.courses,
            ocr_fixes,
        })
    }

    /// Corrige aplicando solo OCR fixes (el JSON ya tiene los nombres correctos)
    pub fn correct(&self, code: &str, raw_name: &str) -> String {
        // Si el código está en el JSON, usá ese nombre
        if let Some(official) = self.known_courses.get(code) {
            return official.clone();
        }

        // Sino, aplicar OCR fixes al nombre sucio
        let mut result = raw_name.to_string();
        for (wrong, right) in &self.ocr_fixes {
            result = result.replace(wrong, right);
        }
        result.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    /// Verifica si el código existe en el JSON
    pub fn is_valid_code(&self, code: &str) -> bool {
        self.known_courses.contains_key(code)
    }
}
