use std::path::Path;

use db_diagram_to_sea_orm::error_enum::AppResult;
use db_diagram_to_sea_orm::tools::get_file_content;
use db_diagram_to_sea_orm::{generate_file, parse_file};
use log::debug;

fn main() -> AppResult<()> {
    env_logger::init();
    let file_path = Path::new("input/test.dbml");

    let file_content = get_file_content(file_path)?;
    let file_content_static :&'static str = Box::leak(file_content.into_boxed_str());

    debug!("{}",file_content_static);

    let res = parse_file(file_content_static)?;

    generate_file(res)?;
    Ok(())
}

