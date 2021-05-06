use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "wicli", about = "Find it")]
pub struct Wicli {
    #[structopt(subcommand)]
    pub commands: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(about = "Manage your file sources")]
    Sources(SourcesSubcommand),
    #[structopt(about = "Search your sources")]
    Search { term: String },
}

#[derive(Debug, StructOpt)]
pub enum SourcesSubcommand {
    #[structopt(about = "Add a new file source")]
    Add {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
    #[structopt(about = "List all sources")]
    List,
    Remove {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
}
