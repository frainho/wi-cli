mod cli;
mod events;
mod results_state;
mod ui;

use anyhow::Result;
use cli::{Command, SourcesSubcommand, Wicli};
use config::config::Config;
use config::sources::Sources;
use events::Events;
use results_state::ResultsState;
use search::Search;
use structopt::StructOpt;
use ui::UI;

fn main() -> Result<()> {
    let opt = Wicli::from_args();
    let mut config = Config::default();

    match opt.commands {
        Command::Sources(sources) => {
            let sources_manager = Sources::default();
            match sources {
                SourcesSubcommand::Add { path_or_url } => {
                    sources_manager.add(&mut config, &path_or_url)?
                }
                SourcesSubcommand::List => sources_manager.list(&config)?,
                SourcesSubcommand::Remove { path } => sources_manager.remove(&mut config, path)?,
            };
        }
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
