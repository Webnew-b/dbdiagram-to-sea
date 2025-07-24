use std::fmt;

use thiserror::Error;

use crate::error_enum::generation_error::GenerationErrorKind;
use crate::error_enum::init_error::InitErrorKind;
use crate::error_enum::schema_error::SchemaErrorKind;

pub(crate) mod schema_error;
pub(crate) mod generation_error;
pub mod init_error;

#[derive(Debug,Error)]
pub enum AppErrorKind {
    #[error("Schema error: {0} ")]
    SchemaErrorKind(#[from] SchemaErrorKind),

    #[error("Parse error: {0}")]
    ParserErrorKind(#[from] ParserErrorKind),

    #[error("Generate error: {0}")]
    GenerationErrorKind(#[from] GenerationErrorKind),

    #[error("Init client error:{0}")]
    InitErrorKind(#[from] InitErrorKind),

    #[error("Other error:{0}")]
    Other(String)
}

type GenericsError = Box<dyn std::error::Error + Send + Sync>; 

#[derive(Debug)]
pub struct AppError {
    pub kind:AppErrorKind,
    pub source:Option<GenericsError>
}

impl AppError {
    pub fn new(kind:AppErrorKind,source:GenericsError)->Self {
        Self { kind, source: Some(source) }
    }
}

impl From<AppErrorKind> for AppError {
    fn from(value: AppErrorKind) -> Self {
        Self { kind: value, source: None }
    }
}

impl From<SchemaErrorKind> for AppError {
    fn from(value: SchemaErrorKind) -> Self {
        Self {kind:AppErrorKind::SchemaErrorKind(value),source:None}
    }
}

impl From<ParserErrorKind> for AppError {
    fn from(value: ParserErrorKind) -> Self {
        Self {kind:AppErrorKind::ParserErrorKind(value),source:None}
    }
}

impl From<GenerationErrorKind> for AppError {
    fn from(value: GenerationErrorKind) -> Self {
        Self {kind:AppErrorKind::GenerationErrorKind(value),source:None}
    }
}

impl From<InitErrorKind> for AppError {
    fn from(value: InitErrorKind) -> Self {
        Self {kind:AppErrorKind::InitErrorKind(value),source:None}
    }
}

pub type AppResult<T> = std::result::Result<T,AppError>;


#[derive(Debug)]
pub enum ParserErrorKind {
    ParserNotFound,
    OpenFileFailed,
    ReadFileFailed,
    ParseTableFail,
    ParseEnumFail,

    NameDuplicated(String),
    ItemNameDuplicated(String,String),
}

impl fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorKind::ParserNotFound => write!(f,"Nothing was parsed from the file."),
            ParserErrorKind::OpenFileFailed => write!(f,"Could not open file."),
            ParserErrorKind::ReadFileFailed => write!(f,"Could not read file."),
            ParserErrorKind::ParseTableFail => write!(f,"Fail to parse the table from the file."),
            ParserErrorKind::ParseEnumFail => write!(f,"Fail to parse the enum from the file."),
                       
            ParserErrorKind::NameDuplicated(e) => write!(f,"Duplicate element name found in file:{}",e),
            ParserErrorKind::ItemNameDuplicated(k, v) 
                => write!(f,"Duplicate element found in {}:{}.",k,v),
        }
    }
}

impl std::error::Error for ParserErrorKind {
    
}


