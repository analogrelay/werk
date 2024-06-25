use std::process::ExitCode;

use clap::{Parser, Subcommand};
use thiserror::Error;

mod on;
mod clone;

#[derive(Debug, Error)]
enum Error {
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: WerkCommand,
}

#[derive(Subcommand)]
enum WerkCommand {
    #[command(arg_required_else_help = true)]
    Clone(clone::CloneArgs),

    #[command(arg_required_else_help = true)]
    On(on::OnArgs),
}

pub fn run() -> ExitCode {
    let args = Args::parse();
    let result = match args.command {
        WerkCommand::Clone(args) => clone::run(args),
        WerkCommand::On(args) => on::run(args),
    };
    ExitCode::SUCCESS
}