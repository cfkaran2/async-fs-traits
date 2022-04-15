//! TODO: Fill this in

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt as _;

/// Windows-specific extensions.
#[cfg(windows)]
pub mod windows {
    #[doc(no_inline)]
    pub use std::fs::{FileType, Metadata, Permissions};
    #[cfg(unix)]
    use std::os::unix::fs::{DirEntryExt as _, OpenOptionsExt as _};
    #[doc(no_inline)]
    pub use std::os::windows::fs::MetadataExt;
    #[cfg(windows)]
    use std::os::windows::fs::OpenOptionsExt as _;
    use std::{
        ffi::OsString,
        fmt,
        future::Future,
        io::{self, SeekFrom},
        path::{Path, PathBuf},
        pin::Pin,
        sync::Arc,
        task::{Context, Poll}
    };

    use async_lock::Mutex;
    use blocking::{unblock, Unblock};
    use futures_lite::{
        future,
        io::{AsyncRead, AsyncSeek, AsyncWrite, AsyncWriteExt},
        ready,
        stream::Stream
    };

    use super::*;
    /// Creates a new directory symbolic link on the filesystem.
    ///
    /// The `dst` path will be a directory symbolic link pointing to the `src`
    /// path.
    pub async fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(
        src: P,
        dst: Q)
        -> io::Result<()> {
        let src = src.as_ref().to_owned();
        let dst = dst.as_ref().to_owned();
        unblock(move || std::os::windows::fs::symlink_dir(&src, &dst)).await
    }

    /// Creates a new file symbolic link on the filesystem.
    ///
    /// The `dst` path will be a file symbolic link pointing to the `src` path.
    pub async fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(
        src: P,
        dst: Q)
        -> io::Result<()> {
        let src = src.as_ref().to_owned();
        let dst = dst.as_ref().to_owned();
        unblock(move || std::os::windows::fs::symlink_file(&src, &dst)).await
    }

    /// Windows-specific extensions to [`OpenOptions`].
    pub trait OpenOptionsExt {
        /// Overrides the `dwDesiredAccess` argument to the call to
        /// [`CreateFile`] with the specified value.
        ///
        /// This will override the `read`, `write`, and `append` flags on the
        /// [`OpenOptions`] structure. This method provides fine-grained
        /// control over the permissions to read, write and append data,
        /// attributes (like hidden and system), and extended attributes.
        ///
        /// [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
        fn access_mode(&mut self, access: u32) -> &mut Self;

        /// Overrides the `dwShareMode` argument to the call to
        /// [`CreateFile`] with the specified value.
        ///
        /// By default `share_mode` is set to `FILE_SHARE_READ |
        /// FILE_SHARE_WRITE | FILE_SHARE_DELETE`. This allows other processes
        /// to read, write, and delete/rename the same file while it is open.
        /// Removing any of the flags will prevent other processes from
        /// performing the corresponding operation until the file handle is
        /// closed.
        ///
        /// [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
        fn share_mode(&mut self, val: u32) -> &mut Self;

        /// Sets extra flags for the `dwFileFlags` argument to the call to
        /// [`CreateFile2`] to the specified value (or combines it with
        /// `attributes` and `security_qos_flags` to set the
        /// `dwFlagsAndAttributes` for [`CreateFile`]).
        ///
        /// Custom flags can only set flags, not remove flags set by Rust's
        /// options. This option overwrites any previously set custom flags.
        ///
        /// [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
        /// [`CreateFile2`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfile2
        fn custom_flags(&mut self, flags: u32) -> &mut Self;

        /// Sets the `dwFileAttributes` argument to the call to
        /// [`CreateFile2`] to the specified value (or combines it with
        /// `custom_flags` and `security_qos_flags` to set the
        /// `dwFlagsAndAttributes` for[`CreateFile`]).
        ///
        /// If a _new_ file is created because it does not yet exist and
        /// `.create(true)` or `.create_new(true)` are specified, the new file
        /// is given the attributes declared with `.attributes()`.
        ///
        /// If an _existing_ file is opened with `.create(true).truncate
        /// (true)`, its existing attributes are preserved and combined with
        /// the ones declared with `.attributes()`.
        ///
        /// In all other cases the attributes get ignored.
        ///
        /// [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
        /// [`CreateFile2`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfile2
        fn attributes(&mut self, val: u32) -> &mut Self;

        /// Sets the `dwSecurityQosFlags` argument to the call to
        /// [`CreateFile2`] to the specified value (or combines it with
        /// `custom_flags` and `attributes` to set the `dwFlagsAndAttributes`
        /// for [`CreateFile`]).
        ///
        /// By default `security_qos_flags` is not set. It should be specified
        /// when opening a named pipe, to control to which degree a server
        /// process can act on behalf of a client process
        /// (security impersonation level).
        ///
        /// When `security_qos_flags` is not set, a malicious program can gain
        /// the elevated privileges of a privileged Rust process when it allows
        /// opening user-specified paths, by tricking it into opening a named
        /// pipe. So arguably `security_qos_flags` should also be set when
        /// opening arbitrary paths. However the bits can then conflict with
        /// other flags, specifically `FILE_FLAG_OPEN_NO_RECALL`.
        ///
        /// For information about possible values, see [Impersonation Levels] on
        /// the Windows Dev Center site. The `SECURITY_SQOS_PRESENT` flag is
        /// set automatically when using this method.
        ///
        /// [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
        /// [`CreateFile2`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfile2
        /// [Impersonation Levels]: https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-security_impersonation_level
        fn security_qos_flags(&mut self, flags: u32) -> &mut Self;
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
