//! TODO: Fill this in

#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};
use std::{
    future::Future,
    io::{self},
    path::Path
};

use blocking::unblock;

/// A builder for creating directories with configurable options.
///
/// For Unix-specific options, import the
/// [`DirBuilderExt`][`std::os::unix::fs::DirBuilderExt`] trait.
#[derive(Debug, Default)]
pub struct DirBuilder {
    /// Set to `true` if non-existent parent directories should be created.
    recursive: bool,

    /// Unix mode for newly created directories.
    #[cfg(unix)]
    mode: Option<u32>
}

impl DirBuilder {
    /// Creates a blank set of options.
    ///
    /// The [`recursive()`][`DirBuilder::recursive()`] option is initially set
    /// to `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_fs::DirBuilder;
    ///
    /// let builder = DirBuilder::new();
    /// ```
    pub fn new() -> DirBuilder {
        #[cfg(not(unix))]
        let builder = DirBuilder { recursive: false };

        #[cfg(unix)]
        let builder = DirBuilder { recursive: false,
                                   mode: None };

        builder
    }

    /// Sets the option for recursive mode.
    ///
    /// When set to `true`, this option means all parent directories should be
    /// created recursively if they don't exist. Parents are created with the
    /// same permissions as the final directory.
    ///
    /// This option is initially set to `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_fs::DirBuilder;
    ///
    /// let mut builder = DirBuilder::new();
    /// builder.recursive(true);
    /// ```
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.recursive = recursive;
        self
    }

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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::DirBuilder;
    ///
    /// # futures_lite::future::block_on(async {
    /// DirBuilder::new().recursive(true)
    ///                  .create("./some/directory")
    ///                  .await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn create<P: AsRef<Path>>(&self,
                                  path: P)
                                  -> impl Future<Output = io::Result<()>> {
        let mut builder = std::fs::DirBuilder::new();
        builder.recursive(self.recursive);

        #[cfg(unix)]
        {
            if let Some(mode) = self.mode {
                std::os::unix::fs::DirBuilderExt::mode(&mut builder, mode);
            }
        }

        let path = path.as_ref().to_owned();
        unblock(move || builder.create(path))
    }
}

#[cfg(unix)]
impl std::os::unix::fs::DirBuilderExt for DirBuilder {
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.mode = Some(mode);
        self
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
