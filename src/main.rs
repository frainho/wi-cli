mod cli;
mod config;
mod sources;

use anyhow::Result;
use cli::{Command, SourcesSubcommand, Wicli};
use config::Config;
use sources::Sources;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Wicli::from_args();
    let mut config = Config::default();

    match opt.commands {
        Command::Sources(sources) => match sources {
            SourcesSubcommand::Add { path } => Sources::add(&mut config, path)?,
            SourcesSubcommand::List => Sources::list(&config)?,
            SourcesSubcommand::Remove { path } => Sources::remove(&mut config, path)?,
        },
        Command::Search { term } => {
            println!("{}", term)
        }
    }

    Ok(())
}
