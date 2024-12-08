use crate::algebra::CommandExt;

#[derive(Debug, Default, Clone)]
pub struct Cd {}

impl CommandExt for Cd {
    fn execute(&self, args: &[&str], _path: &[String]) -> anyhow::Result<()> {
        if args.len() > 1 {
            let mut new_dir = args[1].to_string();
            if new_dir.starts_with('~') {
                let home = std::env::var("HOME")?;
                new_dir = new_dir.replacen("~", &home, 1);
            }
            let exists = std::path::Path::new(&new_dir).exists();
            if exists {
                std::env::set_current_dir(new_dir)?;
            } else {
                println!("cd: {}: No such file or directory", new_dir);
            }
        }

        Ok(())
    }
}
