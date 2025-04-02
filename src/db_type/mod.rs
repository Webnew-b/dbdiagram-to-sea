use std::fmt;

#[derive(Debug)]
pub struct Table {
    pub name:String,
    pub columns:Vec<Column>
}

#[derive(Debug)]
pub struct Column{
    pub name:String,
    pub field_type:String,
    pub attrs:Option<Vec<String>>
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Column: \n name:{} \n type:{} \n attrs:{:?}",
            self.name,self.field_type,self.attrs
        )
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Table: \n name:{} \n columns:{:?}",
            self.name,self.columns
        )
    }
}
