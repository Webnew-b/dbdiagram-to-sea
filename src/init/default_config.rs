
pub(crate) struct FileDefaultConfig {
    generate_config:&'static str,
    schema_config:&'static str,
    migrate_template:&'static str,
}

impl FileDefaultConfig {
    pub fn new() -> FileDefaultConfig {
        Self {
        generate_config:
r#"
[default_value]

needs_quotes = [
  "text",
  "varchar",
  "character varying",
  "char",
  "character",

  "timestamp",
  "timestamp without time zone",
  "timestamptz",
  "timestamp with time zone",
  "date",
  "time",
  "time without time zone",
  "timetz",
  "time with time zone",
  "interval",

  "json",
  "jsonb",

  "uuid",

  "inet",
  "cidr",
  "macaddr",
  "macaddr8",

  "xml",

  "enum",      
  "point",     
  "line",
  "lseg",
  "box",
  "path",
  "polygon",
  "circle",
  "array"      
]

no_quotes = [
  "integer",
  "int",
  "int4",
  "smallint",
  "int2",
  "bigint",
  "int8",
  "decimal",
  "numeric",
  "real",
  "float4",
  "double precision",
  "float8",

  "serial",
  "serial4",
  "smallserial",
  "serial2",
  "bigserial",
  "serial8",

  "boolean",
  "bool"
]
"#,
        schema_config:
r#"
[table]
allow_type=[
  "SMALLINT",
  "INTEGER",
  "INT",
  "BIGINT",
  "DECIMAL",

  "NUMERIC",

  "REAL",
  "DOUBLE PRECISION",
  "SMALLSERIAL",
  "SERIAL",
  "BIGSERIAL",

  "VARCHAR",
  "CHAR",

  "TEXT",

  "TIMESTAMP",

  "TIMESTAMP WITHOUT TIME ZONE",

  "TIMESTAMPTZ",

  "TIMESTAMP WITH TIME ZONE",
  "DATE",

  "TIME",

  "TIME WITHOUT TIME ZONE",

  "TIMETZ",

  "TIME WITH TIME ZONE",

  "INTERVAL",

  "BOOLEAN",
  "BOOL",
  "JSON",
  "JSONB",
  "UUID",
  "BYTEA",
  "INET"
]
allow_name="^[a-zA-Z_][a-zA-Z0-9_]*$"
allow_column_name="^[a-zA-Z_][a-zA-Z0-9_]*$"
allow_column_attr=[
  "pk",
  "primary key",
  "not null",
  "unique",
  "increment",
  "null",
  "default",
  "note",
]

[enum]
allow_name="^[a-zA-Z_][a-zA-Z0-9_]*$"
allow_column_name="^[a-zA-Z_][a-zA-Z0-9_]*$"

[relation]
allow_name="^[a-zA-Z_][a-zA-Z0-9_]*$"
"#,
            migrate_template:
r##"
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(r#"{{ up_sql }}"#).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"{{ down_sql }}"#).await?;
        Ok(())
    }
}
"##
        }
    }
    pub fn migrate_template(&self) -> &'static str {
        self.migrate_template
    }
    pub fn schema_config(&self) -> &'static str {
        self.schema_config
    }
    pub fn generate_config(&self) -> &'static str {
        self.generate_config
    }
}
