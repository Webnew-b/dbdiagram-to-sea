use std::fmt;

use crate::db_type::HashName;

#[derive(Debug)]
pub struct Relation{
    pub from_schema:Option<String>,
    pub from_table:String,
    pub from_column:String,
    pub to_schema:Option<String>,
    pub to_table:String,
    pub to_column:String,
    pub relation:RelationEnum,
    pub update_action:Option<ReferentialAction>,
    pub delete_action:Option<ReferentialAction>
}

#[derive(Debug,PartialEq, Eq,Clone)]
pub enum ReferentialAction {
    Cascade,
    NoAction,
    Restrict,
    SetNull,
    SetDefault,
}

#[derive(Debug)]
pub enum RelationEnum {
    OneToMany,
    OneToOne,
    ManyToOne,
    ManyToMany,
}

pub fn get_action_from_str(input:&str) -> Option<ReferentialAction> {
    match input {
        "cascade" => Some(ReferentialAction::Cascade),
        "no action" => Some(ReferentialAction::NoAction),
        "restrict" => Some(ReferentialAction::Restrict),
        "set null" => Some(ReferentialAction::SetNull),
        "set default" => Some(ReferentialAction::SetDefault),
        _ => None
    }
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
//todo Should be deleted.
impl HashName for Relation {
    fn get_name(&self)-> String {
        String::new()
    }
}

impl fmt::Display for ReferentialAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReferentialAction::Cascade => write!(f,"CASCADE"),
            ReferentialAction::NoAction => write!(f,"CASCADE"),
            ReferentialAction::Restrict => write!(f,"RESTRICT"),
            ReferentialAction::SetNull => write!(f,"SETNULL"),
            ReferentialAction::SetDefault => write!(f,"SETDEFAULT"),
        }
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
            "Relation: from:{}.{},to:{}.{},relation_type:{}",
            self.from_table,self.from_column,self.to_table,self.to_column,self.relation
        )
    }
}
