use std::str::FromStr;

use crate::{algebra::CommandExt, command::Command, find_cmd_in_path};

#[derive(Debug, Clone, Default)]
pub struct Type;

impl CommandExt for Type {
    fn execute(&self, args: &[&str], path: &[String]) -> anyhow::Result<()> {
        let command_to_describe: Option<Command> = Command::from_str(args[1]).ok();
        let description = match command_to_describe {
            Some(cmd) => format!("{} is a shell builtin", cmd),
            None => {
                let path_of_cmd = find_cmd_in_path(args[1], path);
                if let Some(path) = path_of_cmd {
                    format!("{} is {}", &args[1], path)
                } else {
                    format!("{}: not found", &args[1])
                }
            }
        };
        println!("{}", description);
        Ok(())
    }
}
