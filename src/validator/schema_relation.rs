
use regex::Regex;
use serde::Deserialize;

use crate::error_enum::AppError;
use crate::validator::vailtor_schema::compile_regex;

#[derive(Deserialize,Debug)]
pub(crate) struct SchemaRelation {
    allow_name:String,
    allow_relation_name:String,
}

pub(crate) struct SchemaRelationRegex {
    pub allow_name_regex:Regex,
    pub allow_relation_name_regex:Regex,
}

impl TryFrom<SchemaRelation> for SchemaRelationRegex {
    type Error = AppError;

    fn try_from(value: SchemaRelation) -> Result<Self, Self::Error> {
        let allow_relation_name_regex = 
            compile_regex(value.allow_relation_name.as_str())?;
        let allow_name_regex = 
            compile_regex(value.allow_name.as_str())?;
        Ok(Self{
            allow_name_regex,
            allow_relation_name_regex,
        })
    }
}
