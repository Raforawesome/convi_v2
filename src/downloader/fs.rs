use std::fs::{self, File};
use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    let data_dir: PathBuf = home_dir.join(".convi");
    if !data_dir.exists() {
        let _ = std::fs::create_dir(&data_dir);
    }
    data_dir
}

pub fn get_or_create_dir(dir_name: &str) -> Result<PathBuf, std::io::Error> {
    let mut data_dir = get_data_dir();
    data_dir.push(dir_name);
    if !data_dir.exists() {
        let _ = fs::create_dir(&data_dir)?;
    }
    Ok(data_dir)
}

pub fn get_or_create_file(file_name: &str) -> Result<File, std::io::Error> {
    let mut data_dir = get_data_dir();
    data_dir.push(file_name);
    if fs::try_exists(&data_dir).is_err() {
        let _ = fs::write(&data_dir, []);
    }
    File::open(&data_dir)
}
