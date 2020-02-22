use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn from<P: AsRef<Path>>(path: P) -> WalkPaths {
    WalkPaths::new(path.as_ref().to_path_buf())
}

#[derive(Debug)]
pub enum Cast {
    Str,
    Bytes,
}

#[derive(Debug)]
pub struct WalkPaths {
    entries: Vec<DirEntry>,
    cast: Cast,
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
            cast: Cast::Str,
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
            Cast::Str => write(path, String::from("include_str"), self),
            Cast::Bytes => write(path, String::from("include_bytes"), self),
        }
    }
}

fn write(path: PathBuf, stmacro: String, walk_paths: WalkPaths) -> Result<(), Box<dyn Error>> {
    let mut all_the_files = File::create(path)?;

    writeln!(&mut all_the_files, r#"use std::collections::HashMap;"#,)?;
    writeln!(&mut all_the_files, r#""#,)?;
    writeln!(&mut all_the_files, r#"#[allow(dead_code)]"#,)?;
    writeln!(
        &mut all_the_files,
        r#"pub fn getAll() -> HashMap<&'static str, &'static str> {{"#,
    )?;
    writeln!(&mut all_the_files, r#"    let mut out = HashMap::new();"#,)?;

    for f in walk_paths.entries {
        writeln!(
            &mut all_the_files,
            r#"    out.insert("{path}", {stmacro}!("{path}"));"#,
            stmacro = stmacro,
            path = f.path().display()
        )?;
    }

    writeln!(&mut all_the_files, r#"    out"#,)?;
    writeln!(&mut all_the_files, r#"}}"#,)?;

    Ok(())
}
