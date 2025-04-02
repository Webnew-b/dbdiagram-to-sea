use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    Other(String),
    ParserNotFound,
    OpenFileFailed,
    ReadFileFailed,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Other(e) => write!(f,"{}",e),
            ParserError::ParserNotFound => write!(f,"Nothing was parsed from the file."),
            ParserError::OpenFileFailed => write!(f,"Could not open file."),
            ParserError::ReadFileFailed => write!(f,"Could not read file."),
        }
    }
}
