use std::{path::PathBuf, process::Command};

use home::home_dir;
use url::Url;

pub trait GitClient {
    fn clone(&self, url: Url) -> PathBuf;
}

pub struct DefaultGitClient;

impl GitClient for DefaultGitClient {
    fn clone(&self, url: Url) -> PathBuf {
        let path = Self::get_path(&url);
        println!("Clonning repository");
        Command::new("git")
            .args(&[
                "clone",
                "--depth",
                "1",
                url.as_str(),
                &path.to_string_lossy(),
            ])
            .output()
            .expect("Unable to clone git repository");

        path
    }
}

impl DefaultGitClient {
    fn get_path(url: &Url) -> PathBuf {
        let mut path = match home_dir() {
            Some(path) => path,
            None => panic!("Unable to determine your home directory"),
        };
        let folder_path = url.path().trim_start_matches('/').replace('/', "-");
        path.push(".wicli");
        path.push(folder_path);
        path
    }
}

pub struct MockGitClient;
pub const EXAMPLE_GIT_PATH: &str = "anything.git";

impl GitClient for MockGitClient {
    fn clone(&self, _url: Url) -> PathBuf {
        PathBuf::from(EXAMPLE_GIT_PATH)
    }
}
