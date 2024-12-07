#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "exit" => break,
            _ => println!("{}: command not found", input),
        }
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
