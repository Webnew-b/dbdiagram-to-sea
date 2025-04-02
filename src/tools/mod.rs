use std::fs;
use std::path::Path;


use crate::error_enum::ParserError;
use crate::ParserResult;



pub fn get_file_content(path:&Path) -> ParserResult<String> {
    fs::read_to_string(path).map_err(|e|{
        eprintln!("{}",e.to_string());
        ParserError::ReadFileFailed
    })
}
