use std::io;
use std::process::exit;
use crate::parser::*;
use crate::backend::*;

mod parser;
mod backend;

fn main() {
    entrypoint();
}

fn entrypoint()
{
    let stdin = io::stdin(); // binding a handle.
    let mut table = Table::new();

    loop // same as while(true)
    {
        let mut input = String::new();

        println!("Please enter the SQL command: ");
        stdin.read_line(&mut input).expect("Failed to read line.");
        input = input.trim().to_string(); // remove trailing newline.

        // Is a command
        if input.starts_with('.')
        {
            match execute_command(&input)
            {
                MetaCommandResult::SUCCESS => { continue; } // Executed command, get next input.
                MetaCommandResult::UNRECOGNIZED => {
                    println!("Unrecognized command: {}", input);
                    continue; // skip this iteration of our IO loop.
                }
            }
        }

        let mut statement = Statement::default();

        match prepare_statement(&input, &mut statement)
        {
            PrepareResult::SUCCESS => { println!("Successfully prepared statement...") }
            PrepareResult::UNRECOGNIZED => {
                println!("Unrecognized keyword at start of {}", input);
                continue;
            }
            PrepareResult::SYNTAX_ERROR => {
                println!("Unrecognized syntax for command. Did you follow the proper format?");
                continue;
            }
        }

        match execute_statement(statement, &mut table) {
            ExecuteResult::SUCCESS => { println!("Successfully executed...") }
            ExecuteResult::TABLE_FULL => { println!("Table is full...") }
        }
    }
}

fn execute_command(cmd: &String) -> MetaCommandResult
{
    if cmd == ".exit"
    {
        exit(0);
    }
    else
    {
        return MetaCommandResult::UNRECOGNIZED;
    }
}

fn execute_statement(statement: Statement, tb: &mut Table) -> ExecuteResult
{
    match statement.cmd {
        StatementType::INSERT => {
            println!("Performing an insert...");
            execute_insert(statement, tb)
        }
        StatementType::SELECT => {
            println!("Performing a select...");
            execute_select(statement, tb)
        }
    }
}

// Todo: Should I move or borrow statement... I think move?
fn execute_insert(statement: Statement, table: &mut Table) -> ExecuteResult {
    table.data.push(statement.row_instance);
    ExecuteResult::SUCCESS
}

fn execute_select(statement: Statement, table: &mut Table) -> ExecuteResult {

    for row in table.data.iter() {
        if row.id == statement.row_instance.id {
            println!("Found data: {:?}", row);
        }
    }
    ExecuteResult::SUCCESS
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::parser::*;

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