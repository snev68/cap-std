use posish::fs::symlinkat;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `symlink`, but which does not perform
/// sandboxing.
pub(crate) fn symlink_unchecked(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    Ok(symlinkat(old_path, new_start, new_path)?)
}
