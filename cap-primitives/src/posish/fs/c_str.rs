use std::ffi::CString;
use std::io;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;
use std::path::Path;

#[allow(dead_code)]
pub(crate) fn c_str(path: &Path) -> io::Result<CString> {
    Ok(CString::new(path.as_os_str().as_bytes())?)
}
