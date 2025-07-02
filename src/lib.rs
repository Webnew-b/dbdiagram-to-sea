use crate::error_enum::{AppResult, ParserErrorKind};
use crate::parser::parse_all;
use crate::validtor::validate_sturcture;

pub mod db_type;
pub mod error_enum;
pub mod tools;
pub mod parser;
pub mod validtor;


pub fn parse_file(input:&str) -> AppResult<()> {

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

    Ok(())
}
