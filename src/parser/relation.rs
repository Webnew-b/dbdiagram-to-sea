use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::{Err, IResult, Parser};

use crate::db_type::relation::{get_action_from_str, get_relation_from_str, ReferentialAction, Relation, RelationEnum};
use crate::parser::is_ident_char;

fn parse_ident(input:&str) -> IResult<&str,&str> {
    take_while1(is_ident_char)(input)
}

fn parse_relation_table(input:&str) -> 
    IResult<&str,(Option<String>,String,String)> 
{
    let (input,first) = parse_ident(input)?;
    let (input,second) = preceded(tag("."), parse_ident).parse(input)?;

    if let Ok((input,third)) = preceded(tag("."), parse_ident).parse(input) {
        return Ok(
            (input,(Some(first.to_string()),second.to_string(),third.to_string()))
        );
    } else {
        return Ok((input,(None,first.to_string(),second.to_string())));
    }
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

fn parse_action_item(input:&str) 
    -> IResult<&str,(&str,Option<ReferentialAction>)> 
{
    let keys = alt((
                tag("cascade"),
                tag("restrict"),
                tag("set default"),
                tag("no action"),
                tag("set null"),
    ));
    let remove_space = delimited(multispace0, keys, multispace0);

    separated_pair(
        parse_ident, 
        parse_colon, 
        map(remove_space,|s:&str|{
            get_action_from_str(s)
        })
    ).parse(input)    
}

fn parse_relation_action(input:&str) 
    -> IResult<&str,
    (Option<ReferentialAction>,Option<ReferentialAction>)
    > 
{
    let action_item = separated_list0(
        delimited(multispace0,tag(","),multispace0),
        parse_action_item
    );
    let action_parse = delimited(
        tag("["), 
        action_item, 
        tag("]")
    );
    let remove_perviou_space = preceded(multispace0,action_parse);
    
    let opt_parse = opt(
             remove_perviou_space
        );
    map(opt_parse, |opt_pair|{
        let mut delete = None;
        let mut update = None;
        if let Some(pair) = opt_pair {
            for (k,v) in pair {
                match k {
                    "update" => update = v,
                    "delete" => delete = v,
                    _ => ()
                }
            }
        }
        (delete,update)
    }).parse(input)
}

pub(crate) fn parse_relation(input:&str) -> IResult<&str,Relation> {
    let (input,_) = preceded(multispace0,tag("Ref") ).parse(input)?;
    let (input,_) = parse_colon(input)?;
    let (input,(from_schema,from_table,from_column)) = parse_relation_table(input)?;
    let (input,relation) = parse_relation_type(input)?;
    let (input,(to_schema,to_table,to_column)) = parse_relation_table(input)?;
    let (input,(delete_action,update_action)) = parse_relation_action(input)?;

    Ok((input,Relation {
        from_schema,
        from_table,
        from_column,
        to_schema,
        to_table,
        to_column,
        relation,
        delete_action,
        update_action
    }))
}

#[cfg(test)]
mod test {
    use crate::parser::relation::parse_relation;

    #[test]
    fn test_parse_relation() {
        let _ = env_logger::try_init();
        let example = "Ref: rental_request.venue_id > venue.id [delete: cascade, update: no action]";
        let res = parse_relation(example);
        let (input,res) = match res {
            Ok(e) => e,
            Err(e) => panic!("{}",e),
        };
        log::info!("this is :{},{:#?}",input,res);
    }

    /*
    #[test]
    fn test_parse_relation_action() {
        let _ = env_logger::try_init();
        let example = " [delete: cascade, update: no action]";
        let res = parse_relation_action(example);
        let (input,res) = match res {
            Ok(e) => e,
            Err(e) => panic!("{}",e),
        };
        log::info!("this is :{},{:#?}",input,res);
    }
    */
}
