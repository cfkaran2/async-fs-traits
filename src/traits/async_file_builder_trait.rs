//! TODO: Fill this in

pub use std::fs::{FileType, Metadata, Permissions};
use std::{future::Future, io, path::Path};

use async_trait::async_trait;

#[doc(no_inline)]
use crate::AsyncFileTrait;

/// A builder for opening files with configurable options.
///
/// Files can be opened in [`read`][1] and/or [`write`][2] mode.
///
/// The [`append`][3] option opens files in a special writing mode that moves
/// the file cursor to the end of file before every write operation.
///
/// It is also possible to [`truncate`][4] the file right after opening, to
/// [`create`][5] a file if it doesn't exist yet, or to always create a new
/// file with[`create_new`][6].
///
/// [1]: AsyncFileBuilderTrait::read()
/// [2]: AsyncFileBuilderTrait::write()
/// [3]: AsyncFileBuilderTrait::append()
/// [4]: AsyncFileBuilderTrait::truncate()
/// [5]: AsyncFileBuilderTrait::create()
/// [6]: AsyncFileBuilderTrait::create_new()
#[async_trait]
pub trait AsyncFileBuilderTrait: std::default::Default {
    /// Creates a blank set of options.
    ///
    /// All options are initially set to `false`.
    async fn new<T>() -> T
        where T: AsyncFileBuilderTrait;

    /// Configures the option for read mode.
    ///
    /// When set to `true`, this option means the file will be readable after
    /// opening.
    async fn read<T>(self, read: bool) -> T
        where T: AsyncFileBuilderTrait;

    /// Configures the option for write mode.
    ///
    /// When set to `true`, this option means the file will be writable after
    /// opening.
    ///
    /// If the file already exists, write calls on it will overwrite the
    /// previous contents without truncating it.
    async fn write<T>(self, write: bool) -> T
        where T: AsyncFileBuilderTrait;

    /// Configures the option for append mode.
    ///
    /// When set to `true`, this option means the file will be writable after
    /// opening and the file cursor will be moved to the end of file before
    /// every write operation.
    async fn append<T>(self, append: bool) -> T
        where T: AsyncFileBuilderTrait;

    /// Configures the option for truncating the previous file.
    ///
    /// When set to `true`, the file will be truncated to the length of 0 bytes.
    ///
    /// The file must be opened in [`write`][1] or [`append`][2] mode for
    /// truncation to work.
    ///
    /// [1]: AsyncFileBuilderTrait::write()
    /// [2]: AsyncFileBuilderTrait::append()
    async fn truncate<T>(self, truncate: bool) -> T
        where T: AsyncFileBuilderTrait;

    /// Configures the option for creating a new file if it doesn't exist.
    ///
    /// When set to `true`, this option means a new file will be created if it
    /// doesn't exist.
    ///
    /// The file must be opened in [`write`][1] or [`append`][2] mode for file
    /// creation to work.
    ///
    /// [1]: AsyncFileBuilderTrait::write()
    /// [2]: AsyncFileBuilderTrait::append()
    async fn create<T>(self, create: bool) -> T
        where T: AsyncFileBuilderTrait;

    /// Configures the option for creating a new file or failing if it already
    /// exists.
    ///
    /// When set to `true`, this option means a new file will be created, or the
    /// open operation will fail if the file already exists.
    ///
    /// The file must be opened in [`write`][1] or [`append`][2] mode for file
    /// creation to work.
    ///
    /// [1]: AsyncFileBuilderTrait::write()
    /// [2]: AsyncFileBuilderTrait::append()
    async fn create_new<T>(self, create_new: bool) -> T
        where T: AsyncFileBuilderTrait;

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
    /// [`read`]: `AsyncFileBuilderTrait::read()`
    /// [`write`]: `AsyncFileBuilderTrait::write()`
    /// [`append`]: `AsyncFileBuilderTrait::append()`
    /// [`truncate`]: `AsyncFileBuilderTrait::truncate()`
    /// [`create`]: `AsyncFileBuilderTrait::create()`
    /// [`create_new`]: `AsyncFileBuilderTrait::create_new()`
    async fn open<P, T>(self, path: P) -> dyn Future<Output = io::Result<T>>
        where P: AsRef<Path>,
              T: AsyncFileTrait;
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
