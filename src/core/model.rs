pub trait Model {
    fn get_fields(&self) -> Vec<String>;
}

pub trait TableName {
    fn get_table_name(&self) -> String;
}
