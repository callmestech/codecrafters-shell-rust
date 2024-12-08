mod cd;
mod echo;
mod exit;
mod invalid;
mod pwd;
mod r#type;

use self::{cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, r#type::Type};
use crate::algebra::CommandExt;
use enum_dispatch::enum_dispatch;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
#[enum_dispatch(CommandExt)]
pub enum Command {
    Cd(Cd),
    Exit(Exit),
    Echo(Echo),
    // #[strum(to_string = "{0} is not found")]
    // #[strum(default)]
    // Invalid(Invalid),
    Pwd(Pwd),
    Type(Type),
}
