use std::path::Path;

use anyhow::Result;

use crate::config::Config;
pub(crate) struct Sources;

impl Sources {
    pub fn add<P: AsRef<Path>>(path: P) -> Result<()> {
        let mut config = Config::default();
        config.add_source(path);

        Ok(())
    }

    pub fn list() -> Result<()> {
        let config = Config::default();

        println!("List of sources");
        for source in config.config_map.sources.as_ref().unwrap().iter() {
            println!("{}", source.display());
        }

        Ok(())
    }
}
