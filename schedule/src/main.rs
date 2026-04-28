mod extraction;
mod parser_qa;
mod error;
mod validator;
mod domain;
mod analytics;

use crate::analytics::model::ScheduleAnalytics;
use crate::extraction::preprocess::normalize_lines::normalize_lines;
use crate::extraction::models::row::RowParser;
use crate::extraction::models::detail::DetailParser;
use crate::extraction::models::class::RawClass;

use crate::parser_qa::ParserQA;
use crate::validator::Validator;

use crate::domain::normalizer::Normalizer;
use crate::domain::models::Class;

fn main() {
    let pdf_path = "horarios_IV.pdf";

    let raw_text = pdf_extract::extract_text(pdf_path)
        .expect("Failed to extract text from PDF");

    let lines = normalize_lines(&raw_text);

    let row_parser = RowParser::new();
    let detail_parser = DetailParser::new();

    let mut qa = ParserQA::new();

    // 🔹 dataset FINAL (ya dominio)
    let mut dataset: Vec<Class> = Vec::new();

    for line in lines {
        qa.inc_total();

        match row_parser.parse(&line) {
            Ok(raw_row) => {
                qa.log_row_success();

                match detail_parser.parse(&raw_row.rest) {
                    Ok(detail) => {
                        qa.log_detail_success();

                        // -----------------------------
                        // 1. RAW MERGE
                        // -----------------------------
                        let raw_class = RawClass::from_parts(raw_row, detail);

                        // -----------------------------
                        // 2. VALIDACIÓN
                        // -----------------------------
                        let validation_errors = Validator::validate(&raw_class);
                        qa.handle_validation(validation_errors.clone(), line.clone());

                        // -----------------------------
                        // 3. NORMALIZACIÓN → DOMAIN
                        // -----------------------------
                        if validation_errors.is_empty() {
                            if let Some(class) = Normalizer::normalize(raw_class) {
                                dataset.push(class);
                            }
                        }
                    }
                    Err(e) => {
                        qa.log_detail_error(e, line.clone());
                    }
                }
            }
            Err(e) => {
                qa.log_row_error(e, line.clone());
            }
        }
    }

    let analytics = ScheduleAnalytics::new(dataset);

    analytics.summary();

    let conflicts = analytics.detect_conflicts();
    println!("Conflictos: {}", conflicts.len());
}