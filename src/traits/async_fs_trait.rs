//! [`AsyncFsTrait`] is a trait for file systems as a whole.
//!
//! There are objects that can be viewed as file systems that are not normally
//! considered to be file systems.  Simple examples include things such as
//! archive and trees.  For this reason, this crate provides the
//! [`AsyncFsTrait`] trait, which defines a set of associated functions that
//! would normally be free functions.  See the documentation for each to
//! understand what they do.

#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};
use std::{
    io,
    path::{Path, PathBuf}
};

use async_trait::async_trait;

#[doc(no_inline)]
use super::AsyncDirEntryTrait;
use super::AsyncReadDirTrait;

/// [`AsyncFsTrait`] is a trait for file systems as a whole.
///
/// There are objects that can be viewed as file systems that are not normally
/// considered to be file systems.  Simple examples include things such as
/// archive and trees.  For this reason, this crate provides the
/// [`AsyncFsTrait`] trait, which defines a set of associated functions that
/// would normally be free functions.  See the documentation for each to
/// understand what they do.
#[async_trait]
pub trait AsyncFsTrait {
    /// Returns the canonical form of a path.
    ///
    /// The returned path is in absolute form with all intermediate components
    /// normalized and symbolic links resolved.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * A non-final component in `path` is not a directory.
    /// * Some other I/O error occurred.
    async fn canonicalize<P>(path: P) -> io::Result<PathBuf>
        where P: AsRef<Path>;

    /// Renames a file or directory to a new location.
    ///
    /// If a file or directory already exists at the target location, it will be
    /// overwritten by this operation.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `src` does not point to an existing file or directory.
    /// * `src` and `dst` are on different filesystems.
    /// * The current process lacks permissions to do the rename operation.
    /// * Some other I/O error occurred.
    async fn rename<P, Q>(src: P, dst: Q) -> io::Result<()>
        where P: AsRef<Path>,
              Q: AsRef<Path>;

    /// Changes the permissions of a file or directory.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * The current process lacks permissions to change attributes on the
    ///   file or directory.
    /// * Some other I/O error occurred.
    async fn set_permissions<P>(path: P, perm: Permissions) -> io::Result<()>
        where P: AsRef<Path>;

    /// Creates a hard link on the filesystem.
    ///
    /// The `dst` path will be a link pointing to the `src` path. Note that
    /// operating systems often require these two paths to be located on the
    /// same filesystem.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `src` does not point to an existing file.
    /// * Some other I/O error occurred.
    async fn hard_link<P, Q>(src: P, dst: Q) -> io::Result<()>
        where P: AsRef<Path>,
              Q: AsRef<Path>;

    /// Reads a symbolic link and returns the path it points to.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing link.
    /// * Some other I/O error occurred.
    async fn read_link<P>(path: P) -> io::Result<PathBuf>
        where P: AsRef<Path>;

    /// Reads metadata for a path without following symbolic links.
    ///
    /// If you want to follow symbolic links before reading metadata of the
    /// target file or directory, use [`metadata()`][1] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * The current process lacks permissions to read metadata for the path.
    /// * Some other I/O error occurred.
    ///
    /// [1]: AsyncFsTrait::metadata
    async fn symlink_metadata<P>(path: P) -> io::Result<Metadata>
        where P: AsRef<Path>;

    /// Reads metadata for a path.
    ///
    /// This function will traverse symbolic links to read metadata for the
    /// target file or directory. If you want to read metadata without
    /// following symbolic links, use[`symlink_metadata()`][1] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * The current process lacks permissions to read metadata for the path.
    /// * Some other I/O error occurred.
    ///
    /// [1]: AsyncFsTrait::symlink_metadata
    async fn metadata<P>(path: P) -> io::Result<Metadata>
        where P: AsRef<Path>;

    /// Copies a file to a new location.
    ///
    /// On success, the total number of bytes copied is returned and equals the
    /// length of the `dst` file after this operation.
    ///
    /// The old contents of `dst` will be overwritten. If `src` and `dst` both
    /// point to the same file, then the file will likely get truncated as a
    /// result of this operation.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `src` does not point to an existing file.
    /// * The current process lacks permissions to read `src` or write `dst`.
    /// * Some other I/O error occurred.
    async fn copy<P, Q>(src: P, dst: Q) -> io::Result<u64>
        where P: AsRef<Path>,
              Q: AsRef<Path>;

    /// Removes a file.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to remove the file.
    /// * Some other I/O error occurred.
    async fn remove_file<P>(path: P) -> io::Result<()>
        where P: AsRef<Path>;

    /// Returns a stream of entries in a directory.
    ///
    /// The stream yields items of type
    /// [`io::Result`]`<`[`AsyncDirEntryTrait`]`>`. Note that I/O errors can
    /// occur while reading from the stream.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing directory.
    /// * The current process lacks permissions to read the contents of the
    ///   directory.
    /// * Some other I/O error occurred.
    async fn read_dir<P, T, U>(path: P) -> io::Result<T>
        where P: AsRef<Path>,
              T: AsyncReadDirTrait<U>,
              U: AsyncDirEntryTrait;

    /// Removes an empty directory.
    ///
    /// Note that this function can only delete an empty directory. If you want
    /// to delete a directory and all of its contents, use
    /// [`remove_dir_all()`][1] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` is not an existing and empty directory.
    /// * The current process lacks permissions to remove the directory.
    /// * Some other I/O error occurred.
    ///
    /// [1]: super::AsyncFsTrait::remove_dir_all()
    async fn remove_dir<P>(path: P) -> io::Result<()>
        where P: AsRef<Path>;

    /// Removes a directory and all of its contents.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` is not an existing and empty directory.
    /// * The current process lacks permissions to remove the directory.
    /// * Some other I/O error occurred.
    async fn remove_dir_all<P>(path: P) -> io::Result<()>
        where P: AsRef<Path>;
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
