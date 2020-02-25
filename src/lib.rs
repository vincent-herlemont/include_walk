use crate::walk_paths::{new, WalkPaths};
use std::path::Path;
pub mod walk_paths;
mod write;

pub fn from<P: AsRef<Path>>(path: P) -> WalkPaths {
    new(path.as_ref().to_path_buf())
}
