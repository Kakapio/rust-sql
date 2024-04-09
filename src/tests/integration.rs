use crate::backend::*;
use crate::parser::*;

// Testing whether unrecognized commands are rejected.
#[test]
fn insert_then_select_all {
    let cmd = String::from(".dummy");
    let out = execute_command(&cmd);
    assert_eq!(out, MetaCommandResult::Unrecognized);
}