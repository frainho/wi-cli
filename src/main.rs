mod cli;
mod config;
mod sources;

use anyhow::Result;
use cli::{Command::Sources, Sources as SourcesSubcommand, Wicli};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Wicli::from_args();

    match opt.commands {
        Sources(sources) => match sources {
            SourcesSubcommand::Add { path } => sources::Sources::add(path)?,
            SourcesSubcommand::List => sources::Sources::list()?,
            SourcesSubcommand::Remove { path } => sources::Sources::remove(path)?,
        },
    }

    Ok(())
}
