use posish::fs::readlinkat;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// *Unsandboxed* function similar to `read_link`, but which does not perform sandboxing.
pub(crate) fn read_link_unchecked(
    start: &fs::File,
    path: &Path,
    reuse: PathBuf,
) -> io::Result<PathBuf> {
    readlinkat(start, path, reuse.into()).map(Into::into)
}
