//! TODO: Fill this in

/// Unix-specific extensions.
#[cfg(unix)]
pub mod unix {
    #[doc(no_inline)]
    pub use std::fs::{FileType, Metadata, Permissions};
    use std::io;
    pub use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
    #[cfg(windows)]
    use std::os::windows::fs::OpenOptionsExt as _;
    #[doc(no_inline)]
    use std::path::Path;

    use blocking::unblock;

    /// Creates a new symbolic link on the filesystem.
    ///
    /// The `dst` path will be a symbolic link pointing to the `src` path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// async_fs::unix::symlink("a.txt", "b.txt").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P,
                                                         dst: Q)
                                                         -> io::Result<()> {
        let src = src.as_ref().to_owned();
        let dst = dst.as_ref().to_owned();
        unblock(move || std::os::unix::fs::symlink(&src, &dst)).await
    }

    /// Unix-specific extensions to [`DirBuilder`].
    pub trait DirBuilderExt {
        /// Sets the mode to create new directories with.
        ///
        /// This option defaults to `0o777`.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use async_fs::{unix::DirBuilderExt, DirBuilder};
        ///
        /// let mut builder = DirBuilder::new();
        /// builder.mode(0o755);
        /// ```
        fn mode(&mut self, mode: u32) -> &mut Self;
    }

    /// Unix-specific extension methods for [`DirEntry`].
    pub trait DirEntryExt {
        /// Returns the underlying `d_ino` field in the contained `dirent`
        /// structure.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use async_fs::unix::DirEntryExt;
        /// use futures_lite::stream::StreamExt;
        ///
        /// # futures_lite::future::block_on(async {
        /// let mut entries = async_fs::read_dir(".").await?;
        ///
        /// while let Some(entry) = entries.try_next().await? {
        ///     println!("{:?}: {}", entry.file_name(), entry.ino());
        /// }
        /// # std::io::Result::Ok(()) });
        /// ```
        fn ino(&self) -> u64;
    }

    /// Unix-specific extensions to [`OpenOptions`].
    pub trait OpenOptionsExt {
        /// Sets the mode bits that a new file will be created with.
        ///
        /// If a new file is created as part of an [`OpenOptions::open()`] call
        /// then this specified `mode` will be used as the permission bits for
        /// the new file.
        ///
        /// If no `mode` is set, the default of `0o666` will be used. The
        /// operating system masks out bits with the system's `umask`, to
        /// produce the final permissions.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use async_fs::{unix::OpenOptionsExt, OpenOptions};
        ///
        /// # futures_lite::future::block_on(async {
        /// let mut options = OpenOptions::new();
        /// // Read/write permissions for owner and read permissions for others.
        /// options.mode(0o644);
        /// let file = options.open("foo.txt").await?;
        /// # std::io::Result::Ok(()) });
        /// ```
        fn mode(&mut self, mode: u32) -> &mut Self;

        /// Passes custom flags to the `flags` argument of `open`.
        ///
        /// The bits that define the access mode are masked out with
        /// `O_ACCMODE`, to ensure they do not interfere with the access mode
        /// set by Rust's options.
        ///
        /// Custom flags can only set flags, not remove flags set by Rust's
        /// options. This options overwrites any previously set custom flags.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use async_fs::{unix::OpenOptionsExt, OpenOptions};
        ///
        /// # futures_lite::future::block_on(async {
        /// let mut options = OpenOptions::new();
        /// options.write(true);
        /// options.custom_flags(libc::O_NOFOLLOW);
        /// let file = options.open("foo.txt").await?;
        /// # std::io::Result::Ok(()) });
        /// ```
        fn custom_flags(&mut self, flags: i32) -> &mut Self;
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
}
