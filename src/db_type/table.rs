use core::fmt;

use crate::db_type::{AttrEnum, HashName};

#[derive(Debug,PartialEq, Eq)]
pub struct Table {
    pub alias:Option<String>,
    pub name:String,
    pub columns:Vec<Column>,
    pub note:Option<String>
}

#[derive(Debug,PartialEq, Eq)]
pub struct Column{
    pub name:String,
    pub field_type:FieldType,
    pub attrs:Option<Vec<AttrEnum>>
}

#[derive(Debug)]
pub struct FieldType {
    pub name:String,
    pub amount:Option<String>
}

impl PartialEq for FieldType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.amount == other.amount
    }
}

impl Eq for FieldType {
    
}


impl HashName for Table {
    fn get_name(&self)-> String {
        self.name.clone()
    }
}

impl HashName for Column {
    fn get_name(&self)-> String {
        self.name.clone()
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Column: \n name:{} \n type:{:?} \n attrs:{:?}",
            self.name,self.field_type,self.attrs
        )
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Table: \n alias:{:?}\n name:{} \n columns:{:?}",
            self.alias,self.name,self.columns
        )
    }
}
