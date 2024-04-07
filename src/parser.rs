use scan_fmt::*;

#[derive(PartialEq, Debug, Default)]
pub enum MetaCommandResult {
    SUCCESS,
    #[default]
    UNRECOGNIZED,
}

#[derive(PartialEq, Debug, Default)]
pub enum PrepareResult {
    SUCCESS,
    #[default]
    UNRECOGNIZED,
    SYNTAX_ERROR,
}

#[derive(PartialEq, Debug, Default)]
pub enum StatementType {
    INSERT,
    #[default]
    SELECT,
}

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
    // First six chars are insert. We use a substring since this is followed by data.
    if cmd.len() >= 6 && &cmd[0..6] == "insert" {
        statement.cmd = StatementType::INSERT;

        // This is how we take our formatted string and put it into variables.
        let (id, username, email) = match scan_fmt!(cmd, "insert {} {} {}", u32, String, String) {
            Ok((id, username, email)) => (id, username, email),
            Err(_) => {
                println!("Parsing error");
                return PrepareResult::SYNTAX_ERROR;
            }
        };

        statement.row_instance = Row {
            id,
            username,
            email,
        };
        return PrepareResult::SUCCESS;
    }
    if cmd == "select" {
        statement.cmd = StatementType::SELECT;
        return PrepareResult::SUCCESS;
    }

    return PrepareResult::UNRECOGNIZED;
}
