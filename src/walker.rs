use std::fs;
use std::path::{Path, PathBuf};

pub struct DirWalker {
    stack: Vec<PathBuf>,
}

impl DirWalker {
    fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut stack = Vec::new();

        let pathbuf = path.as_ref().to_path_buf();
        stack.push(pathbuf);

        DirWalker { stack }
    }
}

impl Iterator for DirWalker {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.stack.pop() {
            let pathname = entry.to_str().unwrap().into();

            if let Ok(md) = entry.metadata() {
                if md.is_dir() {
                    let mut children = Vec::new();

                    let dir = fs::read_dir(entry);
                    if let Ok(dir) = dir {
                        for d in dir {
                            if let Ok(d) = d {
                                children.push(d.path());
                            }
                        }
                    }

                    for child in children.into_iter().rev() {
                        self.stack.push(child);
                    }
                }
            }

            return Some(pathname);
        }

        None
    }
}

/// returns an Iterator that recursively walks through a given directory
/// and yields the path names of all files and subfolders
pub fn dir_walk<P>(path: P) -> DirWalker
where
    P: AsRef<Path>,
{
    DirWalker::new(path)
}
