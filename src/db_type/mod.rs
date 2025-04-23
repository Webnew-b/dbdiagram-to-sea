use crate::db_type::column_enum::ColumnEnum;
use crate::db_type::relation::Relation;
use crate::db_type::table::Table;

pub mod column_enum;
pub mod table;
pub mod relation;

#[derive(Debug)]
pub enum GlobalDefinition {
    Table(Table),
    Enum(ColumnEnum),
    Relation(Relation)
}

#[derive(Debug)]
pub enum AttrEnum {
    Sigle(String),
    KeyValue(String,String),
}


pub(super) trait HashName {
    fn get_name(&self)-> String;
} 
