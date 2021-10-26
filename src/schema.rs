use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schema {
    pub tables: HashMap<String, Table>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            tables: HashMap::new(),
        }
    }

    pub fn add_table(&mut self, table: Table) -> &mut Self {
        self.tables.insert(table.name.to_string(), table);
        self
    }

    pub fn find_table(&self, name: &str) -> anyhow::Result<&Table> {
        self.tables
            .get(name)
            .ok_or_else(|| anyhow!("no table {}", name))
    }

    pub fn find_table_mut(&mut self, name: &str) -> anyhow::Result<&mut Table> {
        self.tables
            .get_mut(name)
            .ok_or_else(|| anyhow!("no table {}", name))
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    #[serde(skip)]
    pub has_is_new: bool,
}

impl Table {
    pub fn new(name: impl Into<String>) -> Table {
        Table {
            name: name.into(),
            columns: vec![],
            has_is_new: false,
        }
    }

    pub fn add_column(&mut self, column: Column) -> &mut Self {
        self.columns.push(column);
        self
    }

    pub fn remove_column(&mut self, column_name: &str) -> &mut Self {
        if let Some(index) = self.columns.iter().position(|c| c.name == column_name) {
            self.columns.remove(index);
        }
        self
    }

    pub fn find_column(&self, name: &str) -> anyhow::Result<&Column> {
        self.columns
            .iter()
            .find(|column| column.name == name)
            .ok_or_else(|| anyhow!("no column {} on table {}", name, self.name))
    }

    pub fn find_column_mut(&mut self, name: &str) -> anyhow::Result<&mut Column> {
        self.columns
            .iter_mut()
            .find(|column| column.name == name)
            .ok_or(anyhow!("no column {} on table {}", name, self.name))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    pub name: String,
    #[serde(skip)]
    pub real_name: Option<String>,
    pub data_type: String,
    pub nullable: bool,
}

impl Column {
    pub fn real_name(&self) -> &str {
        self.real_name.as_ref().unwrap_or(&self.name)
    }

    //pub fn new(
    //    name: impl Into<String>,
    //    real_name: impl Into<String>,
    //    datatype: impl Into<String>,
    //    nullable: bool,
    //) -> Column {
    //    Column {
    //        name: name.into(),
    //        real_name: real_name.into(),
    //        datatype: datatype.into(),
    //        nullable,
    //    }
    //}
}