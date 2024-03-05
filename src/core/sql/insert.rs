use std::sync::Arc;
use log::debug;
use crate::config::Config;

use crate::core::model::{Model, TableName};

pub struct Insert {
    config:Config,
    lazy_sql: String,
    table_name: String,
    fields: Vec<String>,
    values: Vec<String>,
}

impl Insert {
    pub fn default() -> Insert {
        Insert {
            config: Config::default(),
            lazy_sql: String::from("INSERT INTO {tableName} ({fields}) VALUES ({values})"),
            table_name: String::new(),
            fields: vec![],
            values: vec![],
        }
    }
    pub fn info(config:Config)->Insert{
        Insert{
            config,
            lazy_sql: String::from("INSERT INTO {tableName} ({fields}) VALUES ({values})"),
            table_name: String::new(),
            fields: vec![],
            values: vec![],
        }
    }

    pub fn table(mut self, model: Arc<dyn TableName>) -> Self {
        self.table_name = model.get_table_name();
        self
    }
    pub fn table_str(mut self, model: &str) -> Self {
        self.table_name = model.to_string();
        self
    }

    /// 传入字段名称，需依照顺序
    pub fn set_fields_for_vec(mut self, fields: Vec<String>) -> Self {
        let fields = fields.into_iter()
            .map(|s| format!("'{}'", s))
            .collect();
        self.fields = fields;
        self
    }
    pub fn set_fields(mut self, model: Arc<dyn Model>) -> Self {
        let fields = model.get_fields();
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
