use regex::Regex;

#[derive(Debug)]
pub struct RawRow {
    pub major: String,
    pub year: String,
    pub quarter: String,
    pub course_code: String,
    pub course_name: String,
    pub teorical_hours: String,
    pub practical_hours: String,
    pub teorical_practical_hours: String,
    pub total_hours: String,
    pub total_credits: String,
    pub prerequisites: String,
    pub precedents: String,
}


pub struct RowParser {
    re: Regex,
}

impl RowParser {
    pub fn new() -> Self {
        // Ajustamos para que el nombre capture TODO hasta que encuentre 
        // la primera secuencia clara de 5 bloques numéricos (las horas).
        let pattern = r"(?x)
            ^\s*(?P<code>\d{4})                 # Código
            \s+                                  # Espacio
            (?P<name>.+?)                        # Nombre (ahora más flexible)
            \s+(?P<ht>\d+)\s+                    # HT (Ancla fuerte)
            (?P<hp>\d+)\s+                       # HP
            (?P<hti>\d+)\s+                      # HTI
            (?P<th>\d+)\s+                       # TH
            (?P<tc>\d{1,2})                      # TC
            \s+(?P<prereq>.+?)                   # Prerrequisitos
            \s+(?P<preced>.+)$                   # Precedentes
        ";

        Self {
            re: Regex::new(pattern).expect("Error en Regex"),
        }
    }

    pub fn parse(&self, line: &str, major: &str, year: &str, quarter: &str) -> Option<RawRow> {
        let caps = self.re.captures(line)?;

        // Sanitización inmediata de campos de texto
        let name = self.sanitize_field(&caps["name"]);
        let prereq = self.sanitize_field(&caps["prereq"]);
        let preced = self.sanitize_field(&caps["preced"]);

        // Corrección de drift numérico (ej. 1385 -> 135)
        // Si el TH es mayor a 300, probablemente el OCR pegó el TC al final.
        let mut total_h = caps["th"].to_string();
        if total_h.len() > 3 {
            total_h.truncate(3); 
        }

        Some(RawRow {
            major: major.to_string(),
            year: year.to_string(),
            quarter: quarter.to_string(),
            course_code: caps["code"].to_string(),
            course_name: name,
            teorical_hours: caps["ht"].to_string(),
            practical_hours: caps["hp"].to_string(),
            teorical_practical_hours: caps["hti"].to_string(),
            total_hours: total_h,
            total_credits: caps["tc"].to_string(),
            prerequisites: prereq,
            precedents: preced,
        })
    }

    fn sanitize_field(&self, text: &str) -> String {
        let clean = text.replace(['|', '[', ']', '—', '(', ')', ':', '!', '¡'], "")
            .trim()
            .to_string();
            
        if clean.is_empty() || clean.to_lowercase().contains("ningun") {
            "Ninguno".to_string()
        } else {
            clean
        }
    }

    fn preprocess_ocr_text(raw_text: &str) -> Vec<String> {
    let mut combined_lines = Vec::new();
    let mut current_buffer = String::new();

    for line in raw_text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

        // Si la línea empieza con 4 dígitos (ej: 0401), es una nueva asignatura
        if trimmed.len() >= 4 && trimmed[..4].chars().all(|c| c.is_digit(10)) {
            if !current_buffer.is_empty() {
                combined_lines.push(current_buffer);
            }
            current_buffer = trimmed.to_string();
        } else {
            // Es una continuación (ej: "Proyectos" o "de Comunicación")
            current_buffer.push(' ');
            current_buffer.push_str(trimmed);
        }
    }
    
    if !current_buffer.is_empty() {
        combined_lines.push(current_buffer);
    }
    
    combined_lines
}

}

impl RawRow {
    pub fn self_heal(&mut self) {
        let ht = self.teorical_hours.parse::<u32>().unwrap_or(0);
        let hp = self.practical_hours.parse::<u32>().unwrap_or(0);
        let hti = self.teorical_practical_hours.parse::<u32>().unwrap_or(0);
        let th = self.total_hours.parse::<u32>().unwrap_or(0);

        // Si el total leído es correcto pero la suma de las partes falla
        if ht + hp + hti != th {
            // Caso específico: OCR leyó 15 en vez de 75 (Faltan 60 horas)
            if th == (ht + hp + hti + 60) {
                // Asumimos que el error está en HTI por ser el campo más variable
                self.teorical_practical_hours = (hti + 60).to_string();
            }
        }
    }
}