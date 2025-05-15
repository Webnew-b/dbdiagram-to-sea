use crate::db_type::column_enum::ColumnEnum;
use crate::error_enum::AppResult;
use crate::validtor::schema_enum::SchemaEnum;



pub(crate) fn validate_enum(
    column_enum:Vec<&ColumnEnum>,
    schema_enum:SchemaEnum,
    table_names:&[String]
    ) -> AppResult<()> {
    todo!();
    Ok(())
}
