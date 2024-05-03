use log::error;
use owo_colors::colored::*;

use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
};

pub fn check_create_config_dir() -> Result<PathBuf> {
    let mut new_dir = PathBuf::new();
    match dirs::config_dir() {
        Some(config_dir) => {
            new_dir.push(config_dir);
            new_dir.push("knb");
            if !new_dir.as_path().exists() {
                fs::create_dir(&new_dir)?;
            }
        }
        None => {
            error!("Unable to find config directory");
        }
    }

    Ok(new_dir)
}

pub fn create_config_file(config_dir: &str) -> Result<PathBuf> {
    todo!();
}

pub fn read_config_file(config_dir: &str) -> Result<String> {
    todo!();
}

pub fn show_log_file(config_dir: &PathBuf) -> Result<String> {
    let log_path = Path::new(&config_dir).join("knb.log");
    return match log_path.try_exists()? {
        true => Ok(format!(
            "{} {}\n{}",
            "Log location:".italic().dimmed(),
            &log_path.display(),
            fs::read_to_string(&log_path)?
        )),
        false => Ok(format!(
            "{} {}",
            "No log file found:"
                .truecolor(250, 0, 104)
                .bold()
                .to_string(),
            log_path.display()
        )),
    };
}
