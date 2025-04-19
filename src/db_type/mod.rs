use std::fmt;

#[derive(Debug)]
pub enum GlobalDefinition {
    Table(Table),
    Enum(ColumnEnum)
}

#[derive(Debug)]
pub enum AttrEnum {
    Sigle(String),
    KeyValue(String,String),
}


#[derive(Debug)]
pub struct Table {
    pub alias:String,
    pub name:String,
    pub columns:Vec<Column>
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
            "Table: \n alias:{}\n name:{} \n columns:{:?}",
            self.alias,self.name,self.columns
        )
    }
}
