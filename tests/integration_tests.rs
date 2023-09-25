extern crate sql_engine;
use sql_engine::*;

mod integration_tests {

    // Testing whether unrecognized commands are rejected.
    #[test]
    fn execute_command_unrecognized() {
        let cmd = String::from(".dummy");
        let out = execute_command(&cmd);
        assert_eq!(out, MetaCommandResult::UNRECOGNIZED);
    }

    // Testing whether the enum is set properly.
    #[test]
    fn prepare_statement_set_insert() {
        let mut out_statement = Statement::default();
        let cmd = String::from("insert");
        prepare_statement(&cmd, &mut out_statement);
        assert_eq!(out_statement.cmd, StatementType::INSERT);
    }

    // Testing whether the enum is set properly.
    #[test]
    fn prepare_statement_set_select() {
        let mut out_statement = Statement::default();
        let cmd = String::from("select");
        prepare_statement(&cmd, &mut out_statement);
        assert_eq!(out_statement.cmd, StatementType::SELECT);
    }

    // Testing whether the output result is correct.
    #[test]
    fn prepare_statement_out_success() {
        let mut out_statement = Statement::default();
        let cmd = String::from("insert 10 monkeylover ape@gmail.com");
        let out_result = prepare_statement(&cmd, &mut out_statement);
        assert_eq!(out_result, PrepareResult::SUCCESS);
    }

    // Testing whether the output result handles bad commands.
    #[test]
    fn prepare_statement_out_failure() {
        let mut out_statement = Statement::default();
        let cmd = String::from("dummy");
        let out_result = prepare_statement(&cmd, &mut out_statement);
        assert_eq!(out_result, PrepareResult::UNRECOGNIZED);
    }

    // Testing whether the insert syntax error is handled.
    #[test]
    fn prepare_statement_out_syntax_error() {
        let mut out_statement = Statement::default();
        let cmd = String::from("insert");
        let out_result = prepare_statement(&cmd, &mut out_statement);
        assert_eq!(out_result, PrepareResult::SYNTAX_ERROR);
    }

    // Testing whether the parsing works.
    #[test]
    fn prepare_statement_insert_parse() {
        let mut out_statement = Statement::default();
        let cmd = String::from("insert 10 monkeylover ape@gmail.com");
        prepare_statement(&cmd, &mut out_statement);
        assert_eq!(out_statement.row_instance, Row {
            id: 10,
            username: String::from("monkeylover"),
            email: String::from("ape@gmail.com")
        });
    }

    // Testing whether the parsing works by checking with incorrect data.
    #[test]
    fn prepare_statement_insert_parse_fail() {
        let mut out_statement = Statement::default();
        let cmd = String::from("insert 10 monkeylover ape@gmail.com");
        prepare_statement(&cmd, &mut out_statement);
        assert_ne!(out_statement.row_instance, Row {
            id: 10,
            username: String::from("blah"),
            email: String::from("blah@gmail.com")
        });
    }
}