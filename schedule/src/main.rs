mod extraction;
mod parser_qa;
mod error;
mod validator;
mod domain;

use crate::extraction::preprocess::normalize_lines::normalize_lines;
use crate::extraction::models::row::RowParser;
use crate::extraction::models::detail::DetailParser;
use crate::extraction::models::class::RawClass;

use crate::parser_qa::ParserQA;
use crate::validator::Validator;

fn main() {
    let pdf_path = "horarios_IV.pdf";

    let raw_text = pdf_extract::extract_text(pdf_path)
        .expect("Failed to extract text from PDF");

    let lines = normalize_lines(&raw_text);

    let row_parser = RowParser::new();
    let detail_parser = DetailParser::new();

    let mut qa = ParserQA::new();

    // dataset futuro (aún crudo)
    let mut raw_dataset: Vec<RawClass> = Vec::new();

    for line in lines {
        qa.inc_total(); // mejor que tocar metrics directo

        match row_parser.parse(&line) {
            Ok(raw_row) => {
                qa.log_row_success();

                match detail_parser.parse(&raw_row.rest) {
                    Ok(detail) => {
                        qa.log_detail_success();

                        // 🔹 MERGE
                        let raw_class = RawClass::from_parts(raw_row, detail);

                        // 🔹 VALIDACIÓN (ahora sobre RawClass)
                        let validation_errors = Validator::validate(&raw_class);
                        qa.handle_validation(validation_errors.clone(), line.clone());

                        // 🔹 SOLO GUARDAS SI ES VÁLIDO
                        if validation_errors.is_empty() {
                            raw_dataset.push(raw_class);
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

    // ---------------- REPORTE ----------------
    qa.report();

    // 🔹 DEBUG OPCIONAL
    println!("Clases válidas: {}", raw_dataset.len());
    println!("Ejemplo de clase válida: {:#?}", raw_dataset.first());
}