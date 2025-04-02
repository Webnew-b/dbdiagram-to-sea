use nom::bytes::complete::take_while1;
use nom::bytes::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{map, opt};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};

use crate::db_type::{Column, Table};

fn whitespace0(input: &str) -> IResult<&str, &str> {
    nom::character::complete::space0::<&str, nom::error::Error<&str>>(input)
}

pub fn is_ident_char(c:char) -> bool{
    c.is_alphanumeric() || c == '_' || c >= '\u{4E00}'
}

pub fn parse_ident(input:&str) -> IResult<&str,&str> {
    take_while1(is_ident_char)(input)
}

pub fn parse_type(input:&str) -> IResult<&str,&str> {
    take_while1(|c:char| c.is_alphanumeric())(input)
}

pub fn parse_attr(input:&str) -> IResult<&str,Vec<String>>{
    let sep = preceded(
                whitespace0,
                take_while1(|c:char| c != ',' && c != ']')
                );
    let map_fn = map(sep, |s:&str| s.trim().to_string());

    let sep_list = separated_list0(tag(","), map_fn); 
    let mut parser = delimited(
        tag("["), 
        sep_list,
        tag("]")
    );
    parser.parse(input)
}

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

