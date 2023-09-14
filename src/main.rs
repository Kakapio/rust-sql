use std::io;
use std::process::exit;

fn main() {

    let mut input = String::new();
    let stdin = io::stdin(); // binding a handle.

    println!("Please enter the SQL command: ");
    stdin.read_line(&mut input).expect("Failed to read line.");
    input = input.trim().to_string(); // remove trailing newline.
    println!("You entered: {}", input);

    if input == ".exit"
    {
        exit(0);
    }
    else
    {
        println!("Unrecognized command: {}", input);
    }
}
