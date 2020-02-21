use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct WalkPaths {
    entries: Vec<DirEntry>,
    method: String,
}

impl WalkPaths {
    fn new(path: PathBuf) -> WalkPaths {
        WalkPaths {
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
            method: String::from("assets"),
        }
    }

    pub fn filter<P>(self, predicate: P) -> WalkPaths
    where
        P: FnMut(&DirEntry) -> bool,
    {
        WalkPaths {
            entries: self.entries.into_iter().filter(predicate).collect(),
            ..self
        }
    }
    pub fn method<M: AsRef<str>>(self, method: M) -> WalkPaths {
        WalkPaths {
            method: method.as_ref().to_string(),
            ..self
        }
    }

    pub fn bytes(self) -> To {
        To::Bytes(self)
    }

    pub fn str(self) -> To {
        To::Str(self)
    }
}

#[derive(Debug)]
pub enum To {
    Str(WalkPaths),
    Bytes(WalkPaths),
}

pub trait ToWalkPaths<T>: Sized {
    fn new(_: T) -> WalkPaths;
}

impl To {
    pub fn to<P: AsRef<Path>>(self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref().to_path_buf();
        match self {
            To::Str(walk_paths) => write(path, String::from("include_str"), walk_paths),
            To::Bytes(walk_paths) => write(path, String::from("include_bytes"), walk_paths),
        }
    }
}

pub fn walk<P: AsRef<Path>>(path: P) -> WalkPaths {
    WalkPaths::new(path.as_ref().to_path_buf())
}

fn write(path: PathBuf, stmacro: String, walk_paths: WalkPaths) -> Result<(), Box<dyn Error>> {
    let mut all_the_files = File::create(path)?;

    writeln!(&mut all_the_files, r#"use utils::asset::Asset;"#,)?;
    writeln!(&mut all_the_files, r#""#,)?;
    writeln!(&mut all_the_files, r#"#[allow(dead_code)]"#,)?;
    writeln!(
        &mut all_the_files,
        r#"pub fn {}() -> Vec<Asset> {{"#,
        walk_paths.method
    )?;
    writeln!(&mut all_the_files, r#"    vec!["#,)?;

    for f in walk_paths.entries {
        writeln!(&mut all_the_files, r#"        Asset::new("#,)?;
        writeln!(
            &mut all_the_files,
            r#"            "{name}","#,
            name = f.path().display()
        )?;
        writeln!(
            &mut all_the_files,
            r#"            {stmacro}!("{name}"),"#,
            stmacro = stmacro,
            name = f.path().display()
        )?;
        writeln!(&mut all_the_files, r#"        ),"#,)?;
    }

    writeln!(&mut all_the_files, r#"    ]"#,)?;
    writeln!(&mut all_the_files, r#"}}"#,)?;

    Ok(())
}
