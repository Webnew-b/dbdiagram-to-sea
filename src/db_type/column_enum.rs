use core::fmt;

use crate::db_type::HashName;

#[derive(Debug)]
pub struct ColumnEnum {
    pub name:String,
    pub item:Vec<ColumnEnumItem>
}

#[derive(Debug)]
pub struct ColumnEnumItem {
    pub name:String,
    pub attrs:Option<Vec<String>>
}

impl HashName for ColumnEnum {
    fn get_name(&self)-> String {
        self.name.clone()
    }
}


impl fmt::Display for ColumnEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Enum: \n name:{} \n item:[{:?}]",
            self.name,self.item
        )
    }
}

impl fmt::Display for ColumnEnumItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name:{} \n attrs:{:?}",
            self.name,self.attrs
        )
    }
}


