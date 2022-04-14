//! TODO: Fill this in

pub use std::fs::{FileType, Metadata, Permissions};
use std::{
    future::Future,
    io::{self},
    path::Path
};

use blocking::unblock;

#[doc(no_inline)]
use crate::File;

/// A builder for opening files with configurable options.
///
/// Files can be opened in [`read`][`OpenOptions::read()`] and/or
/// [`write`][`OpenOptions::write()`] mode.
///
/// The [`append`][`OpenOptions::append()`] option opens files in a special
/// writing mode that moves the file cursor to the end of file before every
/// write operation.
///
/// It is also possible to [`truncate`][`OpenOptions::truncate()`] the file
/// right after opening, to [`create`][`OpenOptions::create()`] a file if it
/// doesn't exist yet, or to always create a new file with [`create_new`]
/// [`OpenOptions::create_new()`].
///
/// # Examples
///
/// Open a file for reading:
///
/// ```no_run
/// use async_fs::OpenOptions;
///
/// # futures_lite::future::block_on(async {
/// let file = OpenOptions::new().read(true).open("a.txt").await?;
/// # std::io::Result::Ok(()) });
/// ```
///
/// Open a file for both reading and writing, and create it if it doesn't exist
/// yet:
///
/// ```no_run
/// use async_fs::OpenOptions;
///
/// # futures_lite::future::block_on(async {
/// let file = OpenOptions::new().read(true)
///                              .write(true)
///                              .create(true)
///                              .open("a.txt")
///                              .await?;
/// # std::io::Result::Ok(()) });
/// ```
#[derive(Clone, Debug)]
pub struct OpenOptions(std::fs::OpenOptions);

impl OpenOptions {
    /// Creates a blank set of options.
    ///
    /// All options are initially set to `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::OpenOptions;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = OpenOptions::new().read(true).open("a.txt").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn new() -> OpenOptions {
        OpenOptions(std::fs::OpenOptions::new())
    }

    /// Configures the option for read mode.
    ///
    /// When set to `true`, this option means the file will be readable after
    /// opening.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::OpenOptions;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = OpenOptions::new().read(true).open("a.txt").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.0.read(read);
        self
    }

    /// Configures the option for write mode.
    ///
    /// When set to `true`, this option means the file will be writable after
    /// opening.
    ///
    /// If the file already exists, write calls on it will overwrite the
    /// previous contents without truncating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::OpenOptions;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = OpenOptions::new().write(true).open("a.txt").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.0.write(write);
        self
    }

    /// Configures the option for append mode.
    ///
    /// When set to `true`, this option means the file will be writable after
    /// opening and the file cursor will be moved to the end of file before
    /// every write operaiton.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::OpenOptions;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = OpenOptions::new().append(true).open("a.txt").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.0.append(append);
        self
    }

    /// Configures the option for truncating the previous file.
    ///
    /// When set to `true`, the file will be truncated to the length of 0 bytes.
    ///
    /// The file must be opened in [`write`][`OpenOptions::write()`] or
    /// [`append`][`OpenOptions::append()`] mode for truncation to work.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::OpenOptions;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = OpenOptions::new().write(true)
    ///                              .truncate(true)
    ///                              .open("a.txt")
    ///                              .await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.0.truncate(truncate);
        self
    }

    /// Configures the option for creating a new file if it doesn't exist.
    ///
    /// When set to `true`, this option means a new file will be created if it
    /// doesn't exist.
    ///
    /// The file must be opened in [`write`][`OpenOptions::write()`] or
    /// [`append`][`OpenOptions::append()`] mode for file creation to work.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::OpenOptions;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = OpenOptions::new().write(true)
    ///                              .create(true)
    ///                              .open("a.txt")
    ///                              .await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.0.create(create);
        self
    }

    /// Configures the option for creating a new file or failing if it already
    /// exists.
    ///
    /// When set to `true`, this option means a new file will be created, or the
    /// open operation will fail if the file already exists.
    ///
    /// The file must be opened in [`write`][`OpenOptions::write()`] or
    /// [`append`][`OpenOptions::append()`] mode for file creation to work.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::OpenOptions;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = OpenOptions::new().write(true)
    ///                              .create_new(true)
    ///                              .open("a.txt")
    ///                              .await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions {
        self.0.create_new(create_new);
        self
    }

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
    /// [`read`]: `OpenOptions::read()`
    /// [`write`]: `OpenOptions::write()`
    /// [`append`]: `OpenOptions::append()`
    /// [`truncate`]: `OpenOptions::truncate()`
    /// [`create`]: `OpenOptions::create()`
    /// [`create_new`]: `OpenOptions::create_new()`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::OpenOptions;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = OpenOptions::new().read(true).open("a.txt").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn open<P: AsRef<Path>>(&self,
                                path: P)
                                -> impl Future<Output = io::Result<File>> {
        let path = path.as_ref().to_owned();
        let options = self.0.clone();
        async move {
            let file = unblock(move || options.open(path)).await?;
            Ok(File::new(file, false))
        }
    }
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(unix)]
impl std::os::unix::fs::OpenOptionsExt for OpenOptions {
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.0.mode(mode);
        self
    }

    fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.0.custom_flags(flags);
        self
    }
}

#[cfg(windows)]
impl windows::OpenOptionsExt for OpenOptions {
    fn access_mode(&mut self, access: u32) -> &mut Self {
        self.0.access_mode(access);
        self
    }

    fn share_mode(&mut self, val: u32) -> &mut Self {
        self.0.share_mode(val);
        self
    }

    fn custom_flags(&mut self, flags: u32) -> &mut Self {
        self.0.custom_flags(flags);
        self
    }

    fn attributes(&mut self, val: u32) -> &mut Self {
        self.0.attributes(val);
        self
    }

    fn security_qos_flags(&mut self, flags: u32) -> &mut Self {
        self.0.security_qos_flags(flags);
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
