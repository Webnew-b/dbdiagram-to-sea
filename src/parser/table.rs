use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while1};
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::{complete, map, opt, verify};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::{IResult, Parser};

use crate::db_type::table::{Column, FieldType, Table};
use crate::db_type::AttrEnum;
use crate::parser::{is_ident_char, whitespace0};

fn parse_alias(input:&str) -> IResult<&str,Option<String>> {
    let parser = opt(
        preceded(
            preceded(whitespace0, tag("as")),
            preceded(multispace1, parse_ident)
        )
    );
    let mut a = map(parser, |opt_alias| opt_alias.map(|e|e.to_string()));
    a.parse(input)
}

fn parse_ident(input:&str) -> IResult<&str,&str> {
    take_while1(is_ident_char)(input)
}

fn parse_digital(input:&str) -> IResult<&str,&str> {
    verify(
        take_while1(|c:char| c.is_ascii_digit()),
        |s:&str| !s.is_empty()
    ).parse(input)
}

fn parse_type(input:&str) -> IResult<&str,FieldType> {
    let mut res = map(
        (
            take_while1(|c:char| c.is_alphanumeric()),
            opt(delimited(tag("("), parse_digital, tag(")")))
        ),
        |(name,amount)| {
            FieldType {
                name:name.to_string(),
                amount:amount.map(String::from)
            }
        }
    );
    res.parse(input)
}

fn parse_string_value(input:&str) -> IResult<&str,&str> {
    delimited(char('\"'),is_not("\""), char('\"')).parse(input)
}

fn parse_attr_item(input:&str)-> IResult<&str,AttrEnum> {
    preceded(multispace0, 
        alt((
            map(
                separated_pair(
                    parse_ident, 
                    preceded(multispace0, char(':')),
                    preceded(multispace0, alt((parse_string_value,parse_ident)))
                ),
                |(k,v)| AttrEnum::KeyValue(k.to_string(),v.to_string())
            ),
            map(parse_ident, |name| AttrEnum::Sigle(name.to_string()))
        ))
        ).parse(input)
}

fn parse_attr(input:&str) -> IResult<&str,Vec<AttrEnum>>{
    let sep_list = separated_list0(tag(","), parse_attr_item); 
    let mut parser = delimited(
        tag("["), 
        sep_list,
        tag("]")
    );
    parser.parse(input)
}

fn parse_column(input:&str) -> IResult<&str,Column> {
    let mut parser = map (
        (
            preceded(multispace0, parse_ident),
            preceded(multispace1, parse_type),
            opt(preceded(multispace1, parse_attr)
        )
        ),
        |(name,field_type,attrs)| Column {
            name:name.to_string(),
            field_type,
            attrs,
        });
    parser.parse(input)
}

pub(super) fn parse_table(input:&str) -> IResult<&str,Table> {
    let (input,_) = preceded(multispace0,tag("Table") ).parse(input)?;
    let (input,_) = multispace1(input)?;
    let (input,name) = parse_ident(input)?;
    let (input,_) = multispace0(input)?;
    let (input,alias) = parse_alias(input)?;
    let (input,_) = multispace0(input)?;
    let (input,columns) = delimited(
        tag("{"),
        many1(terminated(parse_column,multispace0)),// todo implement the table note parse 
        tag("}")
        ).parse(input)?;

    Ok((input,Table {
        alias,
        name:name.to_string(),
        columns,
        note:None
    }))
}

#[allow(unused)]
pub(crate) fn parse_tables(input:&str) -> IResult<&str,Vec<Table>> {
    let mut parser = many0(
            preceded(multispace0, complete(parse_table))
        );
    parser.parse(input)
}
