use crate::config::config_enum::SqlType;

mod config_enum;

/// sqlx-pro 配置
#[derive(Debug)]
pub struct Config {
    /// sql生成日志
    pub sql_log: bool,
    /// 文件读取形式
    pub sql_type: SqlType,
}
impl Config {
    pub fn default() -> Config {
        Config {
            sql_log: false,
            sql_type: SqlType::Auto,
        }
    }

    pub fn new(sql_log: bool, sql_type: SqlType) -> Config {
        Config { sql_log, sql_type }
    }
}
