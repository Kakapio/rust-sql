#[derive(PartialEq, Debug)]
pub enum MetaCommandResult {
    SUCCESS,
    UNRECOGNIZED
}

#[derive(PartialEq, Debug)]
pub enum PrepareResult {
    SUCCESS,
    UNRECOGNIZED
}

#[derive(PartialEq, Debug)]
pub enum StatementType {
    INSERT,
    SELECT
}

#[derive(PartialEq, Debug)]
pub struct Statement {
    pub cmd: StatementType
}