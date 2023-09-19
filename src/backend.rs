use std::ffi::c_void;
use std::mem::size_of;

const ID_SIZE: usize = size_of::<u32>();
const USERNAME_SIZE: usize = size_of::<[char; 32]>();
const EMAIL_SIZE: usize = size_of::<[char; 255]>();
const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[repr(align(4096))]
pub struct Table {
    num_rows: u32,
    pages: [*mut c_void; TABLE_MAX_PAGES]
}