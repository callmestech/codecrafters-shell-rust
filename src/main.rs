use codecrafters_shell::parse_input;
use codecrafters_shell::{
    algebra::CommandExt, command::Command as CommandDispatch, find_cmd_in_path, read_path_env,
};
use std::io::{self, Write};
use std::process::Command as StdCommand;
use std::str::FromStr;

fn main() -> Result<(), anyhow::Error> {
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

        let parsed_input = parse_input(input);
        let parsed_input = parsed_input.iter().map(|s| s.as_ref()).collect::<Vec<_>>();

        let cmd: Option<CommandDispatch> = parsed_input
            .first()
            .and_then(|cmd| CommandDispatch::from_str(cmd).ok());

        if let Some(cmd) = cmd {
            cmd.execute(&parsed_input, &path)?;
        } else {
            let cmd = parsed_input.first().unwrap();
            let path_of_cmd = find_cmd_in_path(cmd, &path);
            if let Some(path) = path_of_cmd {
                let mut command = StdCommand::new(&path);
                if parsed_input.len() > 1 {
                    command.args(&parsed_input[1..]);
                }
                command.status()?;
            } else {
                println!("{}: not found", cmd);
            }
        }

        print!("$ ");
        stdout.flush().unwrap();
    }
}
