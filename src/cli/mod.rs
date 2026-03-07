use clap::Subcommand;

pub mod args;
pub use args::CliArgs;

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    List(crate::commands::ListArgs),
    Call(crate::commands::CallArgs),
}