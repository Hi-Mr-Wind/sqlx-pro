use std::collections::HashMap;
use crate::config::Config;
use crate::core::model::Entity;
use crate::core::sql::select::Select;

pub struct Delete<'e> {
    config: Config,
    lazy_sql: String,
    table_name: String,
    fields: Vec<String>,
    values: Vec<String>,
    entity: &'e dyn Entity,
    model: Vec<String>,
    eq: HashMap<String, String>,
}

impl Delete {
    pub fn default(entity: &dyn Entity) -> Delete {
        Delete::info(Config::default(), entity)
    }
    pub fn info(config: Config, entity: &dyn Entity) -> Delete {
        Delete {
            config,
            lazy_sql: String::from("DELETE FROM {tableName}"),
            table_name: entity.get_table_name(),
            fields: entity.get_fields(),
            values: vec![],
            entity,
            model: vec![],
            eq: HashMap::new(),
        }
    }
}