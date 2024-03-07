pub trait Model: Entity {
    fn get_fields(&self) -> Vec<String>;
}

pub trait TableName: Entity {
    fn get_table_name(&self) -> String;
}

pub trait Entity {
    fn get_fields(&self) -> Vec<String>;
    fn get_table_name(&self) -> String;
}