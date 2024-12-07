#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self};

enum Command {
    Exit,
    Echo,
    Invalid,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "exit" => Command::Exit,
            "echo" => Command::Echo,
            _ => Command::Invalid,
        }
    }
}

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
        let parsed_input: Vec<&str> = input.split_whitespace().collect();
        let command = Command::from(parsed_input[0]);
        match command {
            Command::Exit => process::exit(0),
            Command::Echo => {
                if parsed_input.len() > 1 {
                    println!("{}", parsed_input[1..].join(" "));
                }
            }
            Command::Invalid => println!("{}: command not found", input),
        }
        print!("$ ");
        stdout.flush().unwrap();
    }
}
