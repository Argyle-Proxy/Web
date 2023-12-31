#![allow(clippy::comparison_chain)]
//! A pure-Rust library to manage extended attributes.
//!
//! It provides support for manipulating extended attributes
//! (`xattrs`) on modern Unix filesystems. See the `attr(5)`
//! manpage for more details.
//!
//! An extension trait [`FileExt`] is provided to directly work with
//! standard `File` objects and file descriptors.
//!
//! NOTE: In case of a symlink as path argument, all methods
//! in this library work on the symlink itself **without**
//! de-referencing it.
//!
//! ```rust
//! let mut xattrs = xattr::list("/").unwrap().peekable();
//!
//! if xattrs.peek().is_none() {
//!     println!("no xattr set on root");
//!     return;
//! }
//!
//! println!("Extended attributes:");
//! for attr in xattrs {
//!     println!(" - {:?}", attr);
//! }
//! ```

mod error;
mod sys;
mod util;

use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::os::unix::io::{AsRawFd, BorrowedFd};
use std::path::Path;

pub use error::UnsupportedPlatformError;
pub use sys::{XAttrs, SUPPORTED_PLATFORM};

/// Get an extended attribute for the specified file.
pub fn get<N, P>(path: P, name: N) -> io::Result<Option<Vec<u8>>>
where
    P: AsRef<Path>,
    N: AsRef<OsStr>,
{
    util::extract_noattr(sys::get_path(path.as_ref(), name.as_ref()))
}

/// Set an extended attribute on the specified file.
pub fn set<N, P>(path: P, name: N, value: &[u8]) -> io::Result<()>
where
    P: AsRef<Path>,
    N: AsRef<OsStr>,
{
    sys::set_path(path.as_ref(), name.as_ref(), value)
}

/// Remove an extended attribute from the specified file.
pub fn remove<N, P>(path: P, name: N) -> io::Result<()>
where
    P: AsRef<Path>,
    N: AsRef<OsStr>,
{
    sys::remove_path(path.as_ref(), name.as_ref())
}

/// List extended attributes attached to the specified file.
///
/// Note: this may not list *all* attributes. Speficially, it definitely won't list any trusted
/// attributes unless you are root and it may not list system attributes.
pub fn list<P>(path: P) -> io::Result<XAttrs>
where
    P: AsRef<Path>,
{
    sys::list_path(path.as_ref())
}

/// Extension trait to manipulate extended attributes on `File`-like objects.
pub trait FileExt: AsRawFd {
    /// Get an extended attribute for the specified file.
    fn get_xattr<N>(&self, name: N) -> io::Result<Option<Vec<u8>>>
    where
        N: AsRef<OsStr>,
    {
        // SAFETY: Implement I/O safety later.
        let fd = unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) };
        util::extract_noattr(sys::get_fd(fd, name.as_ref()))
    }

    /// Set an extended attribute on the specified file.
    fn set_xattr<N>(&self, name: N, value: &[u8]) -> io::Result<()>
    where
        N: AsRef<OsStr>,
    {
        let fd = unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) };
        sys::set_fd(fd, name.as_ref(), value)
    }

    /// Remove an extended attribute from the specified file.
    fn remove_xattr<N>(&self, name: N) -> io::Result<()>
    where
        N: AsRef<OsStr>,
    {
        let fd = unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) };
        sys::remove_fd(fd, name.as_ref())
    }

    /// List extended attributes attached to the specified file.
    ///
    /// Note: this may not list *all* attributes. Speficially, it definitely won't list any trusted
    /// attributes unless you are root and it may not list system attributes.
    fn list_xattr(&self) -> io::Result<XAttrs> {
        let fd = unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) };
        sys::list_fd(fd)
    }
}

impl FileExt for File {}
