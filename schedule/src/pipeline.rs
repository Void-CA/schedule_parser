use crate::{
    extraction::{
        preprocess::normalize_lines::normalize_lines,
        models::{row::RowParser, detail::DetailParser, class::RawClass},
    },
    parser_qa::ParserQA,
    validator::Validator,
    domain::{models::Class, normalizer::Normalizer},
};

pub struct Pipeline;

impl Pipeline {
    pub fn build(pdf_path: &str) -> Vec<Class> {
        let raw_text = pdf_extract::extract_text(pdf_path)
            .expect("Failed to extract PDF");

        let lines = normalize_lines(&raw_text);

        let row_parser = RowParser::new();
        let detail_parser = DetailParser::new();

        let mut qa = ParserQA::new();
        let mut dataset = Vec::new();

        for line in lines {
            qa.inc_total();

            let Ok(raw_row) = row_parser.parse(&line) else {
                qa.log_row_error("Row parse error".to_string(), line.clone());
                continue;
            };

            qa.log_row_success();

            let Ok(detail) = detail_parser.parse(&raw_row.rest) else {
                qa.log_detail_error("Detail parse error".to_string(), line.clone());
                continue;
            };

            qa.log_detail_success();

            let raw_class = RawClass::from_parts(raw_row, detail);

            let errors = Validator::validate(&raw_class);
            qa.handle_validation(errors.clone(), line.clone());

            if !errors.is_empty() {
                continue;
            }

            if let Some(class) = Normalizer::normalize(raw_class) {
                dataset.push(class);
            }
        }

        qa.report();

        dataset
    }
}