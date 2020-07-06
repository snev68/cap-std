//! The `FileType` struct.

use crate::fs::FileTypeExt;
use std::fs;

/// `FileType`'s inner state.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Inner {
    /// A directory.
    Dir,

    /// A file.
    File,

    /// A symbolic link.
    Symlink,

    /// An unknown entity.
    Unknown,

    /// A `FileTypeExt` type.
    #[cfg(any(unix, windows, target_os = "vxworks"))]
    Ext(FileTypeExt),
}

/// A structure representing a type of file with accessors for each file type.
///
/// This corresponds to [`std::fs::FileType`].
///
/// [`std::fs::FileType`]: https://doc.rust-lang.org/std/fs/struct.FileType.html
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileType(Inner);

impl FileType {
    /// Constructs a new instance of `Self` from the given `std::fs::FileType`.
    #[inline]
    pub(crate) fn from_std(std: fs::FileType) -> Self {
        Self(if std.is_dir() {
            Inner::Dir
        } else if std.is_file() {
            Inner::File
        } else if std.is_symlink() {
            Inner::Symlink
        } else if let Some(ext) = FileTypeExt::from_std(std) {
            Inner::Ext(ext)
        } else {
            Inner::Unknown
        })
    }

    /// Creates a `FileType` for which `is_dir()` returns `true`.
    #[inline]
    pub const fn dir() -> Self {
        Self(Inner::Dir)
    }

    /// Creates a `FileType` for which `is_file()` returns `true`.
    #[inline]
    pub const fn file() -> Self {
        Self(Inner::File)
    }

    /// Creates a `FileType` for which `is_symlink()` returns `true`.
    #[inline]
    pub const fn symlink() -> Self {
        Self(Inner::Symlink)
    }

    /// Creates a `FileType` for which `is_unknown()` returns `true`.
    #[inline]
    pub const fn unknown() -> Self {
        Self(Inner::Unknown)
    }

    /// Creates a `FileType` for which `is_dir()` returns `true`.
    #[inline]
    pub(crate) const fn ext(ext: FileTypeExt) -> Self {
        Self(Inner::Ext(ext))
    }

    /// Tests whether this file type represents a directory.
    ///
    /// This corresponds to [`std::fs::FileType::is_dir`].
    ///
    /// [`std::fs::FileType::is_dir`]: https://doc.rust-lang.org/std/fs/struct.FileType.html#method.is_dir
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.0 == Inner::Dir
    }

    /// Tests whether this file type represents a regular file.
    ///
    /// This corresponds to [`std::fs::FileType::is_file`].
    ///
    /// [`std::fs::FileType::is_file`]: https://doc.rust-lang.org/std/fs/struct.FileType.html#method.is_file
    #[inline]
    pub fn is_file(&self) -> bool {
        self.0 == Inner::File
    }

    /// Tests whether this file type represents a symbolic link.
    ///
    /// This corresponds to [`std::fs::FileType::is_symlink`].
    ///
    /// [`std::fs::FileType::is_symlink`]: https://doc.rust-lang.org/std/fs/struct.FileType.html#method.is_symlink
    #[inline]
    pub fn is_symlink(&self) -> bool {
        self.0 == Inner::Symlink
    }
}

#[cfg(unix)]
impl std::os::unix::fs::FileTypeExt for FileType {
    #[inline]
    fn is_block_device(&self) -> bool {
        self.0 == Inner::Ext(crate::fs::FileTypeExt::block_device())
    }

    #[inline]
    fn is_char_device(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::char_device())
    }

    #[inline]
    fn is_fifo(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::fifo())
    }

    #[inline]
    fn is_socket(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::socket())
    }
}

#[cfg(target_os = "vxworks")]
impl std::os::vxworks::fs::FileTypeExt for FileType {
    #[inline]
    fn is_block_device(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::BlockDevice)
    }

    #[inline]
    fn is_char_device(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::CharDevice)
    }

    #[inline]
    fn is_fifo(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::Fifo)
    }

    #[inline]
    fn is_socket(&self) -> bool {
        self.0 == Inner::Ext(FileTypeExt::Socket)
    }
}

#[cfg(windows)]
impl std::os::windows::fs::FileTypeExt for FileType {
    #[inline]
    fn is_symlink_dir(&self) -> bool {
        unimplemnted!("FileTypeExt::is_symlink_dir for Windows")
    }

    #[inline]
    fn is_symlink_file(&self) -> bool {
        unimplemnted!("FileTypeExt::is_symlink_file for Windows")
    }
}
