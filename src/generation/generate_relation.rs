use crate::db_type::relation::{Relation, RelationEnum};
use crate::generation::Migration;

fn create_up_sql(r:&Relation,name:&str) -> String {
    match r.relation {
        RelationEnum::ManyToOne | RelationEnum::OneToOne =>{ 
            format!(r#"
        ALTER TABLE {}
        ADD CONSTRAINT {}
        FOREIGN KEY ({}) 
        REFERENCES {} ({});
        "#,
            r.to_table,
            name,
            r.to_column,
            r.from_table,
            r.from_column
        )},

        RelationEnum::OneToMany => {
        format!(r#"
        ALTER TABLE {}
        ADD CONSTRAINT {}
        FOREIGN KEY ({}) 
        REFERENCES {} ({});
        "#,
            r.from_table,
            name,
            r.from_column,
            r.to_table,
            r.to_column
        )
        },
        RelationEnum::ManyToMany => {
            //todo implement mutiple relation
            String::new()
        },
    }
    
}

fn create_down_sql(r:&Relation,name:&str) -> String {
    match r.relation {
        RelationEnum::OneToMany => 
            format!("ALTER TABLE {} DROP CONSTRAINT {};",r.from_table,name),
        RelationEnum::OneToOne => 
            format!("ALTER TABLE {} DROP CONSTRAINT {};",r.to_table,name),
        RelationEnum::ManyToOne => 
            format!("ALTER TABLE {} DROP CONSTRAINT {};",r.to_table,name),
        RelationEnum::ManyToMany => 
            String::new(),
    }
}

fn create_fk_name(r:&Relation)->String {
    match r.relation {
        RelationEnum::OneToMany => format!("fk_{}_to_{}",r.from_table,r.to_table),
        RelationEnum::OneToOne => format!("fk_{}_to_{}",r.from_table,r.to_table),
        RelationEnum::ManyToOne => format!("fk_{}_to_{}",r.to_table,r.from_table),
        RelationEnum::ManyToMany => format!("fk_{}_to_{}_many",r.from_table,r.to_table),
    }
}

pub(crate) fn generate_relation_sqls(relation:Vec<Relation>) 
    -> Vec<Migration> 
{
    relation.iter()
        .map(|r|{
            let name = create_fk_name(r);
            Migration{
                name:name.clone(),
                up:create_up_sql(r, name.as_str()),
                down:create_down_sql(r,name.as_str())
            }
        })
        .collect()
}
