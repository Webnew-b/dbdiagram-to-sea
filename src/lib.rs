use crate::db_type::GlobalDefinition;
use crate::error_enum::{AppResult, ParserErrorKind};
use crate::generation::generate_migrate_file;
use crate::parser::parse_all;
use crate::validator::validate_sturcture;

pub mod db_type;
pub mod error_enum;
pub mod tools;
pub mod parser;
pub mod validator;
pub mod generation;


pub fn parse_file(input:&str) -> AppResult<Vec<GlobalDefinition>> {
    let (_,res) = parse_all(input).map_err(|e|{
        log::error!("{}",e.to_string());
        ParserErrorKind::ParseEnumFail
    })?;


    for table in &res {
        log::info!("{:#?}",table);
    }

    validate_sturcture(&res)?;

    log::info!("Validation passed.");

    for table in &res {
        log::info!("{:#?}",table);
    }

    Ok(res)
}

pub fn generate_file(structure:Vec<GlobalDefinition>)->AppResult<()> {
    generate_migrate_file(structure)?;
    Ok(())
}
