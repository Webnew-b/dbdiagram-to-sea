
use std::collections::HashSet;

use regex::Regex;
use serde::Deserialize;

use crate::error_enum::AppError;
use crate::validator::valid_schema::compile_regex;


#[derive(Deserialize,Debug)]
pub(crate) struct SchemaTable {
    allow_type:Vec<String>,
    allow_name:String,
    allow_column_name:String,
    allow_column_attr:Vec<String>,
}

pub(crate) struct SchemaTableRegex{
    pub allow_name:Regex,
    pub allow_column_name:Regex,
    pub allow_type:HashSet<String>,
    pub allow_column_attr:HashSet<String>,
}

impl TryFrom<SchemaTable> for SchemaTableRegex {

    type Error = AppError;

    fn try_from(value: SchemaTable) -> Result<Self,Self::Error> {
        let allow_name = compile_regex(value.allow_name.as_str())?;
        let allow_column_name = compile_regex(value.allow_column_name.as_str())?;
        let allow_type = value.allow_type.into_iter().collect();
        let allow_column_attr = value.allow_column_attr.into_iter().collect();
        Ok(Self { 
            allow_name,
            allow_column_name,
            allow_type,
            allow_column_attr
        })
    }
}

