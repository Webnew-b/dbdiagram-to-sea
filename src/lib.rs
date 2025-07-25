use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::db_type::GlobalDefinition;
use crate::error_enum::schema_error::SchemaErrorKind;
use crate::error_enum::{AppError, AppResult, ParserErrorKind};
use crate::generation::generate_migrate_file;
use crate::init::ConfigPath;
use crate::parser::parse_all;
use crate::tools::get_file_content;
use crate::validator::validate_sturcture;

pub mod db_type;
pub mod error_enum;
pub mod tools;
pub mod parser;
pub mod validator;
pub mod generation;
pub mod init;

#[derive(Deserialize,Serialize)]
struct DefaultConfig {
    pub default_value:DefaultValue,
} 
#[derive(Deserialize,Serialize)]
pub struct DefaultValue {
    pub needs_quotes : Vec<String>,
    pub no_quotes : Vec<String>,
}

fn obtain_default_value_config(p:&Path) -> AppResult<DefaultConfig> {
    let config_string = get_file_content(p)?;

    toml::from_str::<DefaultConfig>(config_string.as_str())
        .map_err(|e|{
            AppError::new(SchemaErrorKind::FileFormatFile.into(),e.into())
        })
}

pub fn parse_file(input:&str,schema_config:&Path) -> AppResult<Vec<GlobalDefinition>> {
    let (_input,res) = parse_all(input).map_err(|e|{
        log::error!("{}",e.to_string());
        ParserErrorKind::ParseEnumFail
    })?;

    log::debug!("last：{}",_input);


    for table in &res {
        log::info!("{:#?}",table);
    }

    validate_sturcture(&res,schema_config)?;

    log::info!("Validation passed.");

    for table in &res {
        log::info!("{:#?}",table);
    }

    Ok(res)
}

pub fn generate_file(
    structure:Vec<GlobalDefinition>,
    global_path:ConfigPath
)->AppResult<()> {
    let default_config = 
        obtain_default_value_config(
            global_path.generation_config()
        )?.default_value;
    generate_migrate_file(
        structure,
        default_config,
        global_path.template_path(),
        global_path.output_path()
    )?;
    Ok(())
}
