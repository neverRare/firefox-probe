use dirs::home_dir;
use std::{
    fs::{exists, read_dir},
    path::PathBuf,
};

#[derive(Debug, Clone, Copy)]
pub struct Rank<T, U> {
    pub value: T,
    pub rank: U,
}
impl<T, U> PartialEq for Rank<T, U>
where
    U: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}
impl<T, U> Eq for Rank<T, U> where U: Eq {}
impl<T, U> PartialOrd for Rank<T, U>
where
    U: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}
impl<T, U> Ord for Rank<T, U>
where
    U: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}
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
pub fn firefox_places() -> Option<impl Iterator<Item = PathBuf>> {
    let path = firefox_dir()?;
    Some(read_dir(path).unwrap().filter_map(|entry| {
        let entry = entry.unwrap();
        if entry
            .file_name()
            .as_os_str()
            .to_str()
            .map(|name| name.contains("default"))
            .unwrap_or_default()
        {
            let mut path = entry.path();
            path.push("places.sqlite");
            if exists(&path).unwrap() {
                Some(path)
            } else {
                None
            }
        } else {
            None
        }
    }))
}
