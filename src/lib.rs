use crate::error_enum::ParserError;
use crate::parser::parse_all;

pub mod db_type;
pub mod error_enum;
pub mod tools;
pub mod parser;

pub type ParserResult<T> = Result<T,ParserError>;

pub fn parse_file(input:&str) -> ParserResult<()> {

    let (_,res) = parse_all(input).map_err(|e|{
        log::error!("{}",e.to_string());
        ParserError::ParseEnumFail
    })?;


    for table in res {
        log::info!("{:#?}",table);
    }

    Ok(())
}
