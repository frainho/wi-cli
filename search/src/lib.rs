use core::panic;
use std::{fs::File, io::Read};

use anyhow::Result;
use config::config::Config;
use jwalk::{Parallelism, WalkDir};
use rayon::prelude::*;
pub struct Search;

impl Search {
    pub fn by_term(config: &Config, term: String) -> Result<Vec<String>> {
        let sources_list = &config.config_map.sources;
        if !sources_list.is_empty() {
            let files_with_term = sources_list
                .par_iter()
                .flat_map(|source_path| {
                    println!("Walking source: {:?}", source_path);
                    WalkDir::new(source_path)
                        .parallelism(Parallelism::RayonNewPool(5)) // Needs to use a different thread pool to prevent any livelocks, 5 is an arbitrary number
                        .into_iter()
                        .par_bridge()
                })
                .filter_map(|result| result.ok())
                .filter(|dir_entry| match dir_entry.metadata() {
                    Ok(metadata) => !metadata.is_dir(),
                    Err(_) => false,
                })
                .map(|dir_entry| {
                    let mut file = match File::open(dir_entry.path()) {
                        Ok(file) => file,
                        Err(error) => {
                            eprintln!(
                                "Unable to read file: {:?}, due to {:?}",
                                dir_entry.path(),
                                error
                            );
                            return String::new();
                        }
                    };
                    let mut file_contents = String::new();
                    match file.read_to_string(&mut file_contents) {
                        Ok(_) => {}
                        Err(error) => {
                            eprintln!(
                                "Unable to read file: {:?}, due to {:?}",
                                dir_entry.path(),
                                error
                            );
                            return String::new();
                        }
                    };
                    file_contents
                })
                .filter(|file_contents| file_contents.contains(&term))
                .collect::<Vec<String>>();

            Ok(files_with_term)
        } else {
            panic!("No available sources to search through")
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test_utils;
    use super::*;
    use anyhow::Result;
    use config::config::Configuration;
    use tempfile::tempdir;
    use test_utils::create_fake_source;

    #[test]
    fn it_returns_the_results() -> Result<()> {
        let source = create_fake_source()?;
        let fake_config_dir = tempdir()?;
        let mut config = Config::load_or_create(fake_config_dir.into_path())?;
        config.add_source(source.path())?;

        let result = Search::by_term(&config, "d.txt.test".to_string())?;

        assert_eq!(result, vec!["d.txt.test", "d.txt.test", "d.txt.test"]);
        Ok(())
    }

    #[test]
    fn it_returns_an_empty_vec_when_there_are_no_results() -> Result<()> {
        let source = create_fake_source()?;
        let fake_config_dir = tempdir()?;
        let mut config = Config::load_or_create(fake_config_dir.into_path())?;
        config.add_source(source.path())?;

        let result = Search::by_term(&config, "Can't find this".to_string())?;

        assert_eq!(result, Vec::<String>::new());
        Ok(())
    }

    #[test]
    #[should_panic(expected = "No available sources to search through")]
    fn it_panics_when_there_are_no_sources() {
        let fake_config_dir = tempdir().unwrap();
        let config = Config::load_or_create(fake_config_dir.into_path()).unwrap();

        Search::by_term(&config, "test".to_string()).unwrap();
    }
}
