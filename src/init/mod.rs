use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;

use crate::error_enum::init_error::InitErrorKind;
use crate::error_enum::AppResult;
use crate::init::default_config::FileDefaultConfig;

pub(crate) mod default_config;

#[derive(Debug)]
pub struct ConfigPath{
    input_path:PathBuf,
    schema_config:PathBuf,
    generation_config:PathBuf,
    template_path:PathBuf,
    output_path:PathBuf,
}

impl ConfigPath{
    pub fn schema_config<'path>(&'path self) -> &'path Path {
        self.schema_config.as_path()
    }
    pub fn generation_config<'path>(&'path self) -> &'path Path {
        self.generation_config.as_path()
    }
    pub fn template_path<'path>(&'path self) -> &'path Path {
        self.template_path.as_path()
    }
    pub fn output_path<'path>(&'path self) -> &'path Path {
        self.output_path.as_path()
    }
    pub fn input_path<'path>(&'path self) -> &'path Path {
        self.input_path.as_path()
    }
}

///DBML to SeaORM Migration Generator.
#[derive(Parser,Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    ///Path to the input DBML file.
    input:PathBuf,

    ///Directory for the output SeaORM migration.
    #[arg(short,long)]
    output:Option<PathBuf>,
}

fn create_file_if_not_exist(p:&Path,content:&str) -> AppResult<()> {
    if !p.exists() {
        fs::write(p,content)
           .map_err(|e|{
               eprintln!("It has an error been occured while create configration:{}",e);
               InitErrorKind::ConfigrationCouldNotCreated
           })?;
    }
    Ok(()) 
}

fn create_config(d:&FileDefaultConfig) -> AppResult<(PathBuf,PathBuf,PathBuf)>{
    let config_dir = PathBuf::from("config/");
    if !config_dir.exists() {
        fs::create_dir(config_dir.clone()).map_err(|e|{
            eprintln!("It has an error been occured while create config folder:{}",e);
            InitErrorKind::ConfigrationCouldNotCreated
        })?;
    }

    let generation = config_dir.join("generate_config.toml");
    create_file_if_not_exist(generation.as_path(), d.generate_config())?;
    let schema = config_dir.join("schema_config.toml");
    create_file_if_not_exist(schema.as_path(), d.schema_config())?;

    let template_dir = PathBuf::from("templates/");
    if !template_dir.exists() {
        fs::create_dir(template_dir.clone()).map_err(|e|{
            eprintln!("It has an error been occured while create the template folder:{}",e);
            InitErrorKind::TemplateFolderConldNotCreated
        })?;
    }

    let template_file = template_dir.join("migrate_template.rs.txt");
    create_file_if_not_exist(template_file.as_path(), d.migrate_template())?;

    Ok((generation,schema,template_file))
}

pub fn init() -> AppResult<ConfigPath> {
    let cli = Args::parse();
    log::info!("Obtaining input file and generating default configuration.");
    let default = FileDefaultConfig::new();
    let input_path = cli.input;

    input_path.exists()
        .then_some(())
        .ok_or(InitErrorKind::InputFileNotFound)?;
    
    input_path.is_file()
        .then_some(())
        .ok_or(InitErrorKind::InputMustBeFlie)?;

    let (generation_config,schema_config,template_path)
        = create_config(&default)?;

    let output_path = if let Some(o) = cli.output {
        o.is_dir()
            .then_some(())
            .ok_or(InitErrorKind::OutputMustBeFolder)?;
        o
    } else {
        let output_path = PathBuf::from("output/");
        fs::create_dir(output_path.clone()).map_err(|e|{
            eprintln!("It has an error been occured while create the output folder:{}",e);
            InitErrorKind::OutputFolderCouldNotCreated
        })?;
        output_path
    };
    let global_path = ConfigPath {
        input_path,
        generation_config,
        schema_config,
        template_path,
        output_path,
    };
    Ok(global_path)
}
