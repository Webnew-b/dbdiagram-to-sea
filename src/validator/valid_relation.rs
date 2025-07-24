
use std::collections::HashMap;

use crate::db_type::relation::Relation;
use crate::db_type::table::Column;
use crate::error_enum::schema_error::SchemaErrorKind;
use crate::error_enum::AppResult;
use crate::validator::schema_relation::{SchemaRelation, SchemaRelationRegex};
use crate::validator::valid_schema::validate_field_from_regex;

fn validate_relation_item(
    relation:&Relation,
    schema:&SchemaRelationRegex,
    table_names:&[String],
    column_map:&HashMap<String,&Vec<Column>>
) -> AppResult<()>{
    validate_field_from_regex(
        &schema.allow_name_regex, 
        format!("{}_with_{}",relation.from_table,relation.from_column).as_str(), 
        "relation name".to_string())?;

    if &relation.from_schema != &relation.to_schema {
        let ret_from_schema = if let Some(s) = &relation.from_schema {
            s.clone()
        } else {
            "None".to_string()
        };

        let ret_to_schema = if let Some(s) = &relation.to_schema {
            s.clone()
        } else {
            "None".to_string()
        };
        return Err(SchemaErrorKind::RelationSchemaNotEq { 
            from: ret_from_schema, to: ret_to_schema 
        }.into());
    }
    
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
