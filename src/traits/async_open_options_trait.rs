//! TODO: Fill this in

pub use std::fs::{FileType, Metadata, Permissions};
use std::{
    future::Future,
    io::{self},
    path::Path
};

use async_trait::async_trait;

#[doc(no_inline)]
use crate::AsyncFileTrait;

/// A builder for opening files with configurable options.
///
/// Files can be opened in [`read`][`AsyncOpenOptionsTrait::read()`] and/or
/// [`write`][`AsyncOpenOptionsTrait::write()`] mode.
///
/// The [`append`][`AsyncOpenOptionsTrait::append()`] option opens files in a
/// special writing mode that moves the file cursor to the end of file before
/// every write operation.
///
/// It is also possible to [`truncate`][`AsyncOpenOptionsTrait::truncate()`] the
/// file right after opening, to
/// [`create`][`AsyncOpenOptionsTrait::create()`] a file if it doesn't exist
/// yet, or to always create a new file with
/// [`create_new`][`AsyncOpenOptionsTrait::create_new()`].
#[async_trait]
pub trait AsyncOpenOptionsTrait: std::default::Default {
    /// Creates a blank set of options.
    ///
    /// All options are initially set to `false`.
    async fn new<T>() -> T
        where T: AsyncOpenOptionsTrait;

    /// Configures the option for read mode.
    ///
    /// When set to `true`, this option means the file will be readable after
    /// opening.
    async fn read(&mut self, read: bool) -> &mut Self;

    /// Configures the option for write mode.
    ///
    /// When set to `true`, this option means the file will be writable after
    /// opening.
    ///
    /// If the file already exists, write calls on it will overwrite the
    /// previous contents without truncating it.
    async fn write(&mut self, write: bool) -> &mut Self;

    /// Configures the option for append mode.
    ///
    /// When set to `true`, this option means the file will be writable after
    /// opening and the file cursor will be moved to the end of file before
    /// every write operation.
    async fn append(&mut self, append: bool) -> &mut Self;

    /// Configures the option for truncating the previous file.
    ///
    /// When set to `true`, the file will be truncated to the length of 0 bytes.
    ///
    /// The file must be opened in [`write`][`AsyncOpenOptionsTrait::write()`]
    /// or [`append`][`AsyncOpenOptionsTrait::append()`] mode for truncation to
    /// work.
    async fn truncate(&mut self, truncate: bool) -> &mut Self;

    /// Configures the option for creating a new file if it doesn't exist.
    ///
    /// When set to `true`, this option means a new file will be created if it
    /// doesn't exist.
    ///
    /// The file must be opened in [`write`][`AsyncOpenOptionsTrait::write()`]
    /// or [`append`][`AsyncOpenOptionsTrait::append()`] mode for file creation
    /// to work.
    async fn create(&mut self, create: bool) -> &mut Self;

    /// Configures the option for creating a new file or failing if it already
    /// exists.
    ///
    /// When set to `true`, this option means a new file will be created, or the
    /// open operation will fail if the file already exists.
    ///
    /// The file must be opened in [`write`][`AsyncOpenOptionsTrait::write()`]
    /// or [`append`][`AsyncOpenOptionsTrait::append()`] mode for file creation
    /// to work.
    async fn create_new(&mut self, create_new: bool) -> &mut Self;

    /// Opens a file with the configured options.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The file does not exist and neither [`create`] nor [`create_new`] were
    ///   set.
    /// * The file's parent directory does not exist.
    /// * The current process lacks permissions to open the file in the
    ///   configured mode.
    /// * The file already exists and [`create_new`] was set.
    /// * Invalid combination of options was used, like [`truncate`] was set but
    ///   [`write`] wasn't,
    ///   or none of [`read`], [`write`], and [`append`] modes was set.
    /// * An OS-level occurred, like too many files are open or the file name is
    ///   too long.
    /// * Some other I/O error occurred.
    ///
    /// [`read`]: `AsyncOpenOptionsTrait::read()`
    /// [`write`]: `AsyncOpenOptionsTrait::write()`
    /// [`append`]: `AsyncOpenOptionsTrait::append()`
    /// [`truncate`]: `AsyncOpenOptionsTrait::truncate()`
    /// [`create`]: `AsyncOpenOptionsTrait::create()`
    /// [`create_new`]: `AsyncOpenOptionsTrait::create_new()`
    async fn open<P: AsRef<Path>, T>(&self,
                                     path: P)
                                     -> dyn Future<Output = io::Result<T>>
        where T: AsyncFileTrait;
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
