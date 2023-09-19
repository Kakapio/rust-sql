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

type UsernameArray = [char; 32];
type EmailArray = [char; 255];

#[derive(PartialEq, Debug)]
pub struct Username(pub UsernameArray);
#[derive(PartialEq, Debug)]
pub struct Email(pub EmailArray);

impl Default for Username {
    fn default() -> Self {
        Username([' '; 32])
    }
}
//
// impl PartialEq for Username {
//     fn eq(&self, other: &Self) -> bool {
//         // Compare each element of the array
//         self.0.iter().eq(other.0.iter())
//     }
// }
//
// impl PartialEq for Email {
//     fn eq(&self, other: &Self) -> bool {
//         // Compare each element of the array
//         self.0.iter().eq(other.0.iter())
//     }
// }

impl Default for Email {
    fn default() -> Self {
        Email([' '; 255])
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Row {
    pub id: u32,
    pub username: Username,
    pub email: Email
}