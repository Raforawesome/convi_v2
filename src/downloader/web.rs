use crate::downloader;
use reqwest::blocking as reqwest;
use std::io::Write;

pub fn get_data_from_url(url: &str) -> Result<Vec<u8>, anyhow::Error> {
    let data = reqwest::get(url);
    if let Ok(res) = data {
        Ok(res.bytes()?.into())
    } else {
        Err(anyhow::Error::msg("Error in fetching URL."))
    }
}

pub fn download_file(url: &str, name: &str) -> Result<(), anyhow::Error> {
    let mut file = downloader::fs::get_or_create_file(name)?;
    let res = file.write(&get_data_from_url(url)?);
    if let Ok(_) = res {
        Ok(())
    } else {
        Err(anyhow::Error::new(res.unwrap_err()))
    }
}
