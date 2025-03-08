use dirs::home_dir;
use std::{
    fs::{exists, read_dir},
    path::PathBuf,
};

fn firefox_dir() -> Option<PathBuf> {
    let mut path = home_dir()?;
    if cfg!(any(target_os = "linux", target_os = "macos")) {
        if cfg!(target_os = "linux") {
            path.push(".mozilla/firefox/");
        } else {
            path.push("Library/Application Support/Firefox/Profiles/");
        }
        if exists(&path).unwrap() {
            Some(path)
        } else {
            None
        }
    } else if cfg!(target_os = "windows") {
        path.push(r"AppData\Roaming\Mozilla\Firefox\Profiles\");
        if exists(&path).unwrap() {
            Some(path)
        } else {
            path = home_dir()?;
            path.push(r"AppData\Local\Packages\");
            for entry in read_dir(path).unwrap() {
                let entry = entry.unwrap();
                if entry
                    .file_name()
                    .as_os_str()
                    .to_str()
                    .map(|name| name.starts_with("Mozilla.Firefox"))
                    .unwrap_or_default()
                {
                    let mut path = entry.path();
                    path.push(r"LocalCache\Roaming\Mozilla\Firefox\Profiles\");
                    return Some(path);
                }
            }
            None
        }
    } else {
        None
    }
}
pub fn firefox_profile() -> Option<PathBuf> {
    let path = firefox_dir()?;
    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        println!("{:?}", entry);
        if entry
            .file_name()
            .as_os_str()
            .to_str()
            .map(|name| name.ends_with(".default-release"))
            .unwrap_or_default()
        {
            return Some(entry.path());
        }
    }
    None
}
