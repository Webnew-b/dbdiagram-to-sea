use std::path::Path;

use db_diagram_to_sea_orm::tools::get_file_content;
use db_diagram_to_sea_orm::{parse_file, ParserResult};
use log::debug;

fn main() -> ParserResult<()> {
    env_logger::init();
    let file_path = Path::new("input/test.dbml");

    let file_content = get_file_content(file_path)?;
    let file_content_static :&'static str = Box::leak(file_content.into_boxed_str());

    debug!("{}",file_content_static);

    parse_file(file_content_static)?;
    
    Ok(())
}

