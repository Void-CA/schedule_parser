#[derive(Debug, Clone)]
pub enum ErrorType {
    RowParse,
    DetailParse,
    Room,
    Group,
    Subject,
    Other,
}

#[derive(Debug)]
pub struct ParseError {
    pub error_type: ErrorType,
    pub message: String,
    pub line: String,
}