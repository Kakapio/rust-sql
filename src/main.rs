use crate::backend::*;

/* All our modules must be imported into either our main.rs or lib.rs to 'unite' them for the compiler.
 * They cannot be imported into submodules, e.g backend.rs cannot do 'mod parser' as it would then expect
 * to find a directory called /backend/parser.rs or /backend/parser/mod.rs.
 */
mod backend;
mod parser;
mod tests {
    mod unit_tests;
}

fn main() {
    entrypoint();
}
