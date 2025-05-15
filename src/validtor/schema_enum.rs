
use regex::Regex;
use serde::Deserialize;

use crate::error_enum::AppError;
use crate::validtor::vailtor_schema::compile_regex;

#[derive(Deserialize,Debug)]
pub(crate) struct SchemaEnum {
    allow_name:String,
    allow_column_name:String,
}


pub(crate) struct SchemaEnumRegex {
    allow_name_regex:Regex,
    allow_column_name_regex:Regex,
}

impl TryFrom<SchemaEnum> for SchemaEnumRegex {
    type Error = AppError;

    fn try_from(value: SchemaEnum) -> Result<Self, Self::Error> {
        let allow_column_name_regex = 
            compile_regex(value.allow_column_name.as_str())?;
        let allow_name_regex = 
            compile_regex(value.allow_name.as_str())?;
        Ok(Self{
            allow_name_regex,
            allow_column_name_regex,
        })
    }
}
