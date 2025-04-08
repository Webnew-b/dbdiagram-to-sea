use nom::bytes::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{complete, map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};

use crate::db_type::{ColumnEnum, ColumnEnumItem};
use crate::parser::{parse_attr, parse_ident};

pub fn parse_enum_item(input:&str) -> IResult<&str,ColumnEnumItem> {
    let mut parser = map (
        (
            preceded(multispace0, parse_ident),
            opt(preceded(multispace1, parse_attr)
        )
        ),
        |(name,attrs)| ColumnEnumItem {
            name:name.to_string(),
            attrs,
        });
    parser.parse(input)
}

pub fn parse_enum(input:&str) -> IResult<&str,ColumnEnum> {
    let (input,_) = preceded(multispace0,tag("Enum")).parse(input)?;
    let (input,_) = multispace1(input)?;
    let (input,name) = parse_ident(input)?;
    let (input,_) = multispace0(input)?;
    let (input,item) = delimited(
        tag("{"),
        many0(terminated(parse_enum_item,multispace0)), 
        tag("}")
        ).parse(input)?;

    Ok((input,ColumnEnum {
        name:name.to_string(),
        item
    }))
}

pub fn parse_enums(input:&str) -> IResult<&str,Vec<ColumnEnum>> {
    let mut parser = many0(
            preceded(multispace0, complete(parse_enum))
        );
    parser.parse(input)
}
