use crate::{algebra::CommandExt, find_cmd_in_path};

#[derive(Debug, Clone, Default)]
pub struct Invalid;

impl CommandExt for Invalid {
    fn execute(&self, args: &[&str], path: &[String]) -> anyhow::Result<()> {
        let path_of_cmd = find_cmd_in_path(args[0], path);
        if let Some(path) = path_of_cmd {
            println!("{} is {}", args[0], path);
        } else {
            println!("{}: not found", args[0]);
        }
        Ok(())
    }
}
