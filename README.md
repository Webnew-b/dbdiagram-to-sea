# DBML to SeaORM Migration Generator (db-diagram-to-sea-orm)

[![Build Status](https://img.shields.io/github/actions/workflow/status/Webnew-b/dbdiagram-to-sea/rust.yml?branch=main)](https://github.com/Webnew-b/dbdiagram-to-sea/actions)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A command-line tool for automatically converting [DBML (Database Markup Language)](https://dbml.org/) files into migration code for [Rust SeaORM](https://www.sea-ql.org/SeaORM/). This tool aims to streamline the database schema design and synchronization process by generating boilerplate migration code directly from your DBML definitions.

---

## ⚠️ Important Notice: Preview Version 0.8

This project is currently in a **v0.8-preview** stage. It is intended for **research and testing purposes only**.

**DO NOT USE IT IN A PRODUCTION ENVIRONMENT.**

The generated code and features are subject to change, and there are several known limitations. The current implementation exclusively generates migration syntax for **PostgreSQL**.

### Current Status (v0.8-preview)

#### ✅ Supported Features
* **Enums**: Generates `CREATE TYPE ... AS ENUM` statements for PostgreSQL.
* **Tables**: Generates `CREATE TABLE` statements with basic column definitions.
* **Foreign Keys**: Generates `ADD CONSTRAINT ... FOREIGN KEY` for one-to-one and one-to-many relationships.

#### ❌ Known Limitations & Bugs
1.  **Many-to-Many Relationships**: Not supported. The tool cannot process `many-to-many` references.
2.  **Index Creation**: Not supported. The `indexes` block in DBML is ignored.
3.  **JSONB Type**: The `JSONB` data type is not recognized and will not be correctly generated.
4.  **Array Types**: Array syntax (e.g., `varchar[]`) is not supported.
5.  **Schema**: Schema syntax (e.g., `shcema.table.column`) is not supported.
6.  **Default Values**: The conversion of `default` value settings is not fully implemented and may not work for all cases.
7.  **Basic Syntax Only**: The tool only supports basic syntax conversion; advanced DBML features are not yet implemented.

⚡ Quick Start

This section demonstrates how to convert a simple DBML definition into SeaORM migration code.
1. Input

First, create a file named schema.dbml and define your data table.

schema.dbml:
``` dbml
Table user {
  id BIGSERIAL [pk]
  username varchar(255) [not null]
  email varchar(255) [not null]
  avatar varchar(255)
  gender user_gender [default: 'prefer_not_to_say']
  introduce varchar(300)
  is_show bool [default: false]
  is_delete bool [default: false]
  status user_status [default: 'active']
  createTime timestamp [default: now()]
  updateTime timestamp [note: "Update Time"]
}
```

2. Output

Next, run the generator command:
```bash
db-diagram-to-sea-orm ./schema.dbml --output ./migration/src/
```
The tool will generate a new migration file (e.g., m20250724_000000_init.rs) in the specified output directory with the following content:

`migration/src/m<timestamp>_user.rs`:
``` rust
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(r#"CREATE TABLE IF NOT EXISTS "user" (
id BIGSERIAL PRIMARY KEY,
username varchar(255) NOT NULL,
email varchar(255) NOT NULL,
avatar varchar(255),
gender user_gender DEFAULT 'prefer_not_to_say',
introduce varchar(300),
is_show bool DEFAULT false,
is_delete bool DEFAULT false,
status user_status DEFAULT 'active',
createTime timestamp DEFAULT now(),
updateTime timestamp
);"#).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"DROP TABLE IF EXISTS "user""#).await?;
        Ok(())
    }
}
```

## Supported Type
### Table
``` dbml
Table table_name {
  column_name column_type [column_settings]
}
```
### Enum
``` dbml
enum enum_name {
    enum_item_a
    enum_item_b
    enum_item_c
}
```
### Relation
``` dbml
Ref: table1.column1 < table2.column2
```
## 🚀 Installation

You can install `db-diagram-to-sea-orm` in the following ways:

### 1. From GitHub Releases
Pre-compiled binaries are available for download from the project's [GitHub Releases](https://github.com/YOUR_USERNAME/YOUR_REPO/releases) page.

### 2. From Source
```bash
git clone [https://github.com/YOUR_USERNAME/YOUR_REPO.git](https://github.com/YOUR_USERNAME/YOUR_REPO.git)
cd YOUR_REPO
cargo build --release
# The executable will be available at ./target/release/db-diagram-to-sea-orm
```

## 💡 Usage Workflow

1.  **Create a DBML File**: Author a `schema.dbml` file defining your PostgreSQL database schema. Focus on using the currently supported features: tables, columns, enums, and foreign key relations (`Ref`).

2.  **Run the Generator**: Execute the command-line tool, providing the path to your input DBML file and the desired output directory for your SeaORM migrations.
    ```bash
    db-diagram-to-sea-orm ./schema.dbml --output ./migration/src/
    ```

3.  **Review the Output**: The tool will generate a new migration file (e.g., `m20250724_000000_init.rs`) in the specified directory. This file will contain the `up` and `down` methods with the SeaORM migration logic corresponding to your DBML schema. Always review the generated code carefully before use.

## ⚙️ Command-Line Options

You can view all available options by running the tool with the `--help` flag.

```bash
DBML to SeaORM Migration Generator

Usage: db-diagram-to-sea-orm [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Path to the input DBML file

Options:
  -o, --output <OUTPUT>  Directory for the output SeaORM migration
  -h, --help             Print help
  -V, --version          Print version
```

## 🗺️ Roadmap

The immediate focus is on addressing the known limitations and stabilizing the tool for a production-ready release. Key priorities include:
- [ ] Add support for Index creation.
- [ ] Add support for `JSONB` and other common PostgreSQL types.
- [ ] Implement robust support for Array types.
- [ ] Fully support all `default` value variations.
- [ ] Implement support for Many-to-Many relationships.
- [ ] Improve error reporting and diagnostics.

## 🤝 Contributing

Contributions are welcome! If you would like to help improve this tool, please feel free to fork the repository, make your changes, and submit a pull request. For bugs and feature requests, please open an issue on the GitHub repository.

## 📜 License

This project is licensed under the [Apache License 2.0](https://opensource.org/licenses/Apache-2.0).
