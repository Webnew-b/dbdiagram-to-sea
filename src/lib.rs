use crate::error_enum::ParserError;

pub mod db_type;
pub mod error_enum;
pub mod tools;
pub mod parser;

pub type ParserResult<T> = Result<T,ParserError>;


