use std::{
    fs::{self, File},
    io::Write,
};

use tempfile::{tempdir, TempDir};

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
