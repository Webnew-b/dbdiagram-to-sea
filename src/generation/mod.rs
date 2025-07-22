use std::fs;

use chrono::{DateTime, Utc};
use tera::{Context, Tera};

use crate::db_type::column_enum::ColumnEnum;
use crate::db_type::relation::Relation;
use crate::db_type::table::Table;
use crate::db_type::GlobalDefinition;
use crate::error_enum::generation_error::GenerationErrorKind;
use crate::error_enum::AppResult;
use crate::generation::generate_enum::generate_enums_sql;
use crate::generation::generate_relation::generate_relation_sqls;
use crate::generation::generate_table::generation_table_sqls;
use crate::DefaultValue;

pub mod generate_enum;
pub mod generate_table;
pub mod generate_relation;

pub(crate) struct Migration{
    pub name:String,
    pub up:String,
    pub down:String,
}

fn separate_gobal_defination(structure:Vec<GlobalDefinition>)
    -> AppResult<(Vec<Table>,Vec<ColumnEnum>,Vec<Relation>)> {
    let mut table: Vec<Table> = vec![];
    let mut column_enum: Vec<ColumnEnum> = vec![];
    let mut relation: Vec<Relation> = vec![];

    for item in structure {
        match item {
            GlobalDefinition::Table(t) => table.push(t),
            GlobalDefinition::Enum(e) => column_enum.push(e),
            GlobalDefinition::Relation(r) => relation.push(r),
        }
    }

    Ok((table,column_enum,relation))
}

fn create_file(content:String,file_name:String) -> AppResult<()> {
    fs::create_dir_all("output")
        .map_err(|e|{
        log::error!("{}",e.to_string());
        GenerationErrorKind::CouldNotCreateFolder("output".to_string())
    })?;
    let now:DateTime<Utc> = Utc::now();
    let file_name = format!("m{}_{}",now.format("%Y%m%d_%H%M%S"),file_name);
    let output_path = format!("output/{}.rs",file_name);
    fs::write(output_path.clone(), content).map_err(|e|{
        log::error!("{}",e.to_string());
        GenerationErrorKind::CouldNoteCreateFile(output_path)
    })?;
    Ok(())
}

fn create_migrate_file(
    sqls:Vec<Migration>,
    t:&Tera,
) -> AppResult<()> { 
    for m in sqls {
        let mut context = Context::new();
        context.insert("up_sql", m.up.as_str());
        context.insert("down_sql", m.down.as_str());
        let rendered_code = t.render("migrate_template", &context)
            .map_err(|e| {
                log::error!("{}",e.to_string());
                GenerationErrorKind::CouldNoteRenderContext
            })?;
        create_file(rendered_code,m.name)?;
    }
    Ok(())
}

pub fn generate_migrate_file(
    structure:Vec<GlobalDefinition>,
    default_config:DefaultValue
)-> AppResult<()> {
    let mut default_config = default_config;
    let mut file_template = Tera::default();
    file_template.add_template_file(
        "templates/migrate_template.rs.txt", 
        Some("migrate_template")
    ).map_err(|e| {
        log::error!("{}",e.to_string());
        GenerationErrorKind::CouldNotLoadTemplate(
            "templates/migrate_template.rs.txt".to_string()
        )
    })?;
    let (table,column_enum,relation) = separate_gobal_defination(structure)?;

    let enum_type = column_enum
        .iter()
        .map(|c|{
            c.name.to_string()
        }).collect();

    default_config.needs_quotes = [default_config.needs_quotes,enum_type].concat();

    let enum_sqls = generate_enums_sql(column_enum);
    let relation_sqls = generate_relation_sqls(relation);
    let table_sqls = generation_table_sqls(table,default_config);

    create_migrate_file(enum_sqls, &file_template)?;
    create_migrate_file(table_sqls, &file_template)?;
    create_migrate_file(relation_sqls, &file_template)?;
    Ok(())
}
