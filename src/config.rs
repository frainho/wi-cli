use anyhow::{anyhow, Result};
use core::panic;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::Read,
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
    config_path: PathBuf,
}

impl Config {
    const DEFAULT_CONFIG_NAME: &'static str = "wicli.json";

    fn load_or_create(mut home_dir: PathBuf) -> Result<Self> {
        let config_path = {
            home_dir.push(Self::DEFAULT_CONFIG_NAME);
            home_dir
        };

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&config_path)?;

        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let config_map = serde_json::from_str(&data).unwrap_or_default();

        Ok(Self {
            config_map,
            config_path,
        })
    }

    pub fn add_source<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.validate_source(path.as_ref())?;
        self.config_map
            .sources
            .as_mut()
            .unwrap()
            .push(path.as_ref().into());

        Ok(())
    }

    pub fn delete_source<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let index = self
            .config_map
            .sources
            .as_ref()
            .unwrap()
            .iter()
            .position(|source_path| source_path == path.as_ref());

        if let Some(index) = index {
            self.config_map.sources.as_mut().unwrap().remove(index);
        }

        Ok(())
    }

    fn validate_source<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if let false = path.as_ref().is_dir() {
            return Err(anyhow!("Source is not a valid directory or does not exist"));
        }

        let source_already_added = self
            .config_map
            .sources
            .as_ref()
            .unwrap()
            .iter()
            .any(|source_path| source_path == path.as_ref());

        if let true = source_already_added {
            return Err(anyhow!("Source already exists"));
        }

        Ok(())
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
        let result = match serde_json::to_string_pretty(&self.config_map) {
            Ok(result) => result,
            Err(error) => panic!("Unable to parse config: {}", error),
        };

        if let Err(error) = std::fs::write(&self.config_path, &result) {
            panic!("Unable to save config: {}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
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
        let mut config = Config::load_or_create(dir.path().to_path_buf())?;

        config.add_source(&dir)?;

        assert_eq!(
            config.config_map.sources.as_ref().unwrap(),
            &vec![PathBuf::from(dir.path())]
        );

        Ok(())
    }

    #[test]
    #[should_panic(expected = "Source is not a valid directory or does not exist")]
    fn it_errors_when_the_directory_is_invalid() {
        let dir = tempdir().unwrap();
        let mut config = Config::load_or_create(dir.path().to_path_buf()).unwrap();

        config.add_source("invalid dir").unwrap();
    }

    #[test]
    #[should_panic(expected = "Source already exists")]
    fn it_errors_when_the_source_already_exists() {
        let dir = tempdir().unwrap();
        let mut fake_source_path = dir.path().to_path_buf();
        fake_source_path.push("fake_source");
        fs::create_dir(&fake_source_path).unwrap();

        let mut config = Config::load_or_create(dir.path().to_path_buf()).unwrap();

        config.add_source(&fake_source_path).unwrap();
        config.add_source(fake_source_path).unwrap();
    }

    #[test]
    fn it_saves_the_file_when_dropping() -> Result<()> {
        let dir = tempdir()?;
        let home_dir_path = dir.path().to_path_buf();
        let mut config = Config::load_or_create(home_dir_path.clone())?;

        config.add_source(home_dir_path.clone())?;
        drop(config);

        let config = Config::load_or_create(home_dir_path.clone())?;

        assert_eq!(
            config.config_map.sources.as_ref().unwrap(),
            &vec![home_dir_path]
        );

        Ok(())
    }

    #[test]
    fn it_removes_a_source_from_the_list() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join(Config::DEFAULT_CONFIG_NAME);
        let mut file = File::create(file_path)?;
        let fake_source_1 = "fake_source";
        let fake_source_2 = "fake_source_2";
        writeln!(
            file,
            r#"{{
            "sources": ["{}", "{}"]
        }}"#,
            fake_source_1, fake_source_2
        )?;
        let mut config = Config::load_or_create(dir.into_path())?;

        config.delete_source("fake_source")?;

        assert_eq!(
            config.config_map.sources.as_ref().unwrap(),
            &vec![PathBuf::from(fake_source_2)]
        );

        Ok(())
    }
}
