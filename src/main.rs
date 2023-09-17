use std::io;
use std::process::exit;

enum MetaCommandResult {
    SUCCESS,
    UNRECOGNIZED
}

enum PrepareResult {
    SUCCESS,
    UNRECOGNIZED
}

fn main() {

    let stdin = io::stdin(); // binding a handle.

    loop // same as while(true)
    {
        let mut input = String::new();

        println!("Please enter the SQL command: ");
        stdin.read_line(&mut input).expect("Failed to read line.");
        input = input.trim().to_string(); // remove trailing newline.

        if input.starts_with('.') {
            execute_command(input);
        }
    }
}

fn execute_command(cmd: String) -> MetaCommandResult {
    if cmd == ".exit"
    {
        exit(0);
    }
    else
    {
        println!("Unrecognized command: {}", cmd);
        return MetaCommandResult::UNRECOGNIZED;
    }
}