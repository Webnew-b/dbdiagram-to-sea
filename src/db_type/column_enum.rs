use core::fmt;

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


