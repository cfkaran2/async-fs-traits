//! TODO: Fill this in

pub use std::fs::{FileType, Metadata, Permissions};
use std::io;

use async_trait::async_trait;
use futures_lite::io::{AsyncRead, AsyncSeek, AsyncWrite};

/// An open file on the filesystem.
///
/// Depending on what options the file was opened with, this type can be used
/// for reading and/or writing.  Files should be created or opened using a type
/// that implements the
/// [`AsyncFileBuilderTrait`][crate::async_file_builder_trait::AsyncFileBuilderTrait].
///
/// Files are automatically closed when they get dropped and any errors detected
/// on closing are ignored. Use the
/// [`sync_all()`][AsyncFileTrait::sync_all()]
/// method before dropping a file if such errors need to be handled.
///
/// **NOTE:** If writing to a file, make sure to call
/// [`flush()`][`futures_lite::io::AsyncWriteExt::flush()`],
/// [`sync_data()`][AsyncFileTrait::sync_data()],
/// or
/// [`sync_all()`][AsyncFileTrait::sync_all()]
/// before dropping the file or else some written data might get lost!
#[async_trait]
pub trait AsyncFileTrait:
    std::fmt::Debug + AsyncRead + AsyncSeek + AsyncWrite
{
    /// Synchronizes OS-internal buffered contents and metadata to disk.
    ///
    /// This function will ensure that all in-memory data reaches the
    /// filesystem.
    ///
    /// This can be used to handle errors that would otherwise only be caught by
    /// closing the file. When a file is dropped, errors in synchronizing this
    /// in-memory data are ignored.
    async fn sync_all(&self) -> io::Result<()>;

    /// Synchronizes OS-internal buffered contents to disk.
    ///
    /// This is similar to [`sync_all()`][`AsyncDirEntryTrait::sync_data
    /// ()`], except that file metadata may not be synchronized.
    ///
    /// This is intended for use cases that must synchronize the contents of the
    /// file, but don't need the file metadata synchronized to disk.
    ///
    /// Note that some platforms may simply implement this in terms of
    /// [`sync_all()`][`AsyncDirEntryTrait::sync_data()`].
    async fn sync_data(&self) -> io::Result<()>;

    /// Truncates or extends the file.
    ///
    /// If `size` is less than the current file size, then the file will be
    /// truncated. If it is greater than the current file size, then the file
    /// will be extended to `size` and have all intermediate data filled with
    /// zeros.
    ///
    /// The file's cursor stays at the same position, even if the cursor ends up
    /// being past the end of the file after this operation.
    async fn set_len(&self, size: u64) -> io::Result<()>;

    /// Reads the file's metadata.
    async fn metadata(&self) -> io::Result<Metadata>;

    /// Changes the permissions on the file.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The current process lacks permissions to change attributes on the
    ///   file.
    /// * Some other I/O error occurred.
    async fn set_permissions(&self, perm: Permissions) -> io::Result<()>;
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
