use std::sync::Arc;
use log::{debug, info};
use crate::config::Config;

use crate::core::model::{Entity, Model, TableName};

pub struct Insert<'e> {
    config:Config,
    lazy_sql: String,
    table_name: String,
    fields: Vec<String>,
    values: Vec<String>,
    entity: &'e dyn Entity,
}

impl Insert {
    pub fn default(entity: &dyn Entity) -> Insert {
        Insert::info(Config::default(),entity)
    }
    pub fn info(config:Config,entity: &dyn Entity)->Insert{
        Insert{
            config,
            lazy_sql: String::from("INSERT INTO {tableName} ({fields}) VALUES ({values})"),
            table_name: entity.get_table_name(),
            fields: entity.get_fields(),
            values: vec![],
            entity
        }
    }

    pub fn table_str(mut self, model: &str) -> Self {
        self.table_name = model.to_string();
        self
    }

    /// 传入字段名称，需依照顺序
    pub fn set_fields(mut self, fields: Vec<String>) -> Self {
        let fields = fields.into_iter()
            .map(|s| format!("'{}'", s))
            .collect();
        self.fields = fields;
        self
    }

    /// 要插入数据库的值
    pub fn set_values(mut self, values: Vec<String>) -> Self {
        let values = values.into_iter()
            .map(|s| format!("'{}'", s))
            .collect();
        self.values = values;
        self
    }

    pub fn build(mut self) -> String {
        let field = self.fields.join(",");
        let sql = String::from(self.lazy_sql.as_str());
        let sql = sql.replace("{tableName}", &self.table_name);
        let sql = sql.replace("{fields}", field.as_str());
        let values = self.values.join(",");
        let binding = sql.replace("{values}", values.as_str());
        let sql = binding.as_str();
        if self.config.sql_log {
            debug!("{}",sql)
        }
        String::from(sql)
    }
}
