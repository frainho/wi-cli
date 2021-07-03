use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}};

use tempfile::{tempdir, TempDir};
use anyhow::Result;

use config::Configuration;

pub fn create_fake_source() -> anyhow::Result<TempDir> {
    let tempdir = tempdir()?;
    let folders = ["a", "b", "c"];
    let files = ["d.txt", "e.txt", "f.txt"];
    for folder in folders.iter() {
        for file in files.iter() {
            fs::create_dir_all(tempdir.path().join(folder))?;
            let mut file_handle = File::create(tempdir.path().join(folder).join(file))?;
            write!(file_handle, "{}.test", file)?;
        }
    }

    Ok(tempdir)
}

#[derive(Debug)]
pub struct MockConfiguration {
    pub source_added: Option<PathBuf>,
}

impl Configuration for MockConfiguration {
    fn add_source<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.source_added = Some(path.as_ref().to_path_buf());
        Ok(())
    }

    fn delete_source<P: AsRef<Path>>(&mut self, _path: P) -> Result<()> {
        todo!()
    }

    fn get_sources(&self) -> &Vec<PathBuf> {
        todo!()
    }

    const DEFAULT_CONFIG_NAME: &'static str = "hello";
}
