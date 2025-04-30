use std::fs;
use std::path::Path;

use crate::error_enum::{AppResult, ParserErrorKind};




pub fn get_file_content(path:&Path) -> AppResult<String> {
    fs::read_to_string(path).map_err(|e|{
        eprintln!("{}",e.to_string());
        ParserErrorKind::ReadFileFailed.into()
    })
}
