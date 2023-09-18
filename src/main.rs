use std::io;
use std::process::exit;
use crate::sql_engine::*;

mod sql_engine;

fn main() {
    entrypoint();
}

fn entrypoint() {
    let stdin = io::stdin(); // binding a handle.

    loop // same as while(true)
    {
        let mut input = String::new();

        println!("Please enter the SQL command: ");
        stdin.read_line(&mut input).expect("Failed to read line.");
        input = input.trim().to_string(); // remove trailing newline.

        if input.starts_with('.')
        {
            match execute_command(&input)
            {
                MetaCommandResult::SUCCESS => { continue; }
                MetaCommandResult::UNRECOGNIZED => {
                    println!("Unrecognized command: {}", input);
                    continue;
                }
            }
        }

        let mut statement = Statement::default();

        match prepare_statement(&input, &mut statement)
        {
            PrepareResult::SUCCESS => { break; }
            PrepareResult::UNRECOGNIZED => {
                println!("Unrecognized keyword at start of {}", input);
                continue;
            }
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

fn prepare_statement(cmd: &String, statement: &mut Statement) -> PrepareResult
{
    // First six chars are insert. We use a substring since this is followed by data.
    if cmd.len() >= 6 && &cmd[0..6]== "insert"
    {
        statement.cmd = StatementType::INSERT;
        return PrepareResult::SUCCESS;
    }
    if cmd == "select"
    {
        statement.cmd = StatementType::SELECT;
        return PrepareResult::SUCCESS;
    }

    return PrepareResult::UNRECOGNIZED;
}

#[cfg(test)]
mod tests {
    use crate::{execute_command, prepare_statement};
    use crate::sql_engine::*;

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
        let cmd = String::from("insert");
        let out_result = prepare_statement(&cmd, &mut out_statement);
        assert_eq!(out_result, PrepareResult::SUCCESS);
    }

    // Testing whether the output result is correct.
    #[test]
    fn prepare_statement_out_failure() {
        let mut out_statement = Statement::default();
        let cmd = String::from("dummy");
        let out_result = prepare_statement(&cmd, &mut out_statement);
        assert_eq!(out_result, PrepareResult::UNRECOGNIZED);
    }
}