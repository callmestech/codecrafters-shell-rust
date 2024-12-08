use core::fmt;
use std::io::{self, Write};
use std::{
    fmt::Display,
    process::{self},
};

#[derive(Debug, Clone)]
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
                write!(f, "{}: not found", cmd)
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
    let path = read_path_env();
    // Wait for user input
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        let parsed_input: Vec<&str> = input.split_whitespace().collect();
        let cmd: Option<Command> = parsed_input.first().map(|cmd| Command::from(*cmd));

        if let Some(command) = cmd {
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
                        Command::Invalid(cmd) => {
                            let executables = path
                                .iter()
                                .map(|path| read_dir(path))
                                .filter_map(Result::ok)
                                .flatten()
                                .collect::<Vec<_>>();
                            let path_of_cmd = executables
                                .into_iter()
                                .find(|path| path.ends_with(&format!("/{}", &cmd)));

                            if let Some(path) = path_of_cmd {
                                format!("{} is {}", &cmd, path)
                            } else {
                                format!("{}: not found", &cmd)
                            }
                        }
                    };
                    println!("{}", description);
                }
                command => println!("{}", command),
            }
        }
        print!("$ ");
        stdout.flush().unwrap();
    }
}

/// List all files in a directory
fn read_dir(path: &str) -> io::Result<Vec<String>> {
    let entries = std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.path().to_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();
    Ok(entries)
}

/// Read the PATH environment variable and return a vector of paths
fn read_path_env() -> Vec<String> {
    let path = std::env::var("PATH");

    match path {
        Ok(path) => path.split(':').map(|s| s.to_owned()).collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    }
}
