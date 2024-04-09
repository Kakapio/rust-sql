use crate::parser::*;
use std::io;
use std::process::exit;

#[derive(PartialEq, Debug, Default)]
pub enum ExecuteResult {
    Success(Option<Vec<Row>>),
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
            ExecuteResult::Success(data) => {
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
    table
        .data
        .push(statement.row_instance.expect("Insert is missing row data."));

    ExecuteResult::Success(None)
}

fn execute_select(statement: Statement, table: &mut Table) -> ExecuteResult {
    // Select didn't specify an instance. Return all data in table.
    if statement.row_instance == None {
        for row in table.data.iter() {
            println!("Found data: {:?}", row);
        }
        return ExecuteResult::Success(Some(table.data.iter().cloned().collect()));
    }

    // Select cmd specified an instance of data.
    for row in table.data.iter() {
        /* I use 'as_ref()' here because otherwise the ownership of row_instance would go to unwrap().
         * unwrap() consumes the given option which means iteration gets interrupted.
         */
        if row.id == statement.row_instance.as_ref().unwrap().id {
            println!("Found data: {:?}", row);
        }
    }

    ExecuteResult::Success(Some(
        table
            .data
            .iter()
            .filter(|row| row.id == statement.row_instance.as_ref().unwrap().id)
            .cloned()
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper method to quickly run SQL commands and mutate a table.
    fn do_sql_cmd(tb: &mut Table, cmd: &str) {
        let mut statement = Statement::default();
        prepare_statement(&String::from(cmd), &mut statement);
        execute_statement(statement, tb);
    }

    // Testing whether insert command errors.
    #[test]
    fn execute_statement_insert() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");

        assert_eq!(
            table.data,
            vec![Row {
                id: 13,
                username: String::from("rosh"),
                email: String::from("kakapio@gmail.com")
            }]
        );
    }

    // Making sure our test doesn't allow everything to pass.
    #[test]
    fn execute_statement_insert_fail() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");

        assert_ne!(
            table.data,
            vec![Row {
                id: 13,
                username: String::from("alfred"),
                email: String::from("alfredddd1@gmail.com")
            }]
        );
    }

    // Making sure we can insert multiple things and get them back out.
    #[test]
    fn execute_multiple_insert() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");
        do_sql_cmd(&mut table, "insert 42 stefan stefp@sigma.com");
        do_sql_cmd(
            &mut table,
            "insert 1699 sniper_penut penutterbutter@yahoo.com",
        );

        assert_eq!(
            table.data,
            vec![
                Row {
                    id: 13,
                    username: String::from("rosh"),
                    email: String::from("kakapio@gmail.com")
                },
                Row {
                    id: 42,
                    username: String::from("stefan"),
                    email: String::from("stefp@sigma.com")
                },
                Row {
                    id: 1699,
                    username: String::from("sniper_penut"),
                    email: String::from("penutterbutter@yahoo.com")
                }
            ]
        );
    }
}
