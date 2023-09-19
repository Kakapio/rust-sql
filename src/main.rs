use std::io;
use std::process::exit;
use crate::parser::*;
use scan_fmt::*;

mod parser;
mod backend;

fn main() {
    entrypoint();
}

fn entrypoint()
{
    let stdin = io::stdin(); // binding a handle.

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
                println!("Unrecognized syntax for command. Did you follow the format?");
                continue;
            }
        }

        execute_statement(&statement);
        println!("Successfully executed...");
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

        // This is how we take our formatted string and put it into variables.
        let (id, username, email) = match scan_fmt!(cmd, "insert {} {} {}", u32, String, String) {
            Ok((id, username, email)) => {
                // Convert String to char arrays
                let username_chars: [char; 32] = {
                    let mut chars = [' '; 32];
                    let bytes = username.as_bytes();
                    for (i, &byte) in bytes.iter().enumerate() {
                        chars[i] = byte as char;
                    }
                    chars
                };

                let email_chars: [char; 255] = {
                    let mut chars = [' '; 255];
                    let bytes = email.as_bytes();
                    for (i, &byte) in bytes.iter().enumerate() {
                        chars[i] = byte as char;
                    }
                    chars
                };

                (id, Username(username_chars), Email(email_chars))
            },
            Err(_) => {
                println!("Parsing error");
                return PrepareResult::SYNTAX_ERROR;
            }
        };

        statement.row_to_insert = Row { id, username, email };
        return PrepareResult::SUCCESS;
    }
    if cmd == "select"
    {
        statement.cmd = StatementType::SELECT;
        return PrepareResult::SUCCESS;
    }

    return PrepareResult::UNRECOGNIZED;
}

fn execute_statement(statement: &Statement)
{
    match statement.cmd {
        StatementType::INSERT => { println!("Performing an insert...") }
        StatementType::SELECT => { println!("Performing a select...") }
    }
}

#[cfg(test)]
mod tests {
    use crate::{execute_command, prepare_statement};
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
        assert_eq!(out_statement.row_to_insert, Row {
            id: 10,
            username: Username("monkeylover".chars().take(32).collect::<Vec<char>>().try_into().unwrap()), //String::from("monkeylover"),
            email: Email("ape@gmail.com".chars().take(32).collect::<Vec<char>>().try_into().unwrap())//String::from("ape@gmail.com")
        });
    }

    // Testing whether the parsing works by checking with incorrect data.
    #[test]
    fn prepare_statement_insert_parse_fail() {
        let mut out_statement = Statement::default();
        let cmd = String::from("insert 10 monkeylover ape@gmail.com");
        prepare_statement(&cmd, &mut out_statement);
        assert_ne!(out_statement.row_to_insert, Row {
            id: 10,
            username: Username("blah".chars().take(32).collect::<Vec<char>>().try_into().unwrap()), //String::from("blah"),
            email: Email("ape@gmail.com".chars().take(32).collect::<Vec<char>>().try_into().unwrap()) //String::from("ape@gmail.com")
        });
    }
}