#[derive(PartialEq, Debug, Default)]
pub enum MetaCommandResult {
    SUCCESS,
    #[default]
    UNRECOGNIZED
}

#[derive(PartialEq, Debug, Default)]
pub enum PrepareResult {
    SUCCESS,
    #[default]
    UNRECOGNIZED,
    SYNTAX_ERROR
}

#[derive(PartialEq, Debug, Default)]
pub enum StatementType {
    INSERT,
    #[default]
    SELECT
}

#[derive(PartialEq, Debug, Default)]
pub struct Statement {
    pub cmd: StatementType,
    pub row_to_insert: Row
}

#[derive(PartialEq, Debug, Default)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String
}