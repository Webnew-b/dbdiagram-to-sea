use std::path::PathBuf;
use std::str::FromStr;

use regex::Regex;
use serde::Deserialize;

use crate::error_enum::schema_error::SchemaErrorKind;
use crate::error_enum::{AppError, AppResult};
use crate::tools::get_file_content;
use crate::validtor::schema_enum::SchemaEnum;
use crate::validtor::schema_relation::SchemaRelation;
use crate::validtor::schema_table::SchemaTable;

pub(crate) fn read_config(file_path:&str) -> AppResult<VailtorSchema> {
    let path = PathBuf::from_str(file_path).unwrap();
        
    let config_string = get_file_content(path.as_path())?;

    toml::from_str::<VailtorSchema>(config_string.as_str())
        .map_err(|e|{
            AppError::new(SchemaErrorKind::FileFormatFile.into(),e.into())
        })
}

pub(crate) fn validate_field_from_regex
    (pattern:&Regex,target:&str,type_name:String) -> AppResult<()> {
    pattern.is_match(target)
        .then(|| {()})
        .ok_or_else(||{
            SchemaErrorKind::VailateFieldFail { 
                field: target.to_string(), 
                field_type: type_name
            }
        })?;
    Ok(())
}

pub(super) fn compile_regex(pattern:&str) -> AppResult<Regex> {
    Regex::new(pattern).map_err(|e|{
        AppError::new(
            SchemaErrorKind::RegexMatchError { 
                match_by: pattern.to_string() 
            }.into(),
            e.into())
    })
}



#[derive(Deserialize,Debug)]
pub(crate) struct VailtorSchema {
    pub table:SchemaTable,
    #[serde(rename = "enum")]
    pub column_enum:SchemaEnum,
    pub relation:SchemaRelation
}


