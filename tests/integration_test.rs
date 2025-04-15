use sql_engine::backend::{execute_command, execute_statement, Table};
use sql_engine::parser::{prepare_statement, MetaCommandResult, Statement};

// Testing whether unrecognized commands are rejected.
#[test]
fn test_unrecognized_command() {
    let cmd = ".dummy";
    let out = execute_command(cmd);
    assert_eq!(out, MetaCommandResult::Unrecognized);
}

// Testing a complete workflow of insert and select
#[test]
fn test_basic_workflow() {
    let mut table = Table::new();

    // Insert a row
    let mut statement = Statement::default();
    prepare_statement("insert 42 testuser test@example.com", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(_) = result {
        // Select the row
        let mut statement = Statement::default();
        prepare_statement("select 42", &mut statement);
        let result = execute_statement(statement, &mut table);

        if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].id, 42);
            assert_eq!(rows[0].username, "testuser");
            assert_eq!(rows[0].email, "test@example.com");
        } else {
            panic!("Expected Success with one row");
        }
    } else {
        panic!("Expected Success for insert");
    }
}

// Testing multiple operations in sequence
#[test]
fn test_multiple_operations() {
    let mut table = Table::new();

    // Insert multiple rows
    let mut statement = Statement::default();
    prepare_statement("insert 1 user1 user1@example.com", &mut statement);
    execute_statement(statement, &mut table);

    let mut statement = Statement::default();
    prepare_statement("insert 2 user2 user2@example.com", &mut statement);
    execute_statement(statement, &mut table);

    let mut statement = Statement::default();
    prepare_statement("insert 3 user3 user3@example.com", &mut statement);
    execute_statement(statement, &mut table);

    // Select all rows
    let mut statement = Statement::default();
    prepare_statement("select", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 3);
    } else {
        panic!("Expected Success with three rows");
    }

    // Select a specific row
    let mut statement = Statement::default();
    prepare_statement("select 2", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, 2);
        assert_eq!(rows[0].username, "user2");
        assert_eq!(rows[0].email, "user2@example.com");
    } else {
        panic!("Expected Success with one row");
    }
}

// Testing edge cases with special characters and whitespace
#[test]
fn test_special_characters() {
    let mut table = Table::new();

    // Insert with special characters in username and email
    let mut statement = Statement::default();
    prepare_statement(
        "insert 1 user@123 test.user+label@example.com",
        &mut statement,
    );
    let result = execute_statement(statement, &mut table);
    assert!(matches!(
        result,
        sql_engine::backend::ExecuteResult::Success(_)
    ));

    // Verify the data
    let mut statement = Statement::default();
    prepare_statement("select 1", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].username, "user@123");
        assert_eq!(rows[0].email, "test.user+label@example.com");
    } else {
        panic!("Expected Success with one row");
    }
}

// Testing boundary values for IDs
#[test]
fn test_boundary_values() {
    let mut table = Table::new();

    // Test minimum value (0)
    let mut statement = Statement::default();
    prepare_statement("insert 0 user0 zero@example.com", &mut statement);
    let result = execute_statement(statement, &mut table);
    assert!(matches!(
        result,
        sql_engine::backend::ExecuteResult::Success(_)
    ));

    // Test maximum value (u32::MAX)
    let mut statement = Statement::default();
    prepare_statement("insert 4294967295 usermax max@example.com", &mut statement);
    let result = execute_statement(statement, &mut table);
    assert!(matches!(
        result,
        sql_engine::backend::ExecuteResult::Success(_)
    ));

    // Verify both entries
    let mut statement = Statement::default();
    prepare_statement("select", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].id, 0);
        assert_eq!(rows[1].id, 4294967295);
    } else {
        panic!("Expected Success with two rows");
    }
}

// Testing duplicate IDs
#[test]
fn test_duplicate_ids() {
    let mut table = Table::new();

    // Insert first row
    let mut statement = Statement::default();
    prepare_statement("insert 1 user1 user1@example.com", &mut statement);
    execute_statement(statement, &mut table);

    // Insert duplicate ID
    let mut statement = Statement::default();
    prepare_statement("insert 1 user2 user2@example.com", &mut statement);
    execute_statement(statement, &mut table);

    // Verify both entries are present
    let mut statement = Statement::default();
    prepare_statement("select", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].id, 1);
        assert_eq!(rows[0].username, "user1");
        assert_eq!(rows[1].id, 1);
        assert_eq!(rows[1].username, "user2");
    } else {
        panic!("Expected Success with two rows");
    }
}

// Testing select on empty table
#[test]
fn test_empty_table_operations() {
    let mut table = Table::new();

    // Select all from empty table
    let mut statement = Statement::default();
    prepare_statement("select", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 0);
    } else {
        panic!("Expected Success with zero rows");
    }

    // Select specific ID from empty table
    let mut statement = Statement::default();
    prepare_statement("select 1", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 0);
    } else {
        panic!("Expected Success with zero rows");
    }
}

// Testing complex workflow with mixed operations
#[test]
fn test_complex_workflow() {
    let mut table = Table::new();

    // Insert multiple rows
    let test_data = vec![
        ("insert 1 user1 user1@example.com", 1),
        ("insert 2 user2 user2@example.com", 2),
        ("insert 2 user2_dup user2_dup@example.com", 2), // Duplicate ID
        ("insert 3 user3 user3@example.com", 3),
    ];

    for (cmd, _) in &test_data {
        let mut statement = Statement::default();
        prepare_statement(cmd, &mut statement);
        execute_statement(statement, &mut table);
    }

    // Verify total count
    let mut statement = Statement::default();
    prepare_statement("select", &mut statement);
    let result = execute_statement(statement, &mut table);

    if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
        assert_eq!(rows.len(), 4);
    }

    // Verify each ID's count
    for (_, id) in test_data {
        let mut statement = Statement::default();
        prepare_statement(&format!("select {}", id), &mut statement);
        let result = execute_statement(statement, &mut table);

        if let sql_engine::backend::ExecuteResult::Success(Some(rows)) = result {
            if id == 2 {
                assert_eq!(rows.len(), 2); // Should have two rows with ID 2
            } else {
                assert_eq!(rows.len(), 1);
            }
        }
    }
}
