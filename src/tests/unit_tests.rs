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

// Testing insert with empty username
#[test]
fn prepare_statement_insert_empty_username() {
    let mut out_statement = Statement::default();
    let cmd = "insert 10  ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

// Testing insert with empty email
#[test]
fn prepare_statement_insert_empty_email() {
    let mut out_statement = Statement::default();
    let cmd = "insert 10 monkeylover ";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

// Testing insert with invalid ID (not a number)
#[test]
fn prepare_statement_insert_invalid_id() {
    let mut out_statement = Statement::default();
    let cmd = "insert abc monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

// Testing insert with negative ID
#[test]
fn prepare_statement_insert_negative_id() {
    let mut out_statement = Statement::default();
    let cmd = "insert -10 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

// Testing insert with very large ID
#[test]
fn prepare_statement_insert_large_id() {
    let mut out_statement = Statement::default();
    let cmd = "insert 4294967295 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing insert with ID exceeding u32 max
#[test]
fn prepare_statement_insert_id_overflow() {
    let mut out_statement = Statement::default();
    let cmd = "insert 4294967296 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

// Testing select with invalid ID
#[test]
fn prepare_statement_select_invalid_id() {
    let mut out_statement = Statement::default();
    let cmd = "select abc";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

// Testing select with negative ID
#[test]
fn prepare_statement_select_negative_id() {
    let mut out_statement = Statement::default();
    let cmd = "select -10";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

// Testing select with very large ID
#[test]
fn prepare_statement_select_large_id() {
    let mut out_statement = Statement::default();
    let cmd = "select 4294967295";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing select with ID exceeding u32 max
#[test]
fn prepare_statement_select_id_overflow() {
    let mut out_statement = Statement::default();
    let cmd = "select 4294967296";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

// Testing select with extra parameters
#[test]
fn prepare_statement_select_extra_params() {
    let mut out_statement = Statement::default();
    let cmd = "select 10 extra";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing insert with extra parameters
#[test]
fn prepare_statement_insert_extra_params() {
    let mut out_statement = Statement::default();
    let cmd = "insert 10 monkeylover ape@gmail.com extra";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing insert with special characters in username
#[test]
fn prepare_statement_insert_special_chars_username() {
    let mut out_statement = Statement::default();
    let cmd = "insert 10 monkey_lover!@#$ ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing insert with special characters in email
#[test]
fn prepare_statement_insert_special_chars_email() {
    let mut out_statement = Statement::default();
    let cmd = "insert 10 monkeylover ape+special@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing insert with very long username
#[test]
fn prepare_statement_insert_long_username() {
    let mut out_statement = Statement::default();
    let cmd = format!("insert 10 {} ape@gmail.com", "a".repeat(1000));
    let out_result = prepare_statement(&cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing insert with very long email
#[test]
fn prepare_statement_insert_long_email() {
    let mut out_statement = Statement::default();
    let cmd = format!("insert 10 monkeylover {}@gmail.com", "a".repeat(1000));
    let out_result = prepare_statement(&cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing case sensitivity in commands
#[test]
fn prepare_statement_case_sensitivity() {
    let mut out_statement = Statement::default();
    let cmd = "INSERT 10 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Unrecognized);
}

// Testing whitespace handling
#[test]
fn prepare_statement_whitespace_handling() {
    let mut out_statement = Statement::default();
    let cmd = "  insert  10  monkeylover  ape@gmail.com  ";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Unrecognized);
}

// Testing empty command
#[test]
fn prepare_statement_empty_command() {
    let mut out_statement = Statement::default();
    let cmd = "";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Unrecognized);
}

// Testing whitespace-only command
#[test]
fn prepare_statement_whitespace_only() {
    let mut out_statement = Statement::default();
    let cmd = "   ";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Unrecognized);
}

// Testing tab characters in command
#[test]
fn prepare_statement_tab_characters() {
    let mut out_statement = Statement::default();
    let cmd = "insert\t10\tmonkeylover\tape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

// Testing newline characters in command
#[test]
fn prepare_statement_newline_characters() {
    let mut out_statement = Statement::default();
    let cmd = "insert\n10\nmonkeylover\nape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}
