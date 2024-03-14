use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Add;

use crate::config::Config;
use crate::core::model::Entity;

pub struct Del<'e> {
    config: Config,
    lazy_sql: String,
    table_name: String,
    fields: Vec<String>,
    values: Vec<String>,
    entity: &'e dyn Entity,
    model: Vec<String>,
    wheres: bool,
    eq: HashMap<String, String>,
}

impl Del<'_> {
    pub fn default(entity: &dyn Entity) -> Del {
        Del::info(Config::default(), entity)
    }
    pub fn info(config: Config, entity: &dyn Entity) -> Del {
        Del {
            config,
            lazy_sql: String::from("DELETE FROM {tableName} "),
            table_name: entity.get_table_name(),
            fields: entity.get_fields(),
            values: vec![],
            entity,
            model: vec![],
            wheres: false,
            eq: HashMap::new(),
        }
    }

    pub fn eq<T: Display>(mut self, field: &str, value: T) -> Self {
        self.wheres = true;
        self.eq.insert(field.to_string(), value.to_string());
        self
    }

    pub fn eq_map<T: Display>(mut self, data: HashMap<String, T>) -> Self {
        self.wheres = true;
        for (key, value) in data {
            self.eq.insert(key, value.to_string());
        }
        self
    }
    pub fn build(mut self) -> String {
        let sql = String::from(self.lazy_sql.as_str());
        let sql = sql.replace("{tableName}", &self.table_name);
        if self.wheres {
            let mut fields_value: Vec<String> = vec![];
            if sql.contains("WHERE") {
                if !self.eq.is_empty() {
                    let mut wheres = build_where(self.eq);
                    fields_value.append(&mut wheres);
                }
                let wheres = fields_value.join(" AND ");
                sql.add(wheres.as_str())
            } else {
                let sql = sql.add("WHERE ");
                let mut wheres = build_where(self.eq);
                fields_value.append(&mut wheres);
                let wheres: String = fields_value.join(" AND ");
                sql.add(wheres.as_str())
            }
        } else {
            sql
        }
    }
}

fn build_where(eq: HashMap<String, String>) -> Vec<String> {
    let mut fields_value = vec![];
    for (k, v) in eq {
        let mut field = k.add("=");
        let field = field.add(v.as_str());
        fields_value.push(field)
    }
    fields_value
}