use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::{complete, map};
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{IResult, Parser};
use log::{error,info};

use crate::db_type::GlobalDefinition;
use crate::error_enum::ParserError;
use crate::parser::column_enum::parse_enum;
use crate::parser::table::parse_table;
use crate::ParserResult;

pub mod table;
pub mod column_enum;

pub(crate) fn whitespace0(input: &str) -> IResult<&str, &str> {
    nom::character::complete::space0::<&str, nom::error::Error<&str>>(input)
}

pub fn is_ident_char(c:char) -> bool{
    c.is_alphanumeric() || c == '_' || c >= '\u{4E00}'
}


pub fn parse_definition(input:&str) -> IResult<&str,GlobalDefinition>{
    let mut parser = preceded(
        multispace0,
        complete(alt((
            map(parse_table,GlobalDefinition::Table),
            map(parse_enum, GlobalDefinition::Enum)
        )))
    );
    parser.parse(input)
}

pub fn parse_all(input:&str) -> IResult<&str,Vec<GlobalDefinition>> {
    many0(parse_definition).parse(input)
}

pub fn parse_file(input:&str) -> ParserResult<()> {

    let res = parse_all(input).map_err(|e|{
        error!("{}",e.to_string());
        ParserError::ParseEnumFail
    })?;


    for table in res.1 {
        info!("{:?}",table);
    }

    Ok(())
}
