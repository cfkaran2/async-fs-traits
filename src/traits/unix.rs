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
    pub async fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P,
                                                         dst: Q)
                                                         -> io::Result<()> {
        let src = src.as_ref().to_owned();
        let dst = dst.as_ref().to_owned();
        unblock(move || std::os::unix::fs::symlink(&src, &dst)).await
    }

    /// Unix-specific extensions to [`AsyncDirBuilderTrait`].
    pub trait DirBuilderExt {
        /// Sets the mode to create new directories with.
        ///
        /// This option defaults to `0o777`.
        fn mode(&mut self, mode: u32) -> &mut Self;
    }

    /// Unix-specific extension methods for [`AsyncDirEntryTrait`].
    pub trait DirEntryExt {
        /// Returns the underlying `d_ino` field in the contained `dirent`
        /// structure.
        fn ino(&self) -> u64;
    }

    /// Unix-specific extensions to [`AsyncOpenOptionsTrait`].
    pub trait OpenOptionsExt {
        /// Sets the mode bits that a new file will be created with.
        ///
        /// If a new file is created as part of an
        /// [`AsyncOpenOptionsTrait::open()`] call then this specified `mode`
        /// will be used as the permission bits for the new file.
        ///
        /// If no `mode` is set, the default of `0o666` will be used. The
        /// operating system masks out bits with the system's `umask`, to
        /// produce the final permissions.
        fn mode(&mut self, mode: u32) -> &mut Self;

        /// Passes custom flags to the `flags` argument of `open`.
        ///
        /// The bits that define the access mode are masked out with
        /// `O_ACCMODE`, to ensure they do not interfere with the access mode
        /// set by Rust's options.
        ///
        /// Custom flags can only set flags, not remove flags set by Rust's
        /// options. This options overwrites any previously set custom flags.
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
