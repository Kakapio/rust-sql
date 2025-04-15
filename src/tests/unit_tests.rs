#[cfg(test)]
use crate::{
    backend::execute_command,
    parser::{prepare_statement, MetaCommandResult, PrepareResult, Row, Statement, StatementType},
};

// Testing whether unrecognized commands are rejected.
#[test]
fn execute_command_unrecognized() {
    let cmd = ".dummy";
    let out = execute_command(cmd);
    assert_eq!(out, MetaCommandResult::Unrecognized);
}

// Testing whether the enum is set properly.
#[test]
fn prepare_statement_set_insert() {
    let mut out_statement = Statement::default();
    let cmd = "insert";
    prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_statement.cmd, StatementType::Insert);
}

// Testing whether the enum is set properly.
#[test]
fn prepare_statement_set_select() {
    let mut out_statement = Statement::default();
    let cmd = "select";
    prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_statement.cmd, StatementType::Select);
}

// Testing whether the output result is correct.
#[test]
fn prepare_statement_out_success() {
    let cmd = "insert 10 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut Statement::default());
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing whether the output result handles bad commands.
#[test]
fn prepare_statement_out_failure() {
    let cmd = "dummy";
    let out_result = prepare_statement(cmd, &mut Statement::default());
    assert_eq!(out_result, PrepareResult::Unrecognized);
}

// Testing whether the insert syntax error is handled.
#[test]
fn prepare_statement_out_syntax_error() {
    let mut out_statement = Statement::default();
    let cmd = "insert";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

// Testing whether the parsing works.
#[test]
fn prepare_statement_insert_parse() {
    let mut out_statement = Statement::default();
    let cmd = "insert 10 monkeylover ape@gmail.com";
    prepare_statement(cmd, &mut out_statement);
    assert_eq!(
        out_statement.row_instance,
        Some(Row {
            id: 10,
            username: "monkeylover".to_string(),
            email: "ape@gmail.com".to_string()
        })
    );
}

// Testing whether the parsing works by checking with incorrect data.
#[test]
fn prepare_statement_insert_parse_fail() {
    let mut out_statement = Statement::default();
    let cmd = "insert 10 monkeylover ape@gmail.com";
    prepare_statement(cmd, &mut out_statement);
    assert_ne!(
        out_statement.row_instance,
        Some(Row {
            id: 10,
            username: "blah".to_string(),
            email: "blah@gmail.com".to_string()
        })
    );
}
