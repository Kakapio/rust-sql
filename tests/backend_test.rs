use sql_engine::backend::{execute_command, execute_statement, Table};
use sql_engine::parser::{prepare_statement, MetaCommandResult, Row, Statement};

/// Helper method to quickly run SQL commands and mutate a table.
fn do_sql_cmd(tb: &mut Table, cmd: &str) {
    let mut statement = Statement::default();
    prepare_statement(cmd, &mut statement);
    execute_statement(statement, tb);
}

#[test]
fn test_execute_statement_insert() {
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

#[test]
fn test_execute_statement_insert_fail() {
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

#[test]
fn test_execute_multiple_insert() {
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

#[test]
fn test_execute_select_empty_table() {
    let mut table = Table::new();
    let mut statement = Statement::default();
    prepare_statement("select", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 0);
    } else {
        panic!("Expected Success with empty vector");
    }
}

#[test]
fn test_execute_select_nonexistent_id() {
    let mut table = Table::new();
    do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");

    let mut statement = Statement::default();
    prepare_statement("select 42", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 0);
    } else {
        panic!("Expected Success with empty vector");
    }
}

#[test]
fn test_execute_select_existing_id() {
    let mut table = Table::new();
    do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");
    do_sql_cmd(&mut table, "insert 42 stefan stefp@sigma.com");

    let mut statement = Statement::default();
    prepare_statement("select 42", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, 42);
        assert_eq!(rows[0].username, "stefan");
        assert_eq!(rows[0].email, "stefp@sigma.com");
    } else {
        panic!("Expected Success with one row");
    }
}

#[test]
fn test_execute_select_all_multiple_rows() {
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

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 3);
    } else {
        panic!("Expected Success with three rows");
    }
}

#[test]
fn test_execute_insert_duplicate_id() {
    let mut table = Table::new();
    do_sql_cmd(&mut table, "insert 13 rosh kakapio@gmail.com");
    do_sql_cmd(&mut table, "insert 13 stefan stefp@sigma.com");

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

#[test]
fn test_execute_insert_zero_id() {
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

#[test]
fn test_execute_insert_max_id() {
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

#[test]
fn test_execute_select_zero_id() {
    let mut table = Table::new();
    do_sql_cmd(&mut table, "insert 0 rosh kakapio@gmail.com");

    let mut statement = Statement::default();
    prepare_statement("select 0", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, 0);
    } else {
        panic!("Expected Success with one row");
    }
}

#[test]
fn test_execute_select_max_id() {
    let mut table = Table::new();
    do_sql_cmd(&mut table, "insert 4294967295 rosh kakapio@gmail.com");

    let mut statement = Statement::default();
    prepare_statement("select 4294967295", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, 4294967295);
    } else {
        panic!("Expected Success with one row");
    }
}

#[test]
fn test_execute_insert_special_chars_username() {
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

#[test]
fn test_execute_insert_special_chars_email() {
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

#[test]
fn test_execute_insert_long_username() {
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

#[test]
fn test_execute_insert_long_email() {
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

#[test]
fn test_execute_multiple_selects() {
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

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 3);
    } else {
        panic!("Expected Success with three rows");
    }

    // Select specific ID
    let mut statement = Statement::default();
    prepare_statement("select 42", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, 42);
    } else {
        panic!("Expected Success with one row");
    }
}

#[test]
fn test_execute_command_unrecognized() {
    let cmd = ".dummy";
    let out = execute_command(cmd);
    assert_eq!(out, MetaCommandResult::Unrecognized);
}
