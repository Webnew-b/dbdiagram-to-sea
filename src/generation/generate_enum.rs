use crate::db_type::column_enum::ColumnEnum;
use crate::generation::Migration;

fn create_up_sql(e:&ColumnEnum) -> String {
    let mut enum_item_sql = e.item.iter()
        .fold("(".to_string(), |mut acc,item|{ 
            acc.push_str("\"");
            acc.push_str(item.name.as_str());
            acc.push_str("\"");
            acc.push_str(",");
            acc
        });
    enum_item_sql.pop();
    enum_item_sql.push_str(")");

    let res = format!("CREATE TYPE {} AS ENUM {};",e.name,enum_item_sql);
    res
}

fn create_down_sql(name:&str) -> String {
    format!("DROP TYPE IF EXISTS {};",name)
}

pub(crate) fn generate_enums_sql(enums:Vec<ColumnEnum>) 
    -> Vec<Migration> {
    let mut sqls = vec![];
    enums.into_iter()
        .for_each(|e|{
            let up = create_up_sql(&e);
            let down = create_down_sql(e.name.as_str());
            sqls.push(Migration{
                up,
                down,
                name:e.name
            })
        });
    sqls
}
