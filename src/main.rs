use clap::Parser;
use std::process;

mod cli;
mod client;
mod commands;
mod config;
mod error;
mod models;
mod output;
mod request;
mod response;
mod services;
mod utils;
mod validation;

pub mod auth;
pub mod formatters;

use cli::CliArgs;
use error::Result;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let args = CliArgs::parse();
    
    match run(args).await {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

async fn run(args: CliArgs) -> Result<()> {
    match args.command {
        cli::Command::List(opts) => commands::list::execute(opts).await,
        cli::Command::Call(opts) => commands::call::execute(opts).await,
    }
}