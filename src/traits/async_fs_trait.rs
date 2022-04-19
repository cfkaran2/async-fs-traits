//! TODO: Fill this in

#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};
use std::{
    io::{self},
    path::{Path, PathBuf}
};

use async_trait::async_trait;

#[doc(no_inline)]
use crate::AsyncDirEntryTrait;
use crate::AsyncReadDirTrait;

/// TODO: Fill this in
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
    async fn canonicalize<P: AsRef<Path>>(path: P) -> io::Result<PathBuf>;

    /// Copies a file to a new location.
    ///
    /// On success, the total number of bytes copied is returned and equals the
    /// length of the `dst` file after this operation.
    ///
    /// The old contents of `dst` will be overwritten. If `src` and `dst` both
    /// point to the same file, then the file will likely get truncated as a
    /// result of this operation.
    ///
    /// If you're working with open [`AsyncDirEntryTrait`]s and want to copy
    /// contents through those types, use[`futures_lite::io::copy()`] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `src` does not point to an existing file.
    /// * The current process lacks permissions to read `src` or write `dst`.
    /// * Some other I/O error occurred.
    async fn copy<P: AsRef<Path>, Q: AsRef<Path>>(src: P,
                                                  dst: Q)
                                                  -> io::Result<u64>;

    /// Creates a directory.
    ///
    /// Note that this function will only create the final directory in `path`.
    /// If you want to create all of its missing parent directories too, use
    /// [`create_dir_all()`] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` already points to an existing file or directory.
    /// * A parent directory in `path` does not exist.
    /// * The current process lacks permissions to create the directory.
    /// * Some other I/O error occurred.
    async fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()>;

    /// Creates a directory and its parent directories if they are missing.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` already points to an existing file or directory.
    /// * The current process lacks permissions to create the directory or its
    ///   missing parents.
    /// * Some other I/O error occurred.
    async fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()>;

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
    async fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P,
                                                       dst: Q)
                                                       -> io::Result<()>;

    /// Reads metadata for a path.
    ///
    /// This function will traverse symbolic links to read metadata for the
    /// target file or directory. If you want to read metadata without
    /// following symbolic links, use [`symlink_metadata()`] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * The current process lacks permissions to read metadata for the path.
    /// * Some other I/O error occurred.
    async fn metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata>;

    /// Reads the entire contents of a file as raw bytes.
    ///
    /// This is a convenience function for reading entire files. It
    /// pre-allocates a buffer based on the file size when available, so it is
    /// typically faster than manually opening a file and reading from it.
    ///
    /// If you want to read the contents as a string, use [`read_to_string()`]
    /// instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to read the file.
    /// * Some other I/O error occurred.
    async fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>>;

    /// Returns a stream of entries in a directory.
    ///
    /// The stream yields items of type [`io::Result`]`<`[`AsyncDirEntryTrait`]`>`. Note
    /// that I/O errors can occur while reading from the stream.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing directory.
    /// * The current process lacks permissions to read the contents of the
    ///   directory.
    /// * Some other I/O error occurred.
    async fn read_dir<P: AsRef<Path>, T, U>(path: P) -> io::Result<T>
        where T: AsyncReadDirTrait<U>,
              U: AsyncDirEntryTrait;

    /// Reads a symbolic link and returns the path it points to.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing link.
    /// * Some other I/O error occurred.
    async fn read_link<P: AsRef<Path>>(path: P) -> io::Result<PathBuf>;

    /// Reads the entire contents of a file as a string.
    ///
    /// This is a convenience function for reading entire files. It
    /// pre-allocates a string based on the file size when available, so it is
    /// typically faster than manually opening a file and reading from it.
    ///
    /// If you want to read the contents as raw bytes, use [`read()`] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to read the file.
    /// * The contents of the file cannot be read as a UTF-8 string.
    /// * Some other I/O error occurred.
    async fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String>;

    /// Removes an empty directory.
    ///
    /// Note that this function can only delete an empty directory. If you want
    /// to delete a directory and all of its contents, use [`remove_dir_all()`]
    /// instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` is not an existing and empty directory.
    /// * The current process lacks permissions to remove the directory.
    /// * Some other I/O error occurred.
    async fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()>;

    /// Removes a directory and all of its contents.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` is not an existing and empty directory.
    /// * The current process lacks permissions to remove the directory.
    /// * Some other I/O error occurred.
    async fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()>;

    /// Removes a file.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to remove the file.
    /// * Some other I/O error occurred.
    async fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()>;

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
    async fn rename<P: AsRef<Path>, Q: AsRef<Path>>(src: P,
                                                    dst: Q)
                                                    -> io::Result<()>;

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
    async fn set_permissions<P: AsRef<Path>>(path: P,
                                             perm: Permissions)
                                             -> io::Result<()>;

    /// Reads metadata for a path without following symbolic links.
    ///
    /// If you want to follow symbolic links before reading metadata of the
    /// target file or directory, use [`metadata()`] instead.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * The current process lacks permissions to read metadata for the path.
    /// * Some other I/O error occurred.
    async fn symlink_metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata>;

    /// Writes a slice of bytes as the new contents of a file.
    ///
    /// This function will create a file if it does not exist, and will entirely
    /// replace its contents if it does.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The file's parent directory does not exist.
    /// * The current process lacks permissions to write to the file.
    /// * Some other I/O error occurred.
    async fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P,
                                                   contents: C)
                                                   -> io::Result<()>;
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
