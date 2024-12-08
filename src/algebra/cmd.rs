use enum_dispatch::enum_dispatch;

/// The trait that defines the behavior of a command.
#[enum_dispatch]
pub trait CommandExt {
    fn execute(&self, args: &[&str], path: &[String]) -> anyhow::Result<()>;
}
