use regex::Regex;

pub fn normalize_lines(raw_text: &str) -> Vec<String> {
    let re_row_start = Regex::new(
        r"^(Lu|Ma|Mi|Ju|Vi|Sa)\s+\d{2}:\d{2}\s*[aApP][mM]"
    ).unwrap();

    let mut result = Vec::new();
    let mut buffer = String::new();

    for raw_line in raw_text.lines() {
        // 1. limpieza básica
        let mut line = raw_line.trim().replace('\t', " ");

        // colapsar múltiples espacios
        line = line.split_whitespace().collect::<Vec<_>>().join(" ");

        if line.is_empty() {
            continue;
        }

        // 2. ignorar líneas que claramente no son datos
        if line.starts_with("Carrera:") {
            continue;
        }

        // 3. detectar inicio de nueva fila
        if re_row_start.is_match(&line) {
            // guardar buffer anterior si existe
            if !buffer.is_empty() {
                result.push(buffer.clone());
            }
            buffer = line;
        } else {
            // 4. línea continuación → unir
            if !buffer.is_empty() {
                buffer.push(' ');
                buffer.push_str(&line);
            }
        }
    }

    // 5. último buffer
    if !buffer.is_empty() {
        result.push(buffer);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_lines() {
        let raw_text = "Lu 09:00 AM 10:00 AM Clase de matemáticas\nMa 10:00 AM 11:00 AM Clase de física";
        let expected = vec![
            "Lu 09:00 AM 10:00 AM Clase de matemáticas",
            "Ma 10:00 AM 11:00 AM Clase de física"
        ];
        assert_eq!(normalize_lines(raw_text), expected);
    }

    #[test]
    fn test_normalize_lines_multiline() {
        let input = "\
    Lu 08:00 am 09:40 am Matemática Aplicada
    ICE, IME Gpo 1 Ing. Juan Pérez A101
    ";

        let lines = normalize_lines(input);

        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("Matemática Aplicada ICE, IME"));
    }
}