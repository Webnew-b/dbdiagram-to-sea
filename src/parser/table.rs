use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{complete, map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};

use crate::db_type::{Column, Table};
use crate::parser::{parse_attr, parse_ident, parse_type};

pub fn parse_column(input:&str) -> IResult<&str,Column> {
    let mut parser = map (
        (
            preceded(multispace0, parse_ident),
            preceded(multispace1, parse_type),
            opt(preceded(multispace1, parse_attr)
        )
        ),
        |(name,field_type,attrs)| Column {
            name:name.to_string(),
            field_type:field_type.to_string(),
            attrs,
        });
    parser.parse(input)
}

pub fn parse_table(input:&str) -> IResult<&str,Table> {
    let (input,_) = preceded(multispace0,tag("Table") ).parse(input)?;
    let (input,_) = multispace1(input)?;
    let (input,name) = parse_ident(input)?;
    let (input,_) = multispace0(input)?;
    let (input,columns) = delimited(
        tag("{"),
        many0(terminated(parse_column,multispace0)), 
        tag("}")
        ).parse(input)?;

    Ok((input,Table {
        name:name.to_string(),
        columns
    }))
}

pub fn parse_tables(input:&str) -> IResult<&str,Vec<Table>> {
    let mut parser = many0(
            preceded(multispace0, complete(parse_table))
        );
    parser.parse(input)
}
