use crate::parser::*;
use std::io;
use std::process::exit;

#[derive(PartialEq, Debug, Default)]
pub enum ExecuteResult {
    Success,
    #[default]
    TableFull,
}

/// Represents a single SQL table.
pub struct Table {
    pub data: Vec<Row>,
}

impl Table {
    pub fn new() -> Table {
        Table { data: Vec::new() }
    }
}

pub fn entrypoint() {
    let stdin = io::stdin();
    let mut table = Table::new();

    loop {
        let mut input = String::new();

        println!("Please enter the SQL command: ");
        stdin.read_line(&mut input).expect("Failed to read line.");
        input = input.trim().to_string(); // Remove trailing newline.

        // Is a command
        if input.starts_with('.') {
            match execute_command(&input) {
                MetaCommandResult::Success => {
                    continue; // Executed command, get next input.
                }
                MetaCommandResult::Unrecognized => {
                    println!("Unrecognized command: {}", input);
                    continue; // Skip this iteration of our IO loop.
                }
            }
        }

        let mut statement = Statement::default();

        match prepare_statement(&input, &mut statement) {
            PrepareResult::Success => {
                println!("Successfully prepared statement...")
            }
            PrepareResult::Unrecognized => {
                println!("Unrecognized keyword at start of {}", input);
                continue;
            }
            PrepareResult::SyntaxError => {
                println!("Unrecognized syntax for command. Did you follow the proper format?");
                continue;
            }
        }

        match execute_statement(statement, &mut table) {
            ExecuteResult::Success => {
                println!("Successfully executed...")
            }
            ExecuteResult::TableFull => {
                println!("Table is full...")
            }
        }
    }
}

/// Used to execute non-sql CLI commands, e.g exit.
pub fn execute_command(cmd: &String) -> MetaCommandResult {
    if cmd == ".exit" {
        exit(0);
    } else {
        return MetaCommandResult::Unrecognized;
    }
}

fn execute_statement(statement: Statement, tb: &mut Table) -> ExecuteResult {
    match statement.cmd {
        StatementType::Insert => {
            println!("Performing an insert...");
            execute_insert(statement, tb)
        }
        StatementType::Select => {
            println!("Performing a select...");
            execute_select(statement, tb)
        }
    }
}

fn execute_insert(statement: Statement, table: &mut Table) -> ExecuteResult {
    table.data.push(statement.row_instance.expect("Insert is missing row data."));

    ExecuteResult::Success
}

fn execute_select(statement: Statement, table: &mut Table) -> ExecuteResult {

    // Select didn't specify an instance. Return all data in table.
    if statement.row_instance == None {
        for row in table.data.iter() {
            println!("Found data: {:?}", row);
        }
        return ExecuteResult::Success;
    }
    
    // Select specified an instance of data.
    for row in table.data.iter() {
        /* I use 'as_ref()' here because otherwise the ownership of row_instance would go to unwrap().
         * unwrap() consumes the given option which means iteration gets interrupted.
         */
        if row.id == statement.row_instance.as_ref().unwrap().id {
            println!("Found data: {:?}", row);
        }
    }

    ExecuteResult::Success
}
