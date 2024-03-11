use std::collections::HashMap;
use std::ops::Add;

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
    wheres: bool,
    //等于
    eq: HashMap<String, String>,
    //大于
    gt: HashMap<String, String>,
    //小于
    lt: HashMap<String, String>,
    //不等于
    neq: HashMap<String, String>,
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
            wheres: false,
            eq: HashMap::new(),
            gt: Default::default(),
            lt: Default::default(),
            neq: Default::default(),
        }
    }

    pub fn field(mut self, field: Vec<String>) -> Self {
        self.fields = field;
        self
    }

    pub fn eq<T>(mut self, field: &str, value: T) -> Self {
        self.wheres = true;
        self.eq.insert(field.to_string(), value.to_String());
        self
    }

    pub fn eq_map<T>(mut self, data: HashMap<String, T>) -> self {
        self.wheres = true;
        for (key, value) in data {
            self.eq.insert(key, value.to_String());
        }
        self
    }

    pub fn build(mut self) -> String {
        let field = self.fields.join(",");
        let sql = String::from(self.lazy_sql.as_str());
        let sql = sql.replace("{tableName}", &self.table_name);
        let mut sql = sql.replace("{fields}", field.as_str());
        if self.wheres {
            if sql.contains("WHERE ") {} else {
                let mut sql = sql.add("WHERE ");
                let mut fields_value: Vec<String> = vec![];
                if !self.eq.is_empty() {
                    for (k, v) in self.eq {
                        let mut field = k.add("=");
                        let mut field = field.add(v.as_str());
                        let field = field.add("AND");
                        fields_value.push(field)
                    }
                }


            }
        }
        sql
    }
}