
use std::collections::HashMap;

use crate::db_type::relation::Relation;
use crate::db_type::table::Column;
use crate::error_enum::schema_error::SchemaErrorKind;
use crate::error_enum::AppResult;
use crate::validator::schema_relation::{SchemaRelation, SchemaRelationRegex};
use crate::validator::vailtor_schema::validate_field_from_regex;

fn validate_relation_item(
    relation:&Relation,
    schema:&SchemaRelationRegex,
    table_names:&[String],
    column_map:&HashMap<String,&Vec<Column>>
) -> AppResult<()>{
    validate_field_from_regex(
        &schema.allow_name_regex, 
        relation.name.as_str(), 
        "relation name".to_string())?;

    
    table_names.contains(&relation.from_table)
        .then_some(())
        .ok_or(
            SchemaErrorKind::NoContained { 
            colum_type: "relation with table name".to_string()
        })?;

    table_names.contains(&relation.to_table)
        .then_some(())
        .ok_or(
            SchemaErrorKind::NoContained { 
                colum_type: "relation with table name".to_string()
        })?;

    let from_table_column = column_map
        .get(&relation.from_table.clone())
        .ok_or(SchemaErrorKind::NoContained { 
                colum_type: "relation with table name".to_string()
            })?;

    let to_table_column = column_map
        .get(&relation.to_table.clone())
        .ok_or(SchemaErrorKind::NoContained { 
                colum_type: "relation with table name".to_string()
            })?;

    let from_column = from_table_column
        .iter()
        .find(|&c| &c.name == &relation.from_column)
        .ok_or(SchemaErrorKind::NoContained { 
            colum_type: "relation with table column name".to_string()
        })?;

    let to_column = to_table_column
        .iter()
        .find(|&c| &c.name == &relation.to_column)
        .ok_or(SchemaErrorKind::NoContained { 
            colum_type: "relation with table column name".to_string()
        })?;

    if from_column.field_type != to_column.field_type {
        return Err(SchemaErrorKind::RelationColumnNotEq { 
            from: from_column.name.to_string(), 
            to: to_column.name.to_string() 
        }.into());
    }
    // todo validate column.
    Ok(())
}



pub(crate) fn validate_relation(
    relation:Vec<&Relation>,
    schema_relation:SchemaRelation,
    table_names:&[String],
    column_map:&HashMap<String,&Vec<Column>>
    ) -> AppResult<()> {

    let schema = SchemaRelationRegex::try_from(schema_relation)?;
    let map = column_map;

    relation.iter()
        .try_for_each(
            |&item| validate_relation_item(item, &schema, &table_names,&map)
        )?;

    Ok(())
}
