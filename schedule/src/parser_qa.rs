use crate::error::{ErrorType, ParseError};

#[derive(Debug)]
pub struct ParseErrorLog {
    pub row_errors: Vec<ParseError>,
    pub detail_errors: Vec<ParseError>,
}

#[derive(Debug, Default)]
pub struct ParseMetrics {
    pub total_lines: usize,
    pub row_success: usize,
    pub row_fail: usize,
    pub detail_success: usize,
    pub detail_fail: usize,
    pub valid_success: usize,
    pub valid_fail: usize,
}

pub struct ParserQA {
    pub metrics: ParseMetrics,
    pub errors: ParseErrorLog,
}

impl ParserQA {
    pub fn new() -> Self {
        Self {
            metrics: ParseMetrics::default(),
            errors: ParseErrorLog {
                row_errors: Vec::new(),
                detail_errors: Vec::new(),
            },
        }
    }

    pub fn inc_total(&mut self) {
        self.metrics.total_lines += 1;
    }
    // ---------------- ROW ----------------

    pub fn log_row_success(&mut self) {
        self.metrics.row_success += 1;
    }

    pub fn log_row_error(&mut self, message: String, line: String) {
        self.metrics.row_fail += 1;
        self.errors.row_errors.push(ParseError {
            error_type: ErrorType::RowParse,
            message,
            line,
        });
    }

    // ---------------- DETAIL ----------------

    pub fn log_detail_success(&mut self) {
        self.metrics.detail_success += 1;
    }

    pub fn log_detail_error(&mut self, message: String, line: String) {
        self.metrics.detail_fail += 1;
        self.errors.detail_errors.push(ParseError {
            error_type: ErrorType::DetailParse,
            message,
            line,
        });
    }

    // ---------------- VALIDATION ----------------

    pub fn log_valid_success(&mut self) {
        self.metrics.valid_success += 1;
    }

    pub fn log_valid_error(&mut self, message: String, line: String, error_type: ErrorType) {
        self.metrics.valid_fail += 1;
        self.errors.detail_errors.push(ParseError {
            error_type,
            message,
            line,
        });
    }

    // ---------------- REPORT ----------------

    pub fn report(&self) {
        println!("\n=== PARSE METRICS ===");
        println!("Total líneas: {}", self.metrics.total_lines);

        let pct = |x: usize| (x as f64 / self.metrics.total_lines as f64) * 100.0;

        println!("Row success: {} ({:.2}%)", self.metrics.row_success, pct(self.metrics.row_success));
        println!("Detail success: {} ({:.2}%)", self.metrics.detail_success, pct(self.metrics.detail_success));
        println!("Valid success: {} ({:.2}%)", self.metrics.valid_success, pct(self.metrics.valid_success));

        println!("\n=== ERROR SUMMARY ===");

        let mut counts = std::collections::HashMap::new();

        for err in self.errors.detail_errors.iter().chain(self.errors.row_errors.iter()) {
            *counts.entry(format!("{:?}", err.error_type)).or_insert(0) += 1;
        }

        for (k, v) in counts {
            println!("{}: {}", k, v);
        }
    }

    pub fn handle_validation(
        &mut self,
        errors: Vec<(ErrorType, String)>,
        line: String,
    ) {
        if errors.is_empty() {
            self.log_valid_success();
        } else {
            for (error_type, message) in errors {
                self.log_valid_error(message, line.clone(), error_type);
            }
        }
    }
}