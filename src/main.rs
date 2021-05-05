mod cli;
mod config;
mod events;
mod results_state;
mod search;
mod sources;
mod ui;

use anyhow::Result;
use cli::{Command, SourcesSubcommand, Wicli};
use config::Config;
use events::Events;
use results_state::ResultsState;
use search::Search;
use sources::Sources;
use structopt::StructOpt;
use ui::UI;

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
            let results = Search::by_term(&config, term)?;

            let mut ui = UI::default();
            let mut results_state = ResultsState::from_results(&results);
            loop {
                ui.draw(&mut results_state)?;
                let should_exit = Events::read(&mut results_state)?;
                if should_exit {
                    break;
                }
            }
        }
    }

    Ok(())
}
