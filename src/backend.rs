use crate::parser::Row;

#[derive(PartialEq, Debug, Default)]
pub enum ExecuteResult {
    SUCCESS,
    #[default]
    TABLE_FULL
}

pub struct Table {
    pub data: Vec<Row>
}

impl Table {
    pub fn new() -> Table {
        Table { data: Vec::new() }
    }
}