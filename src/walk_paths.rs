use crate::write::write;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn from<P: AsRef<Path>>(path: P) -> WalkPaths {
    new(path.as_ref().to_path_buf())
}

pub fn new(path: PathBuf) -> WalkPaths {
    WalkPaths {
        entry_path: path.to_owned(),
        entries: WalkDir::new(path)
            .into_iter()
            .filter_map(|e| {
                if let Ok(dir_entry) = e {
                    if !dir_entry.path().is_dir() {
                        return Some(dir_entry);
                    }
                }
                None
            })
            .collect(),
        cast: Cast::Str,
    }
}

#[derive(Debug)]
pub enum Cast {
    Str,
    Bytes,
}

#[derive(Debug)]
pub struct WalkPaths {
    entry_path: PathBuf,
    entries: Vec<DirEntry>,
    cast: Cast,
}

impl WalkPaths {
    pub fn filter<P>(self, predicate: P) -> WalkPaths
    where
        P: FnMut(&DirEntry) -> bool,
    {
        WalkPaths {
            entries: self.entries.into_iter().filter(predicate).collect(),
            ..self
        }
    }

    pub fn bytes(self) -> WalkPaths {
        WalkPaths {
            cast: Cast::Bytes,
            ..self
        }
    }

    pub fn str(self) -> WalkPaths {
        WalkPaths {
            cast: Cast::Str,
            ..self
        }
    }

    pub fn to<P: AsRef<Path>>(self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref().to_path_buf();
        match self.cast {
            Cast::Str => write(path, String::from("include_str"), self.entries),
            Cast::Bytes => write(path, String::from("include_bytes"), self.entries),
        }
    }
}
