use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::{complete, map};
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{IResult, Parser};

use crate::db_type::GlobalDefinition;
use crate::parser::column_enum::parse_enum;
use crate::parser::relation::parse_relation;
use crate::parser::table::parse_table;

pub mod table;
pub mod column_enum;
pub mod relation;

pub(crate) fn whitespace0(input: &str) -> IResult<&str, &str> {
    nom::character::complete::space0::<&str, nom::error::Error<&str>>(input)
}

pub(super) fn is_ident_char(c:char) -> bool{
    c.is_alphanumeric() || c == '_' || c >= '\u{4E00}'
}


pub(super) fn parse_definition(input:&str) -> IResult<&str,GlobalDefinition>{
    let mut parser = preceded(
        multispace0,
        complete(alt((
            map(parse_table,GlobalDefinition::Table),
            map(parse_enum, GlobalDefinition::Enum),
            map(parse_relation, GlobalDefinition::Relation)
        )))
    );
    parser.parse(input)
}

pub(crate) fn parse_all(input:&str) -> IResult<&str,Vec<GlobalDefinition>> {
    many0(parse_definition).parse(input)
}


