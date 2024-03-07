use std::collections::HashMap;

use crate::config::Config;
use crate::core::model::Entity;

pub struct Select<'e> {
    config: Config,
    lazy_sql: String,
    table_name: String,
    fields: Vec<String>,
    values: Vec<String>,
    entity: &'e dyn Entity,
    model: Vec<String>,
    eq: HashMap<String, String>,
}

impl Select {
    pub fn default(entity: &dyn Entity) -> Select {
        Select::info(Config::default(), entity)
    }
    pub fn info(config: Config, entity: &dyn Entity) -> Select {
        Select {
            config,
            lazy_sql: String::from("SELECT {fields} FROM {tableName} "),
            table_name: entity.get_table_name(),
            fields: entity.get_fields(),
            values: vec![],
            entity,
            model: vec![],
            eq: HashMap::new(),
        }
    }

    pub fn field(mut self, field: Vec<String>) -> Self {
        self.fields = field;
        self
    }

    pub fn eq<T>(mut self, field: String, value: T) -> Self {
        self.eq.insert(field, value.to_String());
        self
    }

    pub fn eq_map<T>(mut self, data: HashMap<String, T>) -> self {
        for (key, value) in data {
            self.eq.insert(key, value.to_String());
        }
        self
    }

    pub fn build(mut self) -> String {
        let field = self.fields.join(",");
        let sql = String::from(self.lazy_sql.as_str());
        let sql = sql.replace("{tableName}", &self.table_name);
        let sql = sql.replace("{fields}", field.as_str());
    }
}