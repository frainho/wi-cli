use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Result;
use url::Url;

use crate::{
    config::Configuration,
    git::{DefaultGitClient, GitClient},
};

pub struct Sources<GC> {
    git_client: GC,
}

impl<GC: GitClient> Sources<GC> {
    pub fn add<C: Configuration>(&self, config: &mut C, path: &str) -> Result<()> {
        let source = Source::from_str(path)?;
        let path = match source.path {
            SourcePath::Git(url) => self.git_client.clone(url),
            SourcePath::Local(path) => path,
        };
        config.add_source(path)?;

        Ok(())
    }

    pub fn list<C: Configuration>(&self, config: &C) -> Result<()> {
        println!("List of sources");
        for source in config.get_sources().iter() {
            println!("{}", source.display());
        }

        Ok(())
    }

    pub fn remove<P: AsRef<Path>, C: Configuration>(&self, config: &mut C, path: P) -> Result<()> {
        config.delete_source(path)?;

        Ok(())
    }
}

impl Default for Sources<DefaultGitClient> {
    fn default() -> Self {
        Sources {
            git_client: DefaultGitClient,
        }
    }
}

#[derive(Debug)]
enum SourcePath {
    Git(Url),
    Local(PathBuf),
}

#[derive(Debug)]
pub struct Source {
    path: SourcePath,
}

impl FromStr for Source {
    type Err = anyhow::Error;

    fn from_str(source_path: &str) -> Result<Self, Self::Err> {
        match source_path.ends_with(".git") {
            true => Ok(Source {
                path: SourcePath::Git(Url::parse(source_path)?),
            }),
            false => Ok(Source {
                path: SourcePath::Local(PathBuf::from(source_path)),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_GIT_PATH: &str = "anything.git";

    struct MockGitClient;

    impl GitClient for MockGitClient {
        fn clone(&self, _url: Url) -> PathBuf {
            PathBuf::from(EXAMPLE_GIT_PATH)
        }
    }

    #[derive(Debug)]
    struct MockConfiguration {
        source_added: Option<PathBuf>,
    }

    impl Configuration for MockConfiguration {
        fn add_source<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
            self.source_added = Some(path.as_ref().to_path_buf());
            Ok(())
        }

        fn delete_source<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
            todo!()
        }

        fn get_sources(&self) -> &Vec<PathBuf> {
            todo!()
        }

        const DEFAULT_CONFIG_NAME: &'static str = "hello";
    }

    #[test]
    fn it_adds_git_sources() {
        let sources = Sources {
            git_client: MockGitClient,
        };
        let mut config = MockConfiguration { source_added: None };
        let fake_git_url = format!("http://example.com/${}", EXAMPLE_GIT_PATH);

        sources.add(&mut config, &fake_git_url).unwrap();

        assert_eq!(config.source_added, Some(PathBuf::from(EXAMPLE_GIT_PATH)));
    }

    #[test]
    fn it_adds_a_new_local_source() {
        let sources = Sources {
            git_client: MockGitClient,
        };
        let mut config = MockConfiguration { source_added: None };
        let local_source = "local/path";

        sources.add(&mut config, local_source).unwrap();

        assert_eq!(config.source_added, Some(PathBuf::from(local_source)));
    }
}
