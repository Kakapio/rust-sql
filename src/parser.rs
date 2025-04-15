use scan_fmt::*;

/// The execution result of a non-SQL command.
#[derive(PartialEq, Debug, Default)]
pub enum MetaCommandResult {
    Success,
    #[default]
    Unrecognized,
}

/// The result of parsing a statement and converting it to virtual machine bytecode.
#[derive(PartialEq, Debug, Default)]
pub enum PrepareResult {
    Success,
    #[default]
    Unrecognized,
    SyntaxError,
}

/// The SQL statements we have available to the user.
#[derive(PartialEq, Debug, Default)]
pub enum StatementType {
    Insert,
    #[default]
    Select,
}

/// A particular statement with its corresponding data.
#[derive(PartialEq, Debug, Default)]
pub struct Statement {
    pub cmd: StatementType,
    pub row_instance: Option<Row>,
}

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

/// Converts a SQL statement into bytecode.
pub fn prepare_statement(cmd: &str, statement: &mut Statement) -> PrepareResult {
    // TODO: Use fall-through logic here in the future.
    if cmd.len() >= 6 {
        // First six chars are insert. We use a substring since this is followed by data.
        if &cmd[0..6] == "insert" {
            return prepare_insert(statement, cmd);
        }
        // This can be either 'select' returning all, or 'select 2' return item with ID 2.
        if &cmd[0..6] == "select" {
            return prepare_select(statement, cmd);
        }
    }

    PrepareResult::Unrecognized
}

fn prepare_insert(statement: &mut Statement, cmd: &str) -> PrepareResult {
    statement.cmd = StatementType::Insert;
    let (id, username, email) = match scan_fmt!(cmd, "insert {} {} {}", u32, String, String) {
        Ok((id, username, email)) => (id, username, email),
        Err(_) => {
            println!("Parsing error");
            return PrepareResult::SyntaxError;
        }
    };

    statement.row_instance = Some(Row {
        id,
        username,
        email,
    });

    PrepareResult::Success
}

fn prepare_select(statement: &mut Statement, cmd: &str) -> PrepareResult {
    statement.cmd = StatementType::Select;

    // We are selecting everything in this case, e.g "select". Do not bother looking for specifics.
    if cmd.len() == 6 {
        statement.row_instance = None;
        return PrepareResult::Success;
    }

    // Grab the specific row we are looking for.
    let id = match scan_fmt!(cmd, "select {}", u32) {
        Ok(id) => id,
        Err(_) => {
            println!("Parsing error");
            return PrepareResult::SyntaxError;
        }
    };

    statement.row_instance = Some(Row {
        id,
        username: Default::default(),
        email: Default::default(),
    });

    PrepareResult::Success
}
