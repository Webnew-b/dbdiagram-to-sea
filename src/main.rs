use std::path::Path;

use db_diagram_to_sea_orm::parser::parse_table;
use db_diagram_to_sea_orm::tools::get_file_content;
use db_diagram_to_sea_orm::ParserResult;
use log::{error, info};

fn main() -> ParserResult<()> {
    env_logger::init();
    let file_path = Path::new("input/test.dbml");

    let file_content = get_file_content(file_path)?;
    let file_content_static :&'static str = Box::leak(file_content.into_boxed_str());

    match parse_table(file_content_static) {
       Ok((_,table))=>{
           info!("{}",table);
       },
       Err(e) => {
           error!("{}",e.to_string())
       }
    }
    Ok(())
}

/*
use nom::{
    bytes::complete::{tag, take_while1}, character::complete::multispace0, combinator::map, multi::separated_list0, sequence::{delimited, preceded}, IResult, Parser
};



pub fn parse_attr(input: &str) -> IResult<&str, Vec<String>> {
    let mut parser = delimited(
        tag("["),
        separated_list0(
            tag(","),
            map(
                preceded(
                    whitespace0,
                    take_while1(|c: char| c != ',' && c != ']'),
                ),
                |s: &str| s.trim().to_string(),
            ),
        ),
        tag("]"),
    );
    parser.parse(input)
}

fn main() {
    let result = parse_attr("[a, b, 中文字段]").unwrap();
    println!("{:?}", result);
}
*/
