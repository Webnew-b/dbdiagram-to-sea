
use crate::db_type::relation::Relation;
use crate::error_enum::AppResult;
use crate::validtor::schema_relation::SchemaRelation;



pub(crate) fn validate_relation(
    relation:Vec<&Relation>,
    schema_relation:SchemaRelation,
    table_names:&[String]
    ) -> AppResult<()> {
    todo!();
    Ok(())
}
