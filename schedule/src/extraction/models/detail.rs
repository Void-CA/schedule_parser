use regex::Regex;

#[derive(Debug, Clone)]
pub struct RawDetail {
    pub subject: String,
    pub oferta_raw: String,
    pub group: String,
    pub professor_title: Option<String>,
    pub professor_name: String,
    pub room: String,
}

pub struct DetailParser {
    re_group: Regex,
    re_room: Regex,
    re_title: Regex,
}

impl DetailParser {
    pub fn new() -> Self {
        Self {
            re_group: Regex::new(r"Gpo\s*(\d+)").unwrap(),
            re_room: Regex::new(r"[A-Z]\d{3,4}").unwrap(),
            re_title: Regex::new(r"^(Ing\.|MSc\.|Lic\.)").unwrap(),
        }
    }

    pub fn parse(&self, input: &str) -> Result<RawDetail, String> {
        let mut working = input.trim().to_string();

        // 1. EXTRAER AULA (desde el final)
        let room_caps = self.re_room.find_iter(&working).last()
            .ok_or(format!("No se pudo extraer aula de: {}", input))?;

        let room = room_caps.as_str().to_string();
        working = working[..room_caps.start()].trim().to_string();

        // 2. EXTRAER GRUPO
        let group_caps = self.re_group
            .captures(&working)
            .ok_or("No se pudo extraer grupo")?;

        let group = group_caps[1].to_string();

        let before_group = working[..group_caps.get(0).unwrap().start()].trim();
        let before_group = before_group.replace("*", "");
        let after_group = working[group_caps.get(0).unwrap().end()..].trim();

        // 3. PROFESOR
        let mut professor_title = None;
        let mut professor_name = after_group.to_string();

        if let Some(title_caps) = self.re_title.captures(after_group) {
            professor_title = Some(title_caps[1].to_string());
            professor_name = after_group[title_caps.get(0).unwrap().end()..]
                .trim()
                .to_string();
        }

        // 4. SUBJECT + OFERTA
        // Estrategia: detectar carreras (tokens en mayúsculas tipo ICE, IME)
        let mut subject_parts = Vec::new();
        let mut oferta_parts = Vec::new();
        const VALID_MAJORS: &[&str] = &["ICE", "IME", "IMS", "IGI", "IEE", "IEM", "LAF", "LCM"];
        for token in before_group.split_whitespace() {
            let clean = token.trim_matches(|c: char| !c.is_alphanumeric());

            if VALID_MAJORS.contains(&clean) {
                oferta_parts.push(clean);
                continue;
            }

            subject_parts.push(clean);
            
        }

        let subject = subject_parts.join(" ");
        let oferta_raw = oferta_parts.join(", ");

        Ok(RawDetail {
            subject,
            oferta_raw,
            group,
            professor_title,
            professor_name,
            room,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
fn test_detail_parser() {
    let parser = DetailParser::new();

    let input = "Matemáticas ICE, IME Gpo 1 Ing. Juan Pérez A101";

    let result = parser.parse(input).unwrap();

    assert_eq!(result.subject, "Matemáticas");
    assert_eq!(result.oferta_raw, "ICE, IME");
    assert_eq!(result.group, "1");
    assert_eq!(result.professor_title, Some("Ing.".to_string()));
    assert_eq!(result.professor_name, "Juan Pérez");
    assert_eq!(result.room, "A101");
}

#[test]
fn test_detail_parser_no_title() {
    let parser = DetailParser::new();

    let input = "Física IME Gpo 2 María Gómez B202";

    let result = parser.parse(input).unwrap();

    assert_eq!(result.subject, "Física");
    assert_eq!(result.oferta_raw, "IME");
    assert_eq!(result.group, "2");
    assert_eq!(result.professor_title, None);
    assert_eq!(result.professor_name, "María Gómez");
    assert_eq!(result.room, "B202");
}
}