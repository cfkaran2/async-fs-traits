//! TODO: Fill this in

#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};
use std::{
    future::Future,
    io::{self},
    path::Path
};

use async_trait::async_trait;

/// A builder for creating directories with configurable options.
///
/// For Unix-specific options, import the
/// [`DirBuilderExt`][`std::os::unix::fs::DirBuilderExt`] trait.
#[async_trait]
pub trait DirBuilder: std::fmt::Debug + std::default::Default {
    /// Creates a blank set of options.
    ///
    /// The [`recursive()`][`DirBuilder::recursive()`] option is initially set
    /// to `false`.
    async fn new<T>() -> T where T:DirBuilder;

    /// Sets the option for recursive mode.
    ///
    /// When set to `true`, this option means all parent directories should be
    /// created recursively if they don't exist. Parents are created with the
    /// same permissions as the final directory.
    ///
    /// This option is initially set to `false`.
    async fn recursive(&mut self, recursive: bool) -> &mut Self;

    /// Creates a directory with the configured options.
    ///
    /// It is considered an error if the directory already exists unless
    /// recursive mode is enabled.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` already points to an existing file or directory.
    /// * The current process lacks permissions to create the directory or its
    ///   missing parents.
    /// * Some other I/O error occurred.
    async fn create<P: AsRef<Path>>(&self,
                                  path: P)
                                  -> dyn Future<Output = io::Result<()>>;
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
