//! TODO: Fill this in

#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};
use std::{
    ffi::OsString,
    fmt,
    io::{self},
    path::PathBuf,
    sync::Arc
};

use blocking::unblock;

/// An entry in a directory.
///
/// A stream of entries in a directory is returned by [`read_dir()`].
///
/// For Unix-specific options, import the [`DirEntryExt`][`std::os::unix::fs::DirEntryExt`] trait.
pub struct DirEntry(Arc<std::fs::DirEntry>);

impl DirEntry {
    /// TODO: Fill this in
    pub fn new(entry: std::fs::DirEntry) -> DirEntry {
        DirEntry(Arc::new(entry))
    }

    /// Returns the full path to this entry.
    ///
    /// The full path is created by joining the original path passed to [`read_dir()`] with the
    /// name of this entry.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures_lite::stream::StreamExt;
    ///
    /// # futures_lite::future::block_on(async {
    /// let mut dir = async_fs::read_dir(".").await?;
    ///
    /// while let Some(entry) = dir.try_next().await? {
    ///     println!("{:?}", entry.path());
    /// }
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn path(&self) -> PathBuf {
        self.0.path()
    }

    /// Reads the metadata for this entry.
    ///
    /// This function will traverse symbolic links to read the metadata.
    ///
    /// If you want to read metadata without following symbolic links, use [`symlink_metadata()`]
    /// instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read the metadata.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures_lite::stream::StreamExt;
    ///
    /// # futures_lite::future::block_on(async {
    /// let mut dir = async_fs::read_dir(".").await?;
    ///
    /// while let Some(entry) = dir.try_next().await? {
    ///     println!("{:?}", entry.metadata().await?);
    /// }
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn metadata(&self) -> io::Result<Metadata> {
        let inner = self.0.clone();
        unblock(move || inner.metadata()).await
    }

    /// Reads the file type for this entry.
    ///
    /// This function will not traverse symbolic links if this entry points at one.
    ///
    /// If you want to read metadata with following symbolic links, use [`metadata()`] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read this entry's metadata.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures_lite::stream::StreamExt;
    ///
    /// # futures_lite::future::block_on(async {
    /// let mut dir = async_fs::read_dir(".").await?;
    ///
    /// while let Some(entry) = dir.try_next().await? {
    ///     println!("{:?}", entry.file_type().await?);
    /// }
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn file_type(&self) -> io::Result<FileType> {
        let inner = self.0.clone();
        unblock(move || inner.file_type()).await
    }

    /// Returns the bare name of this entry without the leading path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures_lite::stream::StreamExt;
    ///
    /// # futures_lite::future::block_on(async {
    /// let mut dir = async_fs::read_dir(".").await?;
    ///
    /// while let Some(entry) = dir.try_next().await? {
    ///     println!("{}", entry.file_name().to_string_lossy());
    /// }
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn file_name(&self) -> OsString {
        self.0.file_name()
    }
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.path()).finish()
    }
}

impl Clone for DirEntry {
    fn clone(&self) -> Self {
        DirEntry(self.0.clone())
    }
}

#[cfg(unix)]
impl std::os::unix::fs::DirEntryExt for DirEntry {
    fn ino(&self) -> u64 {
        self.0.ino()
    }
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
