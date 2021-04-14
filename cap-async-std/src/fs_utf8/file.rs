use crate::fs::{Metadata, Permissions};
#[cfg(unix)]
use async_std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
use async_std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use async_std::{
    fs,
    io::{self, IoSlice, IoSliceMut, Read, Seek, SeekFrom, Write},
    task::{Context, Poll},
};
use cap_primitives::{ambient_authority, AmbientAuthority};
use std::{fmt, pin::Pin};
use unsafe_io::OwnsRaw;
#[cfg(windows)]
use {
    async_std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle},
    unsafe_io::os::windows::{AsRawHandleOrSocket, IntoRawHandleOrSocket, RawHandleOrSocket},
};

/// A reference to an open file on a filesystem.
///
/// This corresponds to [`async_std::fs::File`].
///
/// Note that this `File` has no `open` or `create` methods. To open or create
/// a file, you must first obtain a [`Dir`] containing the path, and then call
/// [`Dir::open`] or [`Dir::create`].
///
/// [`Dir`]: crate::fs::Dir
/// [`Dir::open`]: crate::fs::Dir::open
/// [`Dir::create`]: crate::fs::Dir::create
pub struct File {
    cap_std: crate::fs::File,
}

impl File {
    /// Constructs a new instance of `Self` from the given `async_std::fs::File`.
    ///
    /// # Ambient Authority
    ///
    /// `async_std::fs::File` is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub fn from_std(std: fs::File, ambient_authority: AmbientAuthority) -> Self {
        Self::from_cap_std(crate::fs::File::from_std(std, ambient_authority))
    }

    /// Constructs a new instance of `Self` from the given `cap_std::fs::File`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::File) -> Self {
        Self { cap_std }
    }

    /// Consumes `self` and returns an `async_std::fs::File`.
    #[inline]
    pub fn into_std(self) -> fs::File {
        self.cap_std.into_std()
    }

    // async_std doesn't have `with_options`.

    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// This corresponds to [`async_std::fs::File::sync_all`].
    #[inline]
    pub async fn sync_all(&self) -> io::Result<()> {
        self.cap_std.sync_all().await
    }

    /// This function is similar to `sync_all`, except that it may not synchronize
    /// file metadata to a filesystem.
    ///
    /// This corresponds to [`async_std::fs::File::sync_data`].
    #[inline]
    pub async fn sync_data(&self) -> io::Result<()> {
        self.cap_std.sync_data().await
    }

    /// Truncates or extends the underlying file, updating the size of this file
    /// to become size.
    ///
    /// This corresponds to [`async_std::fs::File::set_len`].
    #[inline]
    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        self.cap_std.set_len(size).await
    }

    /// Queries metadata about the underlying file.
    ///
    /// This corresponds to [`async_std::fs::File::metadata`].
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.cap_std.metadata()
    }

    // async_std doesn't have `try_clone`.

    /// Changes the permissions on the underlying file.
    ///
    /// This corresponds to [`async_std::fs::File::set_permissions`].
    #[inline]
    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        self.cap_std.set_permissions(perm).await
    }
}

#[cfg(not(windows))]
impl FromRawFd for File {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(fs::File::from_raw_fd(fd), ambient_authority())
    }
}

#[cfg(windows)]
impl FromRawHandle for File {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_std(fs::File::from_raw_handle(handle), ambient_authority())
    }
}

#[cfg(not(windows))]
impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.cap_std.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for File {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.cap_std.as_raw_handle()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for File {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.cap_std.as_raw_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl IntoRawFd for File {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.cap_std.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawHandle for File {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.cap_std.into_raw_handle()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for File {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.cap_std.into_raw_handle_or_socket()
    }
}

// Safety: `File` wraps a `fs::File` which owns its handle.
unsafe impl OwnsRaw for File {}

impl Read for File {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read(Pin::new(&mut self.cap_std), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &mut [IoSliceMut],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read_vectored(Pin::new(&mut self.cap_std), cx, bufs)
    }

    // async_std doesn't have `is_read_vectored`.

    // async_std doesn't have `initializer`.
}

impl Read for &File {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read(Pin::new(&mut &self.cap_std), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &mut [IoSliceMut],
    ) -> Poll<io::Result<usize>> {
        Read::poll_read_vectored(Pin::new(&mut &self.cap_std), cx, bufs)
    }

    // async_std doesn't have `is_read_vectored`.

    // async_std doesn't have `initializer`.
}

impl Write for File {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Write::poll_write(Pin::new(&mut self.cap_std), cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_flush(Pin::new(&mut self.cap_std), cx)
    }

    #[inline]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_close(Pin::new(&mut self.cap_std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &[IoSlice],
    ) -> Poll<io::Result<usize>> {
        Write::poll_write_vectored(Pin::new(&mut self.cap_std), cx, bufs)
    }

    // async_std doesn't have `can_vector`.

    // async_std doesn't have `write_all_vectored`.
}

impl Write for &File {
    #[inline]
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        Write::poll_write(Pin::new(&mut &self.cap_std), cx, buf)
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_flush(Pin::new(&mut &self.cap_std), cx)
    }

    #[inline]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Write::poll_close(Pin::new(&mut &self.cap_std), cx)
    }

    #[inline]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &[IoSlice],
    ) -> Poll<io::Result<usize>> {
        Write::poll_write_vectored(Pin::new(&mut &self.cap_std), cx, bufs)
    }

    // async_std doesn't have `can_vector`.

    // async_std doesn't have `write_all_vectored`.
}

impl Seek for File {
    #[inline]
    fn poll_seek(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        pos: SeekFrom,
    ) -> Poll<io::Result<u64>> {
        Seek::poll_seek(Pin::new(&mut self.cap_std), cx, pos)
    }

    // async_std doesn't have `stream_position`.
}

impl Seek for &File {
    #[inline]
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context, pos: SeekFrom) -> Poll<io::Result<u64>> {
        Seek::poll_seek(Pin::new(&mut &self.cap_std), cx, pos)
    }

    // async_std doesn't have `stream_position`.
}

// TODO: Can async_std implement `From<File>` for `process::Stdio`?

// async_std doesn't have `FileExt`.

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cap_std.fmt(f)
    }
}
