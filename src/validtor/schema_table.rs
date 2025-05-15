
use regex::Regex;
use serde::Deserialize;

use crate::error_enum::schema_error::SchemaErrorKind;
use crate::error_enum::{AppError, AppResult};
use crate::validtor::vailtor_schema::compile_regex;


#[derive(Deserialize,Debug)]
pub(crate) struct SchemaTable {
    allow_type:Vec<String>,
    allow_name:String,
    allow_column_name:String,
    allow_column_attr:Vec<String>,
}

pub(crate) struct SchemaTableRegex{
    allow_name:Regex,
    allow_column_name:Regex,
}

impl TryFrom<SchemaTable> for SchemaTableRegex {

    type Error = AppError;

    fn try_from(value: SchemaTable) -> Result<Self,Self::Error> {
        let allow_name = compile_regex(value.allow_name.as_str())?;
        let allow_column_name = compile_regex(value.allow_column_name.as_str())?;
        Ok(Self { 
            allow_name,
            allow_column_name
        })
    }
}

impl SchemaTable {

    pub fn check_colmun_type(&self,target:String) -> AppResult<()> {
        let res = self.allow_type.contains(&target.to_uppercase());
        if !res {
            return Err(SchemaErrorKind::NoContained { colum_type:target }.into());
        }
        Ok(())
    }

    pub fn check_colmun_attr(&self,target:String) -> AppResult<()> {
        let res = self.allow_column_attr.contains(&target.to_uppercase());
        if !res {
            return Err(SchemaErrorKind::NoContained { colum_type:target }.into());
        }
        Ok(())
    }
}
