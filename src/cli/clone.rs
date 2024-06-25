use clap::Args;

use crate::cli::Error;

#[derive(Args)]
pub struct CloneArgs {
    /// The repository to clone.
    repository: String,
}

pub fn run(args: CloneArgs) -> Result<(), Error> {
    Ok(())
}