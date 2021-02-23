use anyhow::Result;
use core::panic;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigMap {
    pub sources: Option<Vec<PathBuf>>,
}

impl Default for ConfigMap {
    fn default() -> Self {
        Self {
            sources: Some(Vec::new()),
        }
    }
}

pub struct Config {
    pub config_map: ConfigMap,
    file_handle: File,
}

impl Config {
    const DEFAULT_CONFIG_NAME: &'static str = "wicli.json";

    fn load_or_create(mut home_dir: PathBuf) -> Result<Self> {
        let config_path = {
            home_dir.push(Self::DEFAULT_CONFIG_NAME);
            home_dir
        };

        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(config_path)?;

        let config_map = serde_json::from_reader(&file).unwrap_or_default();

        Ok(Self {
            config_map,
            file_handle: file,
        })
    }

    pub fn add_source<P: AsRef<Path>>(&mut self, path: P) {
        self.config_map
            .sources
            .as_mut()
            .unwrap()
            .push(path.as_ref().into());
    }
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = match home_dir() {
            Some(home_dir) => home_dir,
            None => panic!("Unable to determine home dir location"),
        };
        match Self::load_or_create(home_dir) {
            Ok(config) => config,
            Err(error) => panic!("Unable to load file: {}", error),
        }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        if let Err(error) = serde_json::to_writer_pretty(&self.file_handle, &self.config_map) {
            panic!("Unable to save config: {}", error);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn it_loads_an_existing_configuration() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join(Config::DEFAULT_CONFIG_NAME);
        let mut file = File::create(file_path)?;
        writeln!(
            file,
            r#"{{
            "sources": ["fake_source"]
        }}"#
        )?;

        let config = Config::load_or_create(dir.into_path())?;

        assert_eq!(
            config.config_map.sources.as_ref().unwrap(),
            &vec![PathBuf::from("fake_source")]
        );
        Ok(())
    }

    #[test]
    fn it_creates_a_configuration_if_doesnt_exist() -> Result<()> {
        let dir = tempdir()?;
        let config = Config::load_or_create(dir.into_path())?;

        assert_eq!(
            config.config_map.sources.as_ref().unwrap(),
            &Vec::<PathBuf>::new()
        );
        Ok(())
    }

    #[test]
    fn it_adds_a_new_source_to_the_configuration() -> Result<()> {
        let dir = tempdir()?;
        let fake_source_name = "path_to_fake_source";
        let mut config = Config::load_or_create(dir.into_path())?;

        config.add_source(fake_source_name);

        assert_eq!(
            config.config_map.sources.as_ref().unwrap(),
            &vec![PathBuf::from(fake_source_name)]
        );

        Ok(())
    }

    #[test]
    fn it_saves_the_file_when_dropping() -> Result<()> {
        let dir = tempdir()?;
        let fake_source_name = "path_to_fake_source";
        let home_dir_path = dir.into_path();
        let mut config = Config::load_or_create(home_dir_path.clone())?;

        config.add_source(fake_source_name);
        drop(config);

        let config = Config::load_or_create(home_dir_path)?;

        assert_eq!(
            config.config_map.sources.as_ref().unwrap(),
            &vec![PathBuf::from(fake_source_name)]
        );

        Ok(())
    }
}
