use crate::write::write;
use common_path::common_path;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn new<P: AsRef<Path>>(path: P) -> WalkPaths {
    WalkPaths {
        entry_path: path.as_ref().to_owned(),
        files: WalkDir::new(path)
            .into_iter()
            .filter_map(|e| {
                if let Ok(dir_entry) = e {
                    if let Ok(path) = dir_entry.path().canonicalize() {
                        if !path.is_dir() {
                            return Some(path);
                        }
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
        let output_path = output_path.as_ref();

        if !output_path.exists() {
            std::fs::File::create(output_path)?;
        }

        let output_path = output_path.canonicalize()?;
        let files = self.relative_paths(&output_path);
        match self.cast {
            Cast::Str => write(output_path, String::from("include_str"), files),
            Cast::Bytes => write(output_path, String::from("include_bytes"), files),
        }
    }

    fn relative_paths(&self, ref_path: &PathBuf) -> Vec<PathBuf> {
        if let Some(ref_path_dir) = ref_path.parent() {
            self.files
                .iter()
                .map(|p| WalkPaths::relative_path(ref_path, p).map_or(p.to_path_buf(), |p| p))
                .collect()
        } else {
            self.files.to_owned()
        }
    }

    /// Return relative path between to path.
    /// Inspiration : https://nodejs.org/api/path.html#path_path_relative_from_to
    fn relative_path(from: &PathBuf, to: &PathBuf) -> Option<PathBuf> {
        if let Some(common_path) = common_path(from, to) {
            if let Ok(file_name) = to.strip_prefix(&common_path) {
                let mut c: u32 = 0;
                for path in from.ancestors() {
                    if common_path == path {
                        break;
                    }
                    c += 1;
                }
                let mut path = PathBuf::new();
                for _ in 1..c {
                    path = path.join("..");
                }
                Some(path.join(file_name))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::walk_paths::{new, WalkPaths};
    use insta::assert_debug_snapshot;
    use std::path::PathBuf;

    #[test]
    fn relative_path() {
        let from = PathBuf::from("p1/p2/f1");
        let to = PathBuf::from("p1/p3");

        let relative_path = WalkPaths::relative_path(&from, &to);
        assert_debug_snapshot!(relative_path);

        let from = PathBuf::from("p1/p2/p3/f1");
        let to = PathBuf::from("p1/p4/p5");

        let relative_path = WalkPaths::relative_path(&from, &to);
        assert_debug_snapshot!(relative_path);
        // TODO: more test
    }

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

        walk_path.files = vec![PathBuf::from("path/test1/test2")];
        let list_paths = walk_path.relative_paths(&PathBuf::from("path/test3/test4"));
        assert_debug_snapshot!(list_paths);
    }
}
