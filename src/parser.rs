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
    pub row_instance: Row,
}

#[derive(PartialEq, Debug, Default)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

pub fn prepare_statement(cmd: &String, statement: &mut Statement) -> PrepareResult {
    // Use fall-through logic here in the future.
    if cmd.len() >= 6 {
        // First six chars are insert. We use a substring since this is followed by data.
        if &cmd[0..6] == "insert" {
            statement.cmd = StatementType::Insert;

            // This is how we take our formatted string and put it into variables.
            let (id, username, email) = match scan_fmt!(cmd, "insert {} {} {}", u32, String, String)
            {
                Ok((id, username, email)) => (id, username, email),
                Err(_) => {
                    println!("Parsing error");
                    return PrepareResult::SyntaxError;
                }
            };

            statement.row_instance = Row {
                id,
                username,
                email,
            };
            return PrepareResult::Success;
        }
        // This can be either 'select' returning all, or 'select 2' return item with ID 2.
        else if &cmd[0..6] == "select" {
            statement.cmd = StatementType::Select;
            return PrepareResult::Success;
        }
    }

    return PrepareResult::Unrecognized;
}
