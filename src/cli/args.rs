use clap::Parser;
use crate::cli::Command;

#[derive(Parser, Debug)]
#[command(name = "tracker")]
#[command(about = "A Rust CLI tool for calling and interacting with a REST API", long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}