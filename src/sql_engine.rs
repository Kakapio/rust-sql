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
    UNRECOGNIZED
}

#[derive(PartialEq, Debug, Default)]
pub enum StatementType {
    INSERT,
    #[default]
    SELECT
}

#[derive(PartialEq, Debug, Default)]
pub struct Statement {
    pub cmd: StatementType
}