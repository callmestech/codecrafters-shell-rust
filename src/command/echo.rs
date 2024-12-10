use crate::algebra::CommandExt;

#[derive(Debug, Clone, Default)]
pub struct Echo;

impl CommandExt for Echo {
    fn execute(&self, args: &[&str], _path: &[String]) -> anyhow::Result<()> {
        if args.len() > 1 {
            // let mut single_quotes = Vec::new();
            println!("{}", args[1..].join(" "));
        }
        Ok(())
    }
}
