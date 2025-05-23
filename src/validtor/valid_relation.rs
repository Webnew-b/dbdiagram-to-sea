
use crate::db_type::relation::Relation;
use crate::error_enum::schema_error::SchemaErrorKind;
use crate::error_enum::AppResult;
use crate::validtor::schema_relation::{SchemaRelation, SchemaRelationRegex};
use crate::validtor::vailtor_schema::validate_field_from_regex;

fn validate_relation_item(
    relation:&Relation,
    schema:&SchemaRelationRegex,
    table_names:&[String]
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

    // todo validate column.
    Ok(())
}

pub(crate) fn validate_relation(
    relation:Vec<&Relation>,
    schema_relation:SchemaRelation,
    table_names:&[String]
    ) -> AppResult<()> {

    let schema = SchemaRelationRegex::try_from(schema_relation)?;

    relation.iter()
        .try_for_each(
            |&item| validate_relation_item(item, &schema, &table_names)
        )?;

    Ok(())
}
