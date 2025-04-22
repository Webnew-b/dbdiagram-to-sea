use core::fmt;

use crate::db_type::AttrEnum;

#[derive(Debug)]
pub struct Table {
    pub alias:Option<String>,
    pub name:String,
    pub columns:Vec<Column>,
    pub note:Option<String>
}

#[derive(Debug)]
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
