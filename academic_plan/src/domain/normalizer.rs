use crate::raw::row::RawRow;
use super::Class;

pub struct Normalizer;

impl Normalizer {
    pub fn normalize(mut raw: RawRow) -> Option<Class> {
        // --- Paso 1: Curación Aritmética (Self Heal) ---
        // Corrige errores sistemáticos del OCR como confundir 75 con 15
        Self::apply_arithmetic_fixes(&mut raw);

        // --- Paso 2: Limpieza de Strings (Truncados y Ruido) ---
        let clean_name = Self::sanitize_text(&raw.course_name);
        let clean_prereq = Self::sanitize_text(&raw.prerequisites);
        let clean_preced = Self::sanitize_text(&raw.precedents);

        // --- Paso 3: Conversión de tipos con validación básica ---
        Some(Class {
            code: raw.course_code,
            name: clean_name,
            theoretical_hours: raw.teorical_hours.parse().unwrap_or(0),
            practical_hours: raw.practical_hours.parse().unwrap_or(0),
            independent_hours: raw.teorical_practical_hours.parse().unwrap_or(0),
            total_hours: raw.total_hours.parse().unwrap_or(0),
            credits: raw.total_credits.parse().unwrap_or(0),
            prerequisites: clean_prereq,
            precedents: clean_preced,
        })
    }

    fn apply_arithmetic_fixes(raw: &mut RawRow) {
        let ht = raw.teorical_hours.parse::<u32>().unwrap_or(0);
        let hp = raw.practical_hours.parse::<u32>().unwrap_or(0);
        let hti = raw.teorical_practical_hours.parse::<u32>().unwrap_or(0);
        let th = raw.total_hours.parse::<u32>().unwrap_or(0);
        let tc = raw.total_credits.parse::<u32>().unwrap_or(0);

        // 1. Regla de Oro: El total debe ser múltiplo de 45 (Invariante de la ULSA)
        let expected_th_by_credits = tc * 45;
        let sum_parts = ht + hp + hti;

        // 2. Lógica de Consenso para TH
        // Si el TH leído no es múltiplo de 45, pero el cálculo por créditos sí lo es, 
        // o si la suma de las partes coincide con el cálculo por créditos...
        if th != expected_th_by_credits {
            if sum_parts == expected_th_by_credits {
                // Caso: El OCR leyó mal el TH (como el 138 vs 135), pero las partes están bien
                raw.total_hours = expected_th_by_credits.to_string();
            } else if expected_th_by_credits > 0 {
                // Caso: El OCR leyó mal el TH y las partes, pero confiamos en los Créditos
                raw.total_hours = expected_th_by_credits.to_string();
            }
        }

        // 3. Reconstrucción de la variable faltante (HTI suele ser la más ruidosa)
        // Ahora que tenemos un TH confiable (multiplo de 45), reparamos la sumatoria
        let final_th = raw.total_hours.parse::<u32>().unwrap_or(0);
        if (ht + hp + hti) != final_th && final_th > 0 {
            // Asumimos que HT y HP son más estables por ser números pequeños, 
            // y recalculamos HTI (Horas de Trabajo Independiente)
            let corrected_hti = final_th.saturating_sub(ht + hp);
            raw.teorical_practical_hours = corrected_hti.to_string();
        }
    }

    /// Limpia caracteres residuales del OCR y normaliza el texto
    fn sanitize_text(text: &str) -> String {
        let cleaned = text
            .replace(['|', '[', ']', '—', ':', '!', '¡'], "") // Elimina ruido de tablas
            .trim()
            .to_string();

        if cleaned.is_empty() || cleaned.to_lowercase().contains("ningun") {
            "Ninguno".to_string()
        } else {
            cleaned
        }
    }

    fn sanitize_precedent(text: &str) -> String {
    let mut cleaned = text
        .replace(['|', '[', ']', '—', ':', '!', '¡'], "")
        .trim()
        .to_string();

    // Si el OCR absorbió filas de control del PDF, las cortamos
    let stop_words = ["Subtotal", "Totales", "PFP", "Modalidad"];
    for word in stop_words {
        if let Some(pos) = cleaned.find(word) {
            cleaned.truncate(pos);
        }
    }

    let final_text = cleaned.trim().to_string();
    if final_text.is_empty() || final_text.to_lowercase().contains("ningun") {
        "Ninguno".to_string()
    } else {
        final_text
    }
}
}