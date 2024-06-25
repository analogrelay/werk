use clap::Args;

use crate::cli::Error;

#[derive(Args, Clone)]
pub struct OnArgs {
    /// The pull request or branch to work on.
    pub subject: String,
}

pub fn run(args: OnArgs) -> Result<(), Error> {
    Ok(())
}