use anyhow::Result;
use core::panic;
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
    const DEFAULT_PATH: &'static str = ".config/wicli.json";

    pub fn create_or_load<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config_path = match home::home_dir() {
            Some(mut home_dir) => {
                home_dir.push(config_path);
                home_dir
            }
            None => {
                panic!("Unable to determine home path")
            }
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
        match Self::create_or_load(Self::DEFAULT_PATH) {
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
