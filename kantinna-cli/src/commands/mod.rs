mod arg_matches;
mod config;

pub use arg_matches::*;

pub trait CommandExt: Sized {
    fn add_builtins(self) -> clap::Command;
}

impl CommandExt for clap::Command {
    fn add_builtins(self) -> Self {
        builtin()
            .iter()
            .fold(self, |cmd, subcmd| cmd.subcommand(subcmd))
    }
}

#[inline]
fn builtin() -> Vec<clap::Command> {
    vec![config::command()]
}
