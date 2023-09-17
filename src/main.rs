use std::io;
use std::process::exit;

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
                sql_engine::MetaCommandResult::SUCCESS => { continue; }
                sql_engine::MetaCommandResult::UNRECOGNIZED => {
                    println!("Unrecognized command: {}", input);
                    continue;
                }
            }
        }
    }
}

fn execute_command(cmd: &String) -> sql_engine::MetaCommandResult {
    if cmd == ".exit"
    {
        exit(0);
    }
    else
    {
        return sql_engine::MetaCommandResult::UNRECOGNIZED;
    }
}

fn prepare_statement(cmd: &String, statement: &mut sql_engine::Statement)
{
    // First six chars are insert. We use a substring since this is followed by data.
    if &cmd[0..6]== "insert"
    {
        statement.cmd = sql_engine::StatementType::INSERT;
    }
    if cmd == "select"
    {
        statement.cmd = sql_engine::StatementType::SELECT;
    }
}

#[cfg(test)]
mod tests {
    use crate::{execute_command, prepare_statement, sql_engine};

    #[test]
    fn execute_command_1() {
        let cmd = String::from(".dummy");
        let out = execute_command(&cmd);
        assert_eq!(out, sql_engine::MetaCommandResult::UNRECOGNIZED);
    }

    // Testing whether the enum is set properly.
    #[test]
    fn prepare_statement_1() {
        let mut out_statement = sql_engine::Statement{ cmd: sql_engine::StatementType::SELECT };
        let cmd = String::from("insert");
        prepare_statement(&cmd, &mut out_statement);
        let result = 2 + 2;
        assert_eq!(out_statement.cmd, sql_engine::StatementType::INSERT);
    }

    // Testing whether the enum is set properly.
    #[test]
    fn prepare_statement_2() {
        let mut out_statement = sql_engine::Statement{ cmd: sql_engine::StatementType::INSERT };
        let cmd = String::from("select");
        prepare_statement(&cmd, &mut out_statement);
        let result = 2 + 2;
        assert_eq!(out_statement.cmd, sql_engine::StatementType::SELECT);
    }
}