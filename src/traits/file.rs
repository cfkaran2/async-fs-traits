//! TODO: Fill this in

pub use std::fs::{FileType, Metadata, Permissions};
use std::{
    fmt,
    io::{self, SeekFrom},
    path::Path,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use async_lock::Mutex;
use blocking::{unblock, Unblock};
use futures_lite::{
    io::{AsyncRead, AsyncSeek, AsyncWrite, AsyncWriteExt},
    ready
};

#[doc(no_inline)]
use crate::arc_file::ArcFile;

/// An open file on the filesystem.
///
/// Depending on what options the file was opened with, this type can be used
/// for reading and/or writing.
///
/// Files are automatically closed when they get dropped and any errors detected
/// on closing are ignored. Use the [`sync_all()`][`File::sync_all()`] method
/// before dropping a file if such errors need to be handled.
///
/// **NOTE:** If writing to a file, make sure to call
/// [`flush()`][`futures_lite::io::AsyncWriteExt::flush()`],
/// [`sync_data()`][`File::sync_data()`], or [`sync_all()`][`File::sync_all()`]
/// before dropping the file or else some written data might get lost!
///
/// # Examples
///
/// Create a new file and write some bytes to it:
///
/// ```no_run
/// use async_fs::File;
/// use futures_lite::io::AsyncWriteExt;
///
/// # futures_lite::future::block_on(async {
/// let mut file = File::create("a.txt").await?;
///
/// file.write_all(b"Hello, world!").await?;
/// file.flush().await?;
/// # std::io::Result::Ok(()) });
/// ```
///
/// Read the contents of a file into a vector of bytes:
///
/// ```no_run
/// use async_fs::File;
/// use futures_lite::io::AsyncReadExt;
///
/// # futures_lite::future::block_on(async {
/// let mut file = File::open("a.txt").await?;
///
/// let mut contents = Vec::new();
/// file.read_to_end(&mut contents).await?;
/// # std::io::Result::Ok(()) });
/// ```
pub struct File {
    /// Always accessible reference to the file.
    file: Arc<std::fs::File>,

    /// Performs blocking I/O operations on a thread pool.
    unblock: Mutex<Unblock<ArcFile>>,

    /// Logical file cursor, tracked when reading from the file.
    ///
    /// This will be set to an error if the file is not seekable.
    read_pos: Option<io::Result<u64>>,

    /// Set to `true` if the file needs flushing.
    is_dirty: bool
}

impl File {
    /// Creates an async file from a blocking file.
    pub fn new(inner: std::fs::File, is_dirty: bool) -> File {
        let file = Arc::new(inner);
        let unblock = Mutex::new(Unblock::new(ArcFile::new(file.clone())));
        let read_pos = None;
        File { file,
               unblock,
               read_pos,
               is_dirty }
    }

    /// Opens a file in read-only mode.
    ///
    /// See the [`OpenOptions::open()`] function for more options.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to read the file.
    /// * Some other I/O error occurred.
    ///
    /// For more details, see the list of errors documented by
    /// [`OpenOptions::open()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::File;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = File::open("a.txt").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let path = path.as_ref().to_owned();
        let file = unblock(move || std::fs::File::open(&path)).await?;
        Ok(File::new(file, false))
    }

    /// Opens a file in write-only mode.
    ///
    /// This method will create a file if it does not exist, and will truncate
    /// it if it does.
    ///
    /// See the [`OpenOptions::open`] function for more options.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The file's parent directory does not exist.
    /// * The current process lacks permissions to write to the file.
    /// * Some other I/O error occurred.
    ///
    /// For more details, see the list of errors documented by
    /// [`OpenOptions::open()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::File;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = File::create("a.txt").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn create<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let path = path.as_ref().to_owned();
        let file = unblock(move || std::fs::File::create(&path)).await?;
        Ok(File::new(file, false))
    }

    /// Synchronizes OS-internal buffered contents and metadata to disk.
    ///
    /// This function will ensure that all in-memory data reaches the
    /// filesystem.
    ///
    /// This can be used to handle errors that would otherwise only be caught by
    /// closing the file. When a file is dropped, errors in synchronizing this
    /// in-memory data are ignored.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::File;
    /// use futures_lite::io::AsyncWriteExt;
    ///
    /// # futures_lite::future::block_on(async {
    /// let mut file = File::create("a.txt").await?;
    ///
    /// file.write_all(b"Hello, world!").await?;
    /// file.sync_all().await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn sync_all(&self) -> io::Result<()> {
        let mut inner = self.unblock.lock().await;
        inner.flush().await?;
        let file = self.file.clone();
        unblock(move || file.sync_all()).await
    }

    /// Synchronizes OS-internal buffered contents to disk.
    ///
    /// This is similar to [`sync_all()`][`File::sync_data()`], except that file
    /// metadata may not be synchronized.
    ///
    /// This is intended for use cases that must synchronize the contents of the
    /// file, but don't need the file metadata synchronized to disk.
    ///
    /// Note that some platforms may simply implement this in terms of
    /// [`sync_all()`][`File::sync_data()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::File;
    /// use futures_lite::io::AsyncWriteExt;
    ///
    /// # futures_lite::future::block_on(async {
    /// let mut file = File::create("a.txt").await?;
    ///
    /// file.write_all(b"Hello, world!").await?;
    /// file.sync_data().await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn sync_data(&self) -> io::Result<()> {
        let mut inner = self.unblock.lock().await;
        inner.flush().await?;
        let file = self.file.clone();
        unblock(move || file.sync_data()).await
    }

    /// Truncates or extends the file.
    ///
    /// If `size` is less than the current file size, then the file will be
    /// truncated. If it is greater than the current file size, then the file
    /// will be extended to `size` and have all intermediate data filled with
    /// zeros.
    ///
    /// The file's cursor stays at the same position, even if the cursor ends up
    /// being past the end of the file after this operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::File;
    ///
    /// # futures_lite::future::block_on(async {
    /// let mut file = File::create("a.txt").await?;
    /// file.set_len(10).await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        let mut inner = self.unblock.lock().await;
        inner.flush().await?;
        let file = self.file.clone();
        unblock(move || file.set_len(size)).await
    }

    /// Reads the file's metadata.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::File;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = File::open("a.txt").await?;
    /// let metadata = file.metadata().await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn metadata(&self) -> io::Result<Metadata> {
        let file = self.file.clone();
        unblock(move || file.metadata()).await
    }

    /// Changes the permissions on the file.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The current process lacks permissions to change attributes on the
    ///   file.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_fs::File;
    ///
    /// # futures_lite::future::block_on(async {
    /// let file = File::create("a.txt").await?;
    ///
    /// let mut perms = file.metadata().await?.permissions();
    /// perms.set_readonly(true);
    /// file.set_permissions(perms).await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        let file = self.file.clone();
        unblock(move || file.set_permissions(perm)).await
    }

    /// Repositions the cursor after reading.
    ///
    /// When reading from a file, actual file reads run asynchronously in the
    /// background, which means the real file cursor is usually ahead of the
    /// logical cursor, and the data between them is buffered in memory. This
    /// kind of buffering is an important optimization.
    ///
    /// After reading ends, if we decide to perform a write or a seek operation,
    /// the real file cursor must first be repositioned back to the correct
    /// logical position.
    fn poll_reposition(&mut self,
                       cx: &mut Context<'_>)
                       -> Poll<io::Result<()>> {
        if let Some(Ok(read_pos)) = self.read_pos {
            ready!(Pin::new(self.unblock.get_mut()).poll_seek(cx, SeekFrom::Start(read_pos)))?;
        }
        self.read_pos = None;
        Poll::Ready(Ok(()))
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.file.fmt(f)
    }
}

impl From<std::fs::File> for File {
    fn from(inner: std::fs::File) -> File {
        File::new(inner, true)
    }
}

#[cfg(unix)]
impl std::os::unix::io::FromRawFd for File {
    unsafe fn from_raw_fd(raw: std::os::unix::io::RawFd) -> File {
        File::from(std::fs::File::from_raw_fd(raw))
    }
}

#[cfg(windows)]
impl std::os::windows::io::FromRawHandle for File {
    unsafe fn from_raw_handle(raw: std::os::windows::io::RawHandle) -> File {
        File::from(std::fs::File::from_raw_handle(raw))
    }
}

#[cfg(unix)]
impl std::os::unix::io::AsRawFd for File {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.file.as_raw_fd()
    }
}

#[cfg(windows)]
impl std::os::windows::io::AsRawHandle for File {
    fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
        self.file.as_raw_handle()
    }
}

impl AsyncRead for File {
    fn poll_read(mut self: Pin<&mut Self>,
                 cx: &mut Context<'_>,
                 buf: &mut [u8])
                 -> Poll<io::Result<usize>> {
        // Before reading begins, remember the current cursor position.
        if self.read_pos.is_none() {
            // Initialize the logical cursor to the current position in the file.
            self.read_pos =
                Some(ready!(self.as_mut().poll_seek(cx, SeekFrom::Current(0))));
        }

        let n = ready!(Pin::new(self.unblock.get_mut()).poll_read(cx, buf))?;

        // Update the logical cursor if the file is seekable.
        if let Some(Ok(pos)) = self.read_pos.as_mut() {
            *pos += n as u64;
        }

        Poll::Ready(Ok(n))
    }
}

impl AsyncWrite for File {
    fn poll_write(mut self: Pin<&mut Self>,
                  cx: &mut Context<'_>,
                  buf: &[u8])
                  -> Poll<io::Result<usize>> {
        ready!(self.poll_reposition(cx))?;
        self.is_dirty = true;
        Pin::new(self.unblock.get_mut()).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>,
                  cx: &mut Context<'_>)
                  -> Poll<io::Result<()>> {
        if self.is_dirty {
            ready!(Pin::new(self.unblock.get_mut()).poll_flush(cx))?;
            self.is_dirty = false;
        }
        Poll::Ready(Ok(()))
    }

    fn poll_close(mut self: Pin<&mut Self>,
                  cx: &mut Context<'_>)
                  -> Poll<io::Result<()>> {
        Pin::new(self.unblock.get_mut()).poll_close(cx)
    }
}

impl AsyncSeek for File {
    fn poll_seek(mut self: Pin<&mut Self>,
                 cx: &mut Context<'_>,
                 pos: SeekFrom)
                 -> Poll<io::Result<u64>> {
        ready!(self.poll_reposition(cx))?;
        Pin::new(self.unblock.get_mut()).poll_seek(cx, pos)
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
