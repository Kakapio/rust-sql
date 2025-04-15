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
            if let MetaCommandResult::Unrecognized = execute_command(&input) {
                println!("Unrecognized command: {}", input);
            }
            continue; // Skip this iteration of our IO loop.
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
            ExecuteResult::Success(_) => {
                println!("Successfully executed...")
            }
            ExecuteResult::TableFull => {
                println!("Table is full...")
            }
        }
    }
}

/// Used to execute non-sql CLI commands, e.g exit.
pub fn execute_command(cmd: &str) -> MetaCommandResult {
    if cmd == ".exit" {
        exit(0);
    } else {
        MetaCommandResult::Unrecognized
    }
}

pub fn execute_statement(statement: Statement, tb: &mut Table) -> ExecuteResult {
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
    if statement.row_instance.is_none() {
        for row in table.data.iter() {
            println!("Found data: {:?}", row);
        }
        return ExecuteResult::Success(Some(table.data.iter().cloned().collect()));
    }

    // Select cmd specified an instance of data.
    let target_id = statement.row_instance.as_ref().map(|row| row.id);

    if let Some(id) = target_id {
        for row in table.data.iter() {
            if row.id == id {
                println!("Found data: {:?}", row);
            }
        }

        return ExecuteResult::Success(Some(
            table
                .data
                .iter()
                .filter(|row| row.id == id)
                .cloned()
                .collect(),
        ));
    }

    ExecuteResult::Success(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper method to quickly run SQL commands and mutate a table.
    fn do_sql_cmd(tb: &mut Table, cmd: &str) {
        let mut statement = Statement::default();
        prepare_statement(cmd, &mut statement);
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
                username: "rosh".to_string(),
                email: "kakapio@gmail.com".to_string()
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
                username: "alfred".to_string(),
                email: "alfredddd1@gmail.com".to_string()
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
                    username: "rosh".to_string(),
                    email: "kakapio@gmail.com".to_string()
                },
                Row {
                    id: 42,
                    username: "stefan".to_string(),
                    email: "stefp@sigma.com".to_string()
                },
                Row {
                    id: 1699,
                    username: "sniper_penut".to_string(),
                    email: "penutterbutter@yahoo.com".to_string()
                }
            ]
        );
    }

    // Testing select with no rows in table
    #[test]
    fn execute_select_empty_table() {
        let mut table = Table::new();
        let mut statement = Statement::default();
        prepare_statement("select", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 0);
        } else {
            panic!("Expected Success with empty vector");
        }
    }

    // Testing select with specific ID that doesn't exist
    #[test]
    fn execute_select_nonexistent_id() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");

        let mut statement = Statement::default();
        prepare_statement("select 42", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 0);
        } else {
            panic!("Expected Success with empty vector");
        }
    }

    // Testing select with specific ID that exists
    #[test]
    fn execute_select_existing_id() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");
        do_sql_cmd(&mut table, "insert 42 stefan stefp@sigma.com");

        let mut statement = Statement::default();
        prepare_statement("select 42", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].id, 42);
            assert_eq!(rows[0].username, "stefan");
            assert_eq!(rows[0].email, "stefp@sigma.com");
        } else {
            panic!("Expected Success with one row");
        }
    }

    // Testing select all with multiple rows
    #[test]
    fn execute_select_all_multiple_rows() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");
        do_sql_cmd(&mut table, "insert 42 stefan stefp@sigma.com");
        do_sql_cmd(
            &mut table,
            "insert 1699 sniper_penut penutterbutter@yahoo.com",
        );

        let mut statement = Statement::default();
        prepare_statement("select", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 3);
        } else {
            panic!("Expected Success with three rows");
        }
    }

    // Testing insert with duplicate ID
    #[test]
    fn execute_insert_duplicate_id() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");
        do_sql_cmd(&mut table, "insert 13 stefan stefp@sigma.com");

        // The second insert should be added as a new row
        assert_eq!(
            table.data,
            vec![
                Row {
                    id: 13,
                    username: "rosh".to_string(),
                    email: "kakapio@gmail.com".to_string()
                },
                Row {
                    id: 13,
                    username: "stefan".to_string(),
                    email: "stefp@sigma.com".to_string()
                }
            ]
        );
    }

    // Testing insert with zero ID
    #[test]
    fn execute_insert_zero_id() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 0 rosh kakapio@gmail.com");

        assert_eq!(
            table.data,
            vec![Row {
                id: 0,
                username: "rosh".to_string(),
                email: "kakapio@gmail.com".to_string()
            }]
        );
    }

    // Testing insert with maximum u32 ID
    #[test]
    fn execute_insert_max_id() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 4294967295 rosh kakapio@gmail.com");

        assert_eq!(
            table.data,
            vec![Row {
                id: 4294967295,
                username: "rosh".to_string(),
                email: "kakapio@gmail.com".to_string()
            }]
        );
    }

    // Testing select with zero ID
    #[test]
    fn execute_select_zero_id() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 0 rosh kakapio@gmail.com");

        let mut statement = Statement::default();
        prepare_statement("select 0", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].id, 0);
        } else {
            panic!("Expected Success with one row");
        }
    }

    // Testing select with maximum u32 ID
    #[test]
    fn execute_select_max_id() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 4294967295 rosh kakapio@gmail.com");

        let mut statement = Statement::default();
        prepare_statement("select 4294967295", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].id, 4294967295);
        } else {
            panic!("Expected Success with one row");
        }
    }

    // Testing insert with special characters in username
    #[test]
    fn execute_insert_special_chars_username() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh!@#$ kakapio@gmail.com");

        assert_eq!(
            table.data,
            vec![Row {
                id: 13,
                username: "rosh!@#$".to_string(),
                email: "kakapio@gmail.com".to_string()
            }]
        );
    }

    // Testing insert with special characters in email
    #[test]
    fn execute_insert_special_chars_email() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh kakapio+special@gmail.com");

        assert_eq!(
            table.data,
            vec![Row {
                id: 13,
                username: "rosh".to_string(),
                email: "kakapio+special@gmail.com".to_string()
            }]
        );
    }

    // Testing insert with very long username
    #[test]
    fn execute_insert_long_username() {
        let mut table = Table::new();
        let long_username = "a".repeat(1000);
        do_sql_cmd(
            &mut table,
            &format!("insert 13 {} kakapio@gmail.com", long_username),
        );

        assert_eq!(
            table.data,
            vec![Row {
                id: 13,
                username: long_username,
                email: "kakapio@gmail.com".to_string()
            }]
        );
    }

    // Testing insert with very long email
    #[test]
    fn execute_insert_long_email() {
        let mut table = Table::new();
        let long_email = format!("{}@gmail.com", "a".repeat(1000));
        do_sql_cmd(&mut table, &format!("insert 13 rosh {}", long_email));

        assert_eq!(
            table.data,
            vec![Row {
                id: 13,
                username: "rosh".to_string(),
                email: long_email,
            }]
        );
    }

    // Testing multiple selects on the same table
    #[test]
    fn execute_multiple_selects() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");
        do_sql_cmd(&mut table, "insert 42 stefan stefp@sigma.com");
        do_sql_cmd(
            &mut table,
            "insert 1699 sniper_penut penutterbutter@yahoo.com",
        );

        // Select all
        let mut statement = Statement::default();
        prepare_statement("select", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 3);
        } else {
            panic!("Expected Success with three rows");
        }

        // Select specific ID
        let mut statement = Statement::default();
        prepare_statement("select 42", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].id, 42);
        } else {
            panic!("Expected Success with one row");
        }
    }

    // Testing insert and select with whitespace
    #[test]
    fn execute_insert_select_whitespace() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "  insert  13  rosh  kakapio@gmail.com  ");

        let mut statement = Statement::default();
        prepare_statement("  select  13  ", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 0);
        } else {
            panic!("Expected Success with empty vector");
        }
    }

    // Testing insert and select with tab characters
    #[test]
    fn execute_insert_select_tab() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert\t13\trosh\tkakapio@gmail.com");

        let mut statement = Statement::default();
        prepare_statement("select\t13", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].id, 13);
        } else {
            panic!("Expected Success with one row");
        }
    }

    // Testing insert and select with newline characters
    #[test]
    fn execute_insert_select_newline() {
        let mut table = Table::new();
        do_sql_cmd(&mut table, "insert\n13\nrosh\nkakapio@gmail.com");

        let mut statement = Statement::default();
        prepare_statement("select\n13", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].id, 13);
        } else {
            panic!("Expected Success with one row");
        }
    }
}
