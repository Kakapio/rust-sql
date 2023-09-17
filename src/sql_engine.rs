pub enum MetaCommandResult {
    SUCCESS,
    UNRECOGNIZED
}

pub enum PrepareResult {
    SUCCESS,
    UNRECOGNIZED
}

pub enum StatementType {
    INSERT,
    SELECT
}

pub struct Statement {
    pub cmd: StatementType
}