use core::fmt;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    fmt::Display,
    process::{self},
};

enum Command {
    Exit,
    Echo,
    Invalid(String),
    Type,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "exit" => Command::Exit,
            "echo" => Command::Echo,
            "type" => Command::Type,
            command => Command::Invalid(command.to_string()),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Exit => write!(f, "exit"),
            Command::Echo => write!(f, "echo"),
            Command::Invalid(command) => {
                let cmd = command.to_string();
                write!(f, "{}", format!("{}: not found", cmd))
            }
            Command::Type => write!(f, "type"),
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
        let command: Command = parsed_input[0].into();
        match command {
            Command::Exit => process::exit(0),
            Command::Echo => {
                if parsed_input.len() > 1 {
                    println!("{}", parsed_input[1..].join(" "));
                }
            }
            Command::Type => {
                let command_to_describe: Command = parsed_input[1].into();
                let description = match command_to_describe {
                    Command::Exit | Command::Echo | Command::Type => {
                        format!("{} is a shell builtin", command_to_describe)
                    }
                    command => command.to_string(),
                };
                println!("{}", description);
            }
            command => println!("{}", command),
        }
        print!("$ ");
        stdout.flush().unwrap();
    }
}
