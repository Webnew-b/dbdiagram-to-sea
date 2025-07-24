
use crate::db_type::column_enum::ColumnEnum;
use crate::error_enum::AppResult;
use crate::validator::schema_enum::{SchemaEnum, SchemaEnumRegex};
use crate::validator::valid_schema::validate_field_from_regex;


fn validate_enum_item(
    column_enum:&ColumnEnum,
    check_regex:&SchemaEnumRegex,
) -> AppResult<()>{
    validate_field_from_regex(
        &check_regex.allow_name_regex, 
        column_enum.name.as_str(), 
        "column enum name".to_string()
    )?;

    column_enum.item.iter()
        .try_for_each(
            |item| validate_field_from_regex(
                &check_regex.allow_column_name_regex,
                &item.name,
                "enum variant name".to_string()
            )
        )?;
    Ok(())
}

pub(crate) fn validate_enum(
    column_enums:Vec<&ColumnEnum>,
    schema_enum:SchemaEnum,
    ) -> AppResult<()> {

    let schema_enum = SchemaEnumRegex::try_from(schema_enum)?;

    column_enums.iter()
        .try_for_each(
            |&item| validate_enum_item(item, &schema_enum)
    )?;
    Ok(())
}
