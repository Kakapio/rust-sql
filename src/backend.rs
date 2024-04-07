use crate::parser::*;
use std::io;
use std::process::exit;

#[derive(PartialEq, Debug, Default)]
pub enum ExecuteResult {
    SUCCESS,
    #[default]
    TABLE_FULL,
}

pub struct Table {
    pub data: Vec<Row>,
}

impl Table {
    pub fn new() -> Table {
        Table { data: Vec::new() }
    }
}

pub fn entrypoint() {
    let stdin = io::stdin(); // binding a handle.
    let mut table = Table::new();

    loop
    // same as while(true)
    {
        let mut input = String::new();

        println!("Please enter the SQL command: ");
        stdin.read_line(&mut input).expect("Failed to read line.");
        input = input.trim().to_string(); // remove trailing newline.

        // Is a command
        if input.starts_with('.') {
            match execute_command(&input) {
                MetaCommandResult::SUCCESS => {
                    continue;
                } // Executed command, get next input.
                MetaCommandResult::UNRECOGNIZED => {
                    println!("Unrecognized command: {}", input);
                    continue; // skip this iteration of our IO loop.
                }
            }
        }

        let mut statement = Statement::default();

        match prepare_statement(&input, &mut statement) {
            PrepareResult::SUCCESS => {
                println!("Successfully prepared statement...")
            }
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
            ExecuteResult::SUCCESS => {
                println!("Successfully executed...")
            }
            ExecuteResult::TABLE_FULL => {
                println!("Table is full...")
            }
        }
    }
}

pub fn execute_command(cmd: &String) -> MetaCommandResult {
    if cmd == ".exit" {
        exit(0);
    } else {
        return MetaCommandResult::UNRECOGNIZED;
    }
}

fn execute_statement(statement: Statement, tb: &mut Table) -> ExecuteResult {
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
