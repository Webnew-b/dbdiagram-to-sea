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
            parse_ident,
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

fn parse_string_value(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(char('\"'), is_not("\""), char('\"')), // Handles double quotes
        delimited(char('\''), is_not("'"), char('\'')),  // Handles single quotes
        delimited(char('`'), is_not("`"), char('`')),    // Handles backticks
    ))
    .parse(input)
}

fn parse_attr_item(input:&str)-> IResult<&str,AttrEnum> {
    preceded(multispace0, 
        alt((
            map(
                tag("not null"),
                |_| AttrEnum::Sigle("not null".to_string())
            ),
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

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_parse_full_user_table() {
        // The input string to be parsed.
        // Using a raw string literal r#"..."# is convenient for multi-line strings with quotes.
        let table_definition = r#"
        Table user {
            id BIGSERIAL [pk]
            username varchar(255) [not null, note: "用户名"]
            email varchar(255) [not null, note: "邮箱，用于登录"]

            avatar varchar(255) [note: "头像URL (MVP阶段可简化处理或非强制)"]
            gender user_gender [default: 'prefer_not_to_say', note:"用户性别 (MVP阶段可简化处理或非强制)"]
            introduce varchar(300) [note: "用户简介 (MVP阶段可简化处理或非强制)"]
            is_show bool [default: false, note: "是否公开显示个人资料 (MVP阶段可简化处理或非强制)"]

            is_delete bool [default: false, note: "软删除标志"]
            status userStatus [default: 'active', note: "用户账户状态"]

            createTime timestamp [default: `now()`, note: "创建时间"]
            updateTime timestamp [note: "更新时间"]
        }
        "#;

        // The expected structure that the parser should produce.
        // We build this manually to compare with the parser's output.
        let expected = Table {
            name: "user".to_string(),
            alias: None,
            note: None,
            columns: vec![
                Column {
                    name: "id".to_string(),
                    field_type: FieldType { name: "BIGSERIAL".to_string(), amount: None },
                    attrs: Some(vec![AttrEnum::Sigle("pk".to_string())])
                },
                Column {
                    name: "username".to_string(),
                    field_type: FieldType { name: "varchar".to_string(), amount: Some("255".to_string()) },
                    attrs: Some(vec![
                        AttrEnum::Sigle("not null".to_string()),
                        AttrEnum::KeyValue("note".to_string(), "用户名".to_string())
                    ])
                },
                Column {
                    name: "email".to_string(),
                    field_type: FieldType { name: "varchar".to_string(), amount: Some("255".to_string()) },
                    attrs: Some(vec![
                        AttrEnum::Sigle("not null".to_string()),
                        AttrEnum::KeyValue("note".to_string(), "邮箱，用于登录".to_string())
                    ])
                },
                Column {
                    name: "avatar".to_string(),
                    field_type: FieldType { name: "varchar".to_string(), amount: Some("255".to_string()) },
                    attrs: Some(vec![AttrEnum::KeyValue("note".to_string(), "头像URL (MVP阶段可简化处理或非强制)".to_string())])
                },
                Column {
                    name: "gender".to_string(),
                    field_type: FieldType { name: "user_gender".to_string(), amount: None },
                    attrs: Some(vec![
                        AttrEnum::KeyValue("default".to_string(), "prefer_not_to_say".to_string()),
                        AttrEnum::KeyValue("note".to_string(), "用户性别 (MVP阶段可简化处理或非强制)".to_string())
                    ])
                },
                Column {
                    name: "introduce".to_string(),
                    field_type: FieldType { name: "varchar".to_string(), amount: Some("300".to_string()) },
                    attrs: Some(vec![AttrEnum::KeyValue("note".to_string(), "用户简介 (MVP阶段可简化处理或非强制)".to_string())])
                },
                Column {
                    name: "is_show".to_string(),
                    field_type: FieldType { name: "bool".to_string(), amount: None },
                    attrs: Some(vec![
                        AttrEnum::KeyValue("default".to_string(), "false".to_string()),
                        AttrEnum::KeyValue("note".to_string(), "是否公开显示个人资料 (MVP阶段可简化处理或非强制)".to_string())
                    ])
                },
                Column {
                    name: "is_delete".to_string(),
                    field_type: FieldType { name: "bool".to_string(), amount: None },
                    attrs: Some(vec![
                        AttrEnum::KeyValue("default".to_string(), "false".to_string()),
                        AttrEnum::KeyValue("note".to_string(), "软删除标志".to_string())
                    ])
                },
                Column {
                    name: "status".to_string(),
                    field_type: FieldType { name: "userStatus".to_string(), amount: None },
                    attrs: Some(vec![
                        AttrEnum::KeyValue("default".to_string(), "active".to_string()),
                        AttrEnum::KeyValue("note".to_string(), "用户账户状态".to_string())
                    ])
                },
                Column {
                    name: "createTime".to_string(),
                    field_type: FieldType { name: "timestamp".to_string(), amount: None },
                    attrs: Some(vec![
                        AttrEnum::KeyValue("default".to_string(), "now()".to_string()),
                        AttrEnum::KeyValue("note".to_string(), "创建时间".to_string())
                    ])
                },
                Column {
                    name: "updateTime".to_string(),
                    field_type: FieldType { name: "timestamp".to_string(), amount: None },
                    attrs: Some(vec![AttrEnum::KeyValue("note".to_string(), "更新时间".to_string())])
                },
            ]
        };

        // Run the parser.
        let result = parse_table(table_definition);

        let result = match result {
            Ok(e) => e,
            Err(e) => {
                log::debug!("{}",e);
                return ();
            },
        };

        // Assert that the parsing was successful.

        // Extract the parsed data.
        let (remaining_input, parsed_table) = result;

        // Assert that the entire input was consumed.
        assert!(remaining_input.trim().is_empty());

        // Assert that the parsed table is exactly what we expected.
        assert_eq!(parsed_table, expected);
    }
}
