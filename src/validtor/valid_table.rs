use crate::db_type::table::Table;
use crate::error_enum::AppResult;
use crate::validtor::schema_table::SchemaTable;


pub(crate) fn validate_table(
    table:Vec<&Table>,
    schema_table:SchemaTable,
    enum_names:&[String]
    ) -> AppResult<()> {
    todo!();
    Ok(())
}
