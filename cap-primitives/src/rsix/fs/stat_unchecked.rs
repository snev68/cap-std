use crate::fs::{FollowSymlinks, Metadata, MetadataExt};
use rsix::fs::{statat, AtFlags};
use std::path::Path;
use std::{fs, io};

/// *Unsandboxed* function similar to `stat`, but which does not perform
/// sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    let atflags = match follow {
        FollowSymlinks::Yes => AtFlags::empty(),
        FollowSymlinks::No => AtFlags::SYMLINK_NOFOLLOW,
    };

    Ok(statat(start, path, atflags).map(MetadataExt::from_rsix)?)
}
