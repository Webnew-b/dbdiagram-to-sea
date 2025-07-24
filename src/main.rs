use db_diagram_to_sea_orm::error_enum::AppResult;
use db_diagram_to_sea_orm::init::init;
use db_diagram_to_sea_orm::tools::get_file_content;
use db_diagram_to_sea_orm::{generate_file, parse_file};

fn main() -> AppResult<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let global_path = init()?;

    let file_content = get_file_content(global_path.input_path())?;
    let file_content_static :&'static str = Box::leak(file_content.into_boxed_str());

    log::info!("Parsing and validating the DBML file.");
    let res = parse_file(file_content_static,global_path.schema_config())?;

    log::info!("Generating migration file.");
    generate_file(res,global_path)?;
    log::info!("Generation is succeed.");
    Ok(())
}

