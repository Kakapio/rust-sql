use scan_fmt::*;

#[derive(PartialEq, Debug, Default)]
pub enum MetaCommandResult {
    SUCCESS,
    #[default]
    UNRECOGNIZED
}

#[derive(PartialEq, Debug, Default)]
pub enum PrepareResult {
    SUCCESS,
    #[default]
    UNRECOGNIZED,
    SYNTAX_ERROR
}

#[derive(PartialEq, Debug, Default)]
pub enum StatementType {
    INSERT,
    #[default]
    SELECT
}

#[derive(PartialEq, Debug, Default)]
pub struct Statement {
    pub cmd: StatementType,
    pub row_to_insert: Row
}

pub const USERNAME_LENGTH: usize = 32;
pub const EMAIL_LENGTH: usize = 255;

type UsernameArray = [char; USERNAME_LENGTH];
type EmailArray = [char; EMAIL_LENGTH];

#[derive(PartialEq, Debug)]
pub struct Username(pub UsernameArray);
#[derive(PartialEq, Debug)]
pub struct Email(pub EmailArray);

impl Default for Username {
    fn default() -> Self {
        Username([' '; USERNAME_LENGTH])
    }
}

impl Default for Email {
    fn default() -> Self {
        Email([' '; EMAIL_LENGTH])
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Row {
    pub id: u32,
    pub username: Username,
    pub email: Email
}

pub fn prepare_statement(cmd: &String, statement: &mut Statement) -> PrepareResult
{
    // First six chars are insert. We use a substring since this is followed by data.
    if cmd.len() >= 6 && &cmd[0..6]== "insert"
    {
        statement.cmd = StatementType::INSERT;

        // This is how we take our formatted string and put it into variables.
        let (id, username, email) = match scan_fmt!(cmd, "insert {} {} {}", u32, String, String) {
            Ok((id, username, email)) => (id,
                                          Username(crate::str_to_array(&username)),
                                          Email(crate::str_to_array(&email))),
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
