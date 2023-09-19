use std::mem::size_of;

const ID_SIZE: usize = size_of::<u32>();
const USERNAME_SIZE: usize = size_of::<u32>();


const PAGE_SIZE: usize = 4096;

#[derive(PartialEq, Debug, Default)]
pub struct Table {

}