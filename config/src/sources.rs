use std::path::Path;

use anyhow::Result;

use crate::config::Config;

pub struct Sources;

impl Sources {
    pub fn add<P: AsRef<Path>>(config: &mut Config, path: P) -> Result<()> {
        config.add_source(path)?;

        Ok(())
    }

    pub fn list(config: &Config) -> Result<()> {
        println!("List of sources");
        for source in config.config_map.sources.iter() {
            println!("{}", source.display());
        }

        Ok(())
    }

    pub fn remove<P: AsRef<Path>>(config: &mut Config, path: P) -> Result<()> {
        config.delete_source(path)?;

        Ok(())
    }
}
