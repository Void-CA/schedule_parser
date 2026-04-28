use regex::Regex;

#[derive(Debug, Clone)]
pub struct RawRow {
    pub day: String,
    pub start_time: String,
    pub end_time: String,
    pub rest: String, 
}

pub struct RowParser {
    re: Regex,
}

impl RowParser {
    pub fn new() -> Self {
        Self {
            re: Regex::new(
                r"^(Lu|Ma|Mi|Ju|Vi|Sa)\s+(\d{2}:\d{2}\s*[aApP][mM])\s+(\d{2}:\d{2}\s*[aApP][mM])\s+(.*)$"
            ).unwrap()
        }
    }

    pub fn parse(&self, line: &str) -> Result<RawRow, String> {
        if let Some(caps) = self.re.captures(line) {
            Ok(RawRow {
                day: caps[1].to_string(),
                start_time: caps[2].to_string(),
                end_time: caps[3].to_string(),
                rest: caps[4].to_string(),
            })
        } else {
            Err(format!("No match for line: {}", line))
        }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_parser() {
        let parser = RowParser::new();
        let line = "Lu 08:00 am 08:50 am Matemáticas ICE Gpo 1 Ing. Juan Pérez A101";
        let result = parser.parse(line).unwrap();
        assert_eq!(result.day, "Lu");
        assert_eq!(result.start_time, "08:00 am");
        assert_eq!(result.end_time, "08:50 am");
        assert_eq!(result.rest, "Matemáticas ICE Gpo 1 Ing. Juan Pérez A101");
    }

    #[test]
    fn test_row_parser_variations() {
        let parser = RowParser::new();

        let cases = vec![
            "Lu   08:00 am   08:50 am   Matemáticas ICE Gpo 1 Ing. Juan Pérez A101",
            "Lu 08:00am 08:50am Matemáticas ICE Gpo 1 Ing. Juan Pérez A101",
            "Lu\t08:00 am\t08:50 am\tMatemáticas ICE Gpo 1 Ing. Juan Pérez A101",
        ];

        for case in cases {
            assert!(parser.parse(case).is_ok());
        }
    }
}