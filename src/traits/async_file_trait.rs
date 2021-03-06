//! [`AsyncFileTrait`] defines how file-like objects work
//!
//! 'File-like objects' have some high-level characteristics that go beyond the
//! ability to read, write, or seek on them.  These abilities are defined within
//! the [`AsyncFileTrait`] trait.
//!
//! What is **not** defined by the [`AsyncFileTrait`] trait are the methods
//! needed to perform reading, writing, or seeking.  These are defined by the
//! the traits [`futures_io::AsyncRead`] (and in some cases
//! [`futures_io::AsyncBufRead`]), [`futures_io::AsyncWrite`], and
//! [`futures_io::AsyncSeek`].
//!
//! The reason for this is because not all files are the same.  Some files are
//! on read-only file systems, and so the objects that represent them might
//! only implement the ability to read files.  Other 'files' may actually be
//! facades for hardware devices that can only be written to.  Similarly, not
//! all 'files' can have their cursors moved around ('seeking').  Trying to
//! force all file-like objects to implement those capabilities would turn what
//! could be compile-time checks via the type system into runtime checks that
//! might not be caught in testing.  For this reason, [`AsyncFileTrait`] doesn't
//! depend on [`futures_io::AsyncRead`], [`futures_io::AsyncWrite`], or
//! [`futures_io::AsyncSeek`].

pub use std::fs::{FileType, Metadata, Permissions};
use std::io;

use async_trait::async_trait;

/// An open file on the filesystem.
///
/// Depending on what options the file was opened with, this type can be used
/// for reading and/or writing.  Files should be created or opened using a type
/// that implements the [`AsyncFileBuilderTrait`][1].
///
/// Files are automatically closed when they get dropped and any errors detected
/// on closing are ignored. Use the [`sync_all()`][2] method before dropping a
/// file if such errors need to be handled.
///
/// **NOTE:** If writing to a file, make sure to call [`sync_data()`][3], or
/// [`sync_all()`][2] methods before dropping the file or else some written data
/// might get lost!
///
/// [1]: super::AsyncFileBuilderTrait
/// [2]: self::AsyncFileTrait::sync_all()
/// [3]: self::AsyncFileTrait::sync_data()
#[async_trait]
pub trait AsyncFileTrait: std::fmt::Debug {
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
    /// This is similar to [`sync_all()`][1], except that file metadata may not
    /// be synchronized.
    ///
    /// This is intended for use cases that must synchronize the contents of the
    /// file, but don't need the file metadata synchronized to disk.
    ///
    /// Note that some platforms may simply implement this in terms of
    /// [`sync_all()`][1].
    ///
    /// [1]: AsyncFileTrait::sync_all
    async fn sync_data(&self) -> io::Result<()>;

    /// Truncates or extends the file.
    ///
    /// If `size` is less than the current file size, then the file will be
    /// truncated. If it is greater than the current file size, then the file
    /// will be extended to `size` and have all intermediate data filled with
    /// zeros.  Note that truncation or extension always occur at the end of the
    /// file; bytes in positions [0, size - 1] are not affected by this call.
    /// When this call returns, the file's length is exactly `size` bytes.
    ///
    /// The file's cursor stays at the same position, even if the cursor ends up
    /// being past the end of the file after this operation.  It is
    /// implementation defined as to what will happen if cursor is used at that
    /// position before being moved to a point within the range [0, size].
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

//  ????????????????????????  ????????????????????????    ????????????    ????????????????????????    ????????????
//  ????????????????????????  ????????????????????????  ?????????????????????   ????????????????????????  ?????????????????????
//     ??????     ??????        ?????????          ??????     ?????????
//     ??????     ?????????????????????    ??????????????????      ??????      ??????????????????
//     ??????     ??????             ?????????     ??????          ?????????
//     ??????     ????????????????????????  ????????????????????????     ??????     ????????????????????????
//     ??????     ????????????????????????   ???????????????       ??????      ???????????????
//

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
