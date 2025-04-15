use sql_engine::parser::{
    prepare_statement, PrepareResult, Row, Statement, StatementType,
};

// Testing whether the enum is set properly.
#[test]
fn test_prepare_statement_set_insert() {
    let mut out_statement = Statement::default();
    let cmd = "insert";
    prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_statement.cmd, StatementType::Insert);
}

#[test]
fn test_prepare_statement_set_select() {
    let mut out_statement = Statement::default();
    let cmd = "select";
    prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_statement.cmd, StatementType::Select);
}

#[test]
fn test_prepare_statement_success() {
    let cmd = "insert 10 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut Statement::default());
    assert_eq!(out_result, PrepareResult::Success);
}

#[test]
fn test_prepare_statement_unrecognized() {
    let cmd = "dummy";
    let out_result = prepare_statement(cmd, &mut Statement::default());
    assert_eq!(out_result, PrepareResult::Unrecognized);
}

#[test]
fn test_prepare_statement_syntax_error() {
    let mut out_statement = Statement::default();
    let cmd = "insert";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

#[test]
fn test_prepare_statement_insert_parse() {
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

#[test]
fn test_prepare_statement_insert_parse_fail() {
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

#[test]
fn test_prepare_statement_empty_username() {
    let mut out_statement = Statement::default();
    let cmd = "insert 10  ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

#[test]
fn test_prepare_statement_empty_email() {
    let mut out_statement = Statement::default();
    let cmd = "insert 10 monkeylover ";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

#[test]
fn test_prepare_statement_invalid_id() {
    let mut out_statement = Statement::default();
    let cmd = "insert abc monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

#[test]
fn test_prepare_statement_negative_id() {
    let mut out_statement = Statement::default();
    let cmd = "insert -10 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

#[test]
fn test_prepare_statement_large_id() {
    let mut out_statement = Statement::default();
    let cmd = "insert 4294967295 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Success);
}

#[test]
fn test_prepare_statement_id_overflow() {
    let mut out_statement = Statement::default();
    let cmd = "insert 4294967296 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::SyntaxError);
}

#[test]
fn test_prepare_statement_case_sensitivity() {
    let mut out_statement = Statement::default();
    let cmd = "INSERT 10 monkeylover ape@gmail.com";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Unrecognized);
}

#[test]
fn test_prepare_statement_whitespace() {
    let mut out_statement = Statement::default();
    let cmd = "  insert  10  monkeylover  ape@gmail.com  ";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Unrecognized);
}

#[test]
fn test_prepare_statement_empty() {
    let mut out_statement = Statement::default();
    let cmd = "";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Unrecognized);
}

#[test]
fn test_prepare_statement_whitespace_only() {
    let mut out_statement = Statement::default();
    let cmd = "   ";
    let out_result = prepare_statement(cmd, &mut out_statement);
    assert_eq!(out_result, PrepareResult::Unrecognized);
}
