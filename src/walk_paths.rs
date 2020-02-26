use crate::write::write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn new<P: AsRef<Path>>(path: P) -> WalkPaths {
    WalkPaths {
        entry_path: path.as_ref().to_owned(),
        files: WalkDir::new(path)
            .into_iter()
            .filter_map(|e| {
                if let Ok(dir_entry) = e {
                    if !dir_entry.path().is_dir() {
                        return Some(dir_entry.into_path());
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
    files: Vec<PathBuf>,
    cast: Cast,
}

impl WalkPaths {
    pub fn filter<P>(self, predicate: P) -> WalkPaths
    where
        P: FnMut(&PathBuf) -> bool,
    {
        WalkPaths {
            files: self.files.into_iter().filter(predicate).collect(),
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

    pub fn to<P: AsRef<Path>>(self, output_path: P) -> Result<(), Box<dyn std::error::Error>> {
        let output_path = output_path.as_ref().to_path_buf();
        let files = self.relative_paths(&output_path);
        match self.cast {
            Cast::Str => write(output_path, String::from("include_str"), files),
            Cast::Bytes => write(output_path, String::from("include_bytes"), files),
        }
    }

    fn relative_paths(&self, ref_path: &PathBuf) -> Vec<PathBuf> {
        if let Some(ref_path) = ref_path.parent() {
            self.files
                .iter()
                .map(|p| -> _ {
                    p.strip_prefix(ref_path)
                        .ok()
                        .map_or(p.to_path_buf(), |p| p.to_path_buf())
                })
                .collect()
        } else {
            self.files.iter().map(|p| p.to_path_buf()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::walk_paths::new;
    use insta::assert_debug_snapshot;
    use std::path::PathBuf;

    #[test]
    fn new_walk_paths() {
        let walk_path = new("path/test");
        assert_debug_snapshot!(walk_path);
    }

    #[test]
    fn walk_relative_paths() {
        let mut walk_path = new("");
        walk_path.files = vec![
            PathBuf::from("path/test/test/1"),
            PathBuf::from("path/test/1"),
            PathBuf::from("path/1"),
            PathBuf::from("path/2"),
        ];
        let list_paths = walk_path.relative_paths(&PathBuf::from("bad/path/output.rs"));
        assert_debug_snapshot!(list_paths);
        let list_paths = walk_path.relative_paths(&PathBuf::from("path/output.rs"));
        assert_debug_snapshot!(list_paths);
    }
}
