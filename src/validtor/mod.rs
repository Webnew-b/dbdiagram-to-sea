use std::collections::HashMap;

use crate::db_type::column_enum::ColumnEnum;
use crate::db_type::relation::Relation;
use crate::db_type::table::Table;
use crate::db_type::{GlobalDefinition, HashName};
use crate::error_enum::{AppResult, ParserErrorKind};

pub mod vailtor_schema;
pub(crate) mod schema_enum;
pub(crate) mod schema_table;
pub(crate) mod schema_relation;


fn validate_relation(r:&Relation) -> AppResult<()> {
    Ok(())
}

fn validate_enum(e:&ColumnEnum) -> AppResult<()>{
    Ok(())
}

fn validate_table(t:&Table) -> AppResult<()>{
    Ok(())
}

fn validate_and_colloct_duplicate_names<T:HashName>(
    items:&Vec<&T>,
    category:&str
    ) -> Option<String>
{
    let dupliactions = validate_name_duplicate(items);
    if dupliactions.is_empty() {
        None
    } else {
        Some(format!("{}:{}",category.to_string(),dupliactions.join(",")))
    }
}

fn get_collection_name<T:HashName>(i:&[&T]) -> Vec<String> {
    i.iter().map(|e| {e.get_name()}).collect()
}

fn validate_name_duplicate<T:HashName>(i:&Vec<&T>) -> Vec<String> {
    let mut name_counts = HashMap::new();

    for item in i {
        let name = item.get_name();
        *name_counts.entry(name).or_insert(0) += 1;
    }

    name_counts
        .into_iter()
        .filter(|(_,count)| *count > 1)
        .map(|(name,_)| name)
        .collect()
}

fn get_validate_config() -> AppResult<()> {
    // Here must separate schema from the config and return schema ownship
    // 这里要将校验用的schema分出来。然后返回出去
    todo!();
    Ok(())
}

fn validate_duplicate_name_all(validations:Vec<Option<String>>) -> AppResult<()>{
    let mut errors = Vec::new();

    for result in validations {
        if let Some(msg) = result {
            errors.push(msg.clone());
        }
    }

    if !errors.is_empty() {
        let combined_msg = errors.join("; ");
        return Err(ParserErrorKind::NameDuplicated(combined_msg).into());
    }
    Ok(())
}

pub(crate) fn validate_sturcture(sturct_vec:Vec<GlobalDefinition>) -> AppResult<()> {
    

    let mut table_vec = Vec::<&Table>::new();
    let mut enum_vec = Vec::<&ColumnEnum>::new();
    let mut relation_vec = Vec::<&Relation>::new();

    for item in &sturct_vec {
        match item {
            GlobalDefinition::Table(t) => table_vec.push(t),
            GlobalDefinition::Enum(e) => enum_vec.push(e),
            GlobalDefinition::Relation(r) => relation_vec.push(r),
        }
    }

    
    let validations: Vec<Option<String>> = vec![
        validate_and_colloct_duplicate_names(&table_vec, "table_name"),
        validate_and_colloct_duplicate_names(&enum_vec, "enum_name"),
        validate_and_colloct_duplicate_names(&relation_vec, "relation_name"),
    ];

    validate_duplicate_name_all(validations)?;
   

    table_vec
        .iter()
        .try_for_each(|e| validate_table(e))?;

    enum_vec
        .iter()
        .try_for_each(|e| validate_enum(e))?;

    relation_vec
        .iter()
        .try_for_each(|e| validate_relation(e))?;

    Ok(())
}
