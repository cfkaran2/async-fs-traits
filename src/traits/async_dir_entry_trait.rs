//! TODO: Fill this in

#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};
use std::{ffi::OsString, io, path::PathBuf};

use async_trait::async_trait;

/// An entry in a directory.
///
/// A stream of entries in a directory is returned by
/// [`read_dir()`][crate::async_fs_trait::AsyncFsTrait::read_dir].
#[async_trait]
pub trait AsyncDirEntryTrait: std::fmt::Debug + std::clone::Clone {
    /// Returns the full path to this entry.
    ///
    /// The full path is created by joining the original path passed to
    /// [`read_dir()`][crate::async_fs_trait::AsyncFsTrait::read_dir] with the
    /// name of this entry.
    async fn path(&self) -> PathBuf;

    /// Reads the metadata for this entry.
    ///
    /// This function will traverse symbolic links to read the metadata.
    ///
    /// If you want to read metadata without following symbolic links, use
    /// [`symlink_metadata()`][crate::async_fs_trait::AsyncFsTrait::symlink_metadata]
    /// instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read the metadata.
    /// * Some other I/O error occurred.
    async fn metadata(&self) -> io::Result<Metadata>;

    /// Reads the file type for this entry.
    ///
    /// This function will not traverse symbolic links if this entry points at
    /// one.
    ///
    /// If you want to read metadata with following symbolic links, use
    /// [`metadata()`][crate::async_fs_trait::AsyncFsTrait::metadata] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read this entry's metadata.
    /// * Some other I/O error occurred.
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
