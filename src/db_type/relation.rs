use std::fmt;

use crate::db_type::HashName;

#[derive(Debug)]
pub struct Relation{
    pub name:String,
    pub from_schema:Option<String>,
    pub from_table:String,
    pub from_column:String,
    pub to_schema:Option<String>,
    pub to_table:String,
    pub to_column:String,
    pub relation:RelationEnum,
}

#[derive(Debug)]
pub enum RelationEnum {
    OneToMany,
    OneToOne,
    ManyToOne,
    ManyToMany,
}

pub fn get_relation_from_str(input:&str) -> Option<RelationEnum> {
    match input {
        ">" => Some(RelationEnum::ManyToOne),
        "-" => Some(RelationEnum::OneToOne),
        "<" => Some(RelationEnum::OneToMany),
        "<>" => Some(RelationEnum::ManyToMany),
        _ => None
    }
}

impl HashName for Relation {
    fn get_name(&self)-> String {
        self.name.clone()
    }
}

impl fmt::Display for RelationEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelationEnum::ManyToMany => write!(f,"Many to many"),
            RelationEnum::OneToOne => write!(f,"One to one"),
            RelationEnum::OneToMany => write!(f,"One to many"),
            RelationEnum::ManyToOne => write!(f,"Many to one"),
        }
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Relation: name:{},from:{}.{},to:{}.{},relation_type:{}",
            self.name,self.from_table,self.from_column,self.to_table,self.to_column,self.relation
        )
    }
}
