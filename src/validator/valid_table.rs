use std::collections::HashSet;

use crate::db_type::table::{Column, Table};
use crate::db_type::AttrEnum;
use crate::error_enum::schema_error::SchemaErrorKind;
use crate::error_enum::AppResult;
use crate::validator::schema_table::{SchemaTable, SchemaTableRegex};
use crate::validator::valid_schema::validate_field_from_regex;

fn validate_column_attr(
    attr:&AttrEnum,schema_attr:&HashSet<String>
    ) -> AppResult<()> {
    fn is_contain(
        s:&String,
        check_vec:&HashSet<String>) -> AppResult<()> {
        check_vec.contains(s)
            .then_some(())
            .ok_or(
                SchemaErrorKind::NoContained { 
                    colum_type: format!("Table column attr {}",s)
                }.into())
    }
    match attr {
        AttrEnum::Sigle(e) => is_contain(e, schema_attr),
        //todo Need to complement the corresponding validation.
        AttrEnum::KeyValue(k, _v) => is_contain(k, schema_attr),
    }
}

fn validate_table_column(
    column:&Column,
    schema:&SchemaTableRegex,
    enum_names:&[String]
    ) -> AppResult<()> {
    validate_field_from_regex(
            &schema.allow_column_name,
            &column.name, 
            "Table column name".to_string())?;

    let column_attr = column.field_type.name.to_uppercase();

    let attr_res = schema.allow_type.contains(&column_attr) 
        || enum_names.contains(&column_attr);

    if !attr_res {
        let msg = format!("Column type on {},by {}",column.name,&column_attr);
        return Err(SchemaErrorKind::NoContained { colum_type: msg }.into());
    }

    if let Some(attr) = &column.attrs {
        attr.iter().try_for_each(|item| validate_column_attr(item, &schema.allow_column_attr))?;
    }
    Ok(())
}

fn validate_table_name(
    table:&Table,
    schema:&SchemaTableRegex,
    enum_names:&[String]
    ) -> AppResult<()> {
    validate_field_from_regex(
            &schema.allow_name, 
            &table.name, 
            "table name".to_string()
        )?;

    table.columns.iter().try_for_each(
        |column| 
        validate_table_column(&column, schema, enum_names)
    )?;
    Ok(())
}

pub(crate) fn validate_table(
    table:&Vec<&Table>,
    schema_table:SchemaTable,
    enum_names:&[String]
    ) -> AppResult<()> {
    let vaildtor = SchemaTableRegex::try_from(schema_table)?;
    table.iter().try_for_each(
        |&item| validate_table_name(item, &vaildtor,enum_names)
    )?;
    Ok(())
}
