#[derive(Debug)]
pub enum SqlType {
    /// 自动生成
    Auto,
    /// xml读取
    XML(String),
    /// json读取
    JSON(String),
}
