use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::db_type::column_enum::ColumnEnum;
use crate::db_type::relation::Relation;
use crate::db_type::table::{Column, Table};
use crate::db_type::{GlobalDefinition, HashName};
use crate::error_enum::{AppResult, ParserErrorKind};
use crate::validator::valid_schema::read_config;
use crate::validator::valid_enum::validate_enum;
use crate::validator::valid_relation::validate_relation;
use crate::validator::valid_table::validate_table;

pub mod valid_schema;
pub(crate) mod schema_enum;
pub(crate) mod schema_table;
pub(crate) mod schema_relation;
pub(crate) mod valid_table;
pub(crate) mod valid_enum;
pub(crate) mod valid_relation;


fn validate_and_collect_duplicate_names<T:HashName>(
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

fn validate_columns_name(columns: &[Column]) -> AppResult<()> {
    let mut seen_names = HashSet::new();
    
    if let Some(duplicate_column) = 
        columns.iter()
            .find(|column| !seen_names.insert(&column.name)) 
    {
        //Return parse error when it find the duplication
        let item = format!("Column field {} is duplicated",duplicate_column.name);
        Err(ParserErrorKind::NameDuplicated(item).into())
    } else {
        // Return Ok when it doesn't find the dupliaction. 
        Ok(())
    }
}

fn construct_column_valid_name<'a>(
    map:&mut HashMap<String,&'a Vec<Column>>,
    table:&'a Table
    ) -> AppResult<()> {
    validate_columns_name(&table.columns)?;
    map.entry(table.name.to_string()).or_insert(&table.columns);
    Ok(())
}

//
fn get_column_from_table<'table>(
    tables:&'table [&'table Table]
) -> AppResult<HashMap<String,&'table Vec<Column>>> {
    let mut map:HashMap<String,&Vec<Column>> = HashMap::new();
    tables.iter()
        .try_for_each(|t| construct_column_valid_name(&mut map, t))?;
    Ok(map)
}

pub(crate) fn validate_sturcture(
    sturct_vec:&Vec<GlobalDefinition>,
    schema_config:&Path,
) -> AppResult<()> {

    // load configuration for schema
    let schema_config = read_config(schema_config)?;

    // Initial validate sturcture
    let mut table_vec = Vec::<&Table>::new();
    let mut enum_vec = Vec::<&ColumnEnum>::new();
    let mut relation_vec = Vec::<&Relation>::new();

    // Obtain sturcture from parse result
    for item in sturct_vec {
        match item {
            GlobalDefinition::Table(t) => table_vec.push(t),
            GlobalDefinition::Enum(e) => enum_vec.push(e),
            GlobalDefinition::Relation(r) => relation_vec.push(r),
        }
    }
    
    let map = get_column_from_table(&table_vec)?;
    
    // Check if the name is dupliacated
    let validations: Vec<Option<String>> = vec![
        validate_and_collect_duplicate_names(&table_vec, "table_name"),
        validate_and_collect_duplicate_names(&enum_vec, "enum_name"),
    ];

    validate_duplicate_name_all(validations)?;

    // Get the parse item name from the validate sturcture
    let table_name_collection = get_collection_name(&table_vec);
    let enum_name_collection = get_collection_name(&enum_vec);

    // Check if the sturcture is correct.
    validate_table(&table_vec, schema_config.table,&enum_name_collection)?;
    validate_enum(enum_vec, schema_config.column_enum)?;
    validate_relation(relation_vec, schema_config.relation,&table_name_collection,&map)?;
    Ok(())
}
