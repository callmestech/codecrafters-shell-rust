use crate::algebra::CommandExt;

#[derive(Debug, Clone, Default)]
pub struct Exit;

impl CommandExt for Exit {
    fn execute(&self, _args: &[&str], _path: &[String]) -> anyhow::Result<()> {
        std::process::exit(0);
    }
}
