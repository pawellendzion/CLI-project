use std::{path::PathBuf, env, fs};

use crate::error::*;

pub fn get_home_dir() -> Result<PathBuf> {
    let mut home_dir = PathBuf::new();

    if cfg!(windows) {
        home_dir.push(env::var("USERPROFILE")?);
        return Ok(home_dir);
    } 

    home_dir.push("/home");

    if let Ok(value) = env::var("SUDO_USER") {
        home_dir.push(&value);
        if fs::metadata(&home_dir).is_ok() {
            return Ok(home_dir);
        } else {
            home_dir.pop();
        }
    }

    if let Ok(value) = env::var("USER") {
        home_dir.push(&value);
        if fs::metadata(&home_dir).is_ok() {
            return Ok(home_dir);
        }     
    }

    let home_dir = PathBuf::from(env::var("HOME")?);

    Ok(home_dir)
}

pub fn get_local_data_dir() -> Result<PathBuf> {
    let mut data_dir = get_home_dir()?;

    if cfg!(windows) {
        data_dir.push("AppData/Roaming");
    } else {
        data_dir.push(".local/share");
    }

    fs::metadata(&data_dir)?;

    Ok(data_dir)
}

pub fn get_app_data_dir_and_create_if_not_exist() -> Result<PathBuf> {
    let mut app_data_dir = get_local_data_dir()?;

    app_data_dir.push("my-cli-rust-app-data");

    if let Err(_) = fs::metadata(&app_data_dir) {
        fs::create_dir_all(&app_data_dir)?;
    }

    Ok(app_data_dir)
}
