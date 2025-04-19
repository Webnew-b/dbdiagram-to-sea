use std::str;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{complete, map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated};
use nom::{Err, IResult, Parser};

use crate::db_type::relation::{get_relation_from_str, Relation, RelationEnum};
use crate::parser::is_ident_char;

fn parse_ident(input:&str) -> IResult<&str,&str> {
    take_while1(is_ident_char)(input)
}

fn parse_relation_table(input:&str) -> 
    IResult<&str,(Option<String>,String,String)> 
{
    let mut parse = map((
            opt(terminated(parse_ident, tag("."))),
            parse_ident,
            preceded(tag("."), parse_ident)
            ), 
        |(schema,table,column)| 
        (schema.map(String::from),table.to_string(),column.to_string())
        );
    parse.parse(input)
}

fn parse_colon(input:&str) -> IResult<&str,&str> {
    let mut a = delimited(
        multispace0,
        tag(":"), 
        multispace0
    );
    a.parse(input)
}

fn parse_relation_type(input:&str) -> IResult<&str,RelationEnum> {
    let opt_parse = alt((
            tag("<>"),
            tag("<"),
            tag(">"),
            tag("-")
            ));
    let mut parse = delimited(
        multispace0,
        opt_parse, 
        multispace0
    );
    let (input,res) = parse.parse(input)?;

    let relation_enum = get_relation_from_str(res)
        .ok_or_else(|| Err::Error(
                nom::error::Error::new(
                    input, 
                    nom::error::ErrorKind::Tag
                    )
                ))?;

    Ok((input,relation_enum))
}

pub fn parse_relation(input:&str) -> IResult<&str,Relation> {
    let (input,_) = preceded(multispace0,tag("Ref") ).parse(input)?;
    let (input,_) = multispace1(input)?;
    let (input,name) = parse_ident(input)?;
    let (input,_) = parse_colon(input)?;
    let (input,(from_schema,from_table,from_column)) = parse_relation_table(input)?;
    let (input,relation) = parse_relation_type(input)?;
    let (input,(to_schema,to_table,to_column)) = parse_relation_table(input)?;

    Ok((input,Relation {
        name:name.to_string(),
        from_schema,
        from_table,
        from_column,
        to_schema,
        to_table,
        to_column,
        relation,
    }))
}

pub fn parse_relations(input:&str) -> IResult<&str,Vec<Relation>> {
    let mut parser = many0(
            preceded(multispace0, complete(parse_relation))
        );
    parser.parse(input)
}

#[cfg(test)]
mod test {

    use super::parse_relations;

    #[test]
    fn test_parse_relations() {
        let _ = env_logger::try_init();
        let example = "Ref c_to_d: schema1.C.id < schema1.D.cId";
        let (input,res) = parse_relations(example).unwrap();
        log::info!("this is :{},{:#?}",input,res);
    }
}
