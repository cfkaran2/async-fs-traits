//! [`AsyncDirEntryTrait`] represents an entry in the filesystem.
//!
//! Objects in the filesystem may be files, directories, or other special
//! such as symlinks, FIFOs, or anything else that the given filesystem is able
//! to represent.  [`AsyncDirEntryTrait`] implementors don't give you direct
//! access to the underlying object, instead they give you some relatively
//! lightweight metadata about the objects that are encountered.  Note the
//! 'lightweight' part of the earlier sentence as that is the real reason they
//! exist; they are intended to be returned by iterators *quickly* and
//! *cheaply*.  Keep this in mind when implementing this trait.

#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};
use std::{ffi::OsString, io, path::PathBuf};

use async_trait::async_trait;

/// An entry in a directory.
///
/// A stream of entries in a directory is returned by[`read_dir()`][1].
///
/// [1]: super::AsyncFsTrait::read_dir
#[async_trait]
pub trait AsyncDirEntryTrait: std::fmt::Debug + std::clone::Clone {
    /// Returns the full path to this entry.
    ///
    /// The full path is created by joining the original path passed to
    /// [`read_dir()`][1] with the name of this entry.
    ///
    /// [1]: super::AsyncFsTrait::read_dir
    async fn path(&self) -> PathBuf;

    /// Reads the metadata for this entry.
    ///
    /// This function will traverse symbolic links to read the metadata.
    ///
    /// If you want to read metadata without following symbolic links, use
    /// [`symlink_metadata()`][1] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read the metadata.
    /// * Some other I/O error occurred.
    ///
    /// [1]: super::AsyncFsTrait::symlink_metadata
    async fn metadata(&self) -> io::Result<Metadata>;

    /// Reads the file type for this entry.
    ///
    /// This function will not traverse symbolic links if this entry points at
    /// one.
    ///
    /// If you want to read metadata with following symbolic links, use
    /// [`metadata()`][1] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read this entry's metadata.
    /// * Some other I/O error occurred.
    ///
    /// [1]: super::AsyncFsTrait::metadata
    async fn file_type(&self) -> io::Result<FileType>;

    /// Returns the bare name of this entry without the leading path.
    async fn file_name(&self) -> OsString;
}

//  ▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄    ▄▄▄▄    ▄▄▄▄▄▄▄▄    ▄▄▄▄
//  ▀▀▀██▀▀▀  ██▀▀▀▀▀▀  ▄█▀▀▀▀█   ▀▀▀██▀▀▀  ▄█▀▀▀▀█
//     ██     ██        ██▄          ██     ██▄
//     ██     ███████    ▀████▄      ██      ▀████▄
//     ██     ██             ▀██     ██          ▀██
//     ██     ██▄▄▄▄▄▄  █▄▄▄▄▄█▀     ██     █▄▄▄▄▄█▀
//     ▀▀     ▀▀▀▀▀▀▀▀   ▀▀▀▀▀       ▀▀      ▀▀▀▀▀
//

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
