use crate::db_type::table::{Column, FieldType, Table};
use crate::db_type::AttrEnum;
use crate::generation::Migration;

fn match_col_attr_sigle(s:&str) -> String {
    match s {
        "pk" => "PRIMARY KEY".to_string(),
        "unique" | "increment" | "not null" => s.to_uppercase(),
        "null" | _ => String::new(),
    }
}

fn match_col_attr_kv(k:&str,v:&str) -> String {
    //todo check all KeyValue of column attrible
    match k {
        "default" => format!("{} '{}'",k,v),
        "note" | _ => String::new(),
    }
}

fn create_col_attr_sql(attrs:&[AttrEnum]) -> String {
    attrs.iter()
        .fold(String::new(), |acc,a|{
            let res = match a {
                AttrEnum::Sigle(s) => match_col_attr_sigle(s),
                AttrEnum::KeyValue(k, v) => match_col_attr_kv(k, v),
            };
            format!("{} {}",acc,res)
        })
}

fn create_col_type_sql(ft:&FieldType)->String {
    match ft.amount.clone() {
        Some(a) => {
            format!("{}({})",ft.name,a)
        },
        None => format!("{}",ft.name),
    }
}

fn create_col_sql(column:&Column) -> String {
    let col_type = create_col_type_sql(&column.field_type);
    let res = match column.attrs.as_ref() {
        Some(a) => {
            let col_attr = create_col_attr_sql(a);
            format!("{} {} {}",column.name,col_type,col_attr)
        },
        None => format!("{} {}",column.name,col_type),
    };
    res.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn create_up_sql(table:&Table) -> String {
    let col_sqls: Vec<String> = table.columns.iter()
        .map(create_col_sql)
        .collect();

    let columns_str = col_sqls.join(",\n");

    format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" (\n{}\n);",
        table.name,
        columns_str
    )
}

fn create_down_sql(table:&Table) -> String {
    format!("DROP TABLE IF EXISTS '{}'",table.name)
}

pub(crate) fn generation_table_sqls(tables:Vec<Table>)
    -> Vec<Migration>
{
    tables.iter()
        .map(|t|{
          let name = format!("create_{}_table",t.name);
          Migration { 
              name, 
              up: create_up_sql(t), 
              down: create_down_sql(t)
          }
        })
        .collect()
}
