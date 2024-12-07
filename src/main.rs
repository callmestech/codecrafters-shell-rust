#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("$ ");
    stdout.flush().unwrap();

    // Wait for user input
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "exit 0" => process::exit(0),
            _ => println!("{}: command not found", input),
        }
        print!("$ ");
        stdout.flush().unwrap();
    }
}
