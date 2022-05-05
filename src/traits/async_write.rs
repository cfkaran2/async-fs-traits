//! TODO: Fill this in

use std::{
    io,
    io::IoSlice,
    pin::Pin,
    task::{Context, Poll}
};

/// Write bytes asynchronously.
///
/// This trait is analogous to the `std::io::Write` trait, but integrates
/// with the asynchronous task system. In particular, the `poll_write`
/// method, unlike `Write::write`, will automatically queue the current task
/// for wakeup and return if the writer cannot take more data, rather than blocking
/// the calling thread.
pub trait AsyncWrite {
    /// Attempt to write bytes from `buf` into the object.
    ///
    /// On success, returns `Poll::Ready(Ok(num_bytes_written))`.
    ///
    /// If the object is not ready for writing, the method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object becomes
    /// writable or is closed.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    ///
    /// `poll_write` must try to make progress by flushing the underlying object if
    /// that is the only way the underlying object can become writable again.
    fn poll_write(self: Pin<&mut Self>,
                  cx: &mut Context<'_>,
                  buf: &[u8])
                  -> Poll<io::Result<usize>>;

    /// Attempt to write bytes from `bufs` into the object using vectored
    /// IO operations.
    ///
    /// This method is similar to `poll_write`, but allows data from multiple buffers to be written
    /// using a single operation.
    ///
    /// On success, returns `Poll::Ready(Ok(num_bytes_written))`.
    ///
    /// If the object is not ready for writing, the method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object becomes
    /// writable or is closed.
    ///
    /// By default, this method delegates to using `poll_write` on the first
    /// nonempty buffer in `bufs`, or an empty one if none exists. Objects which
    /// support vectored IO should override this method.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    fn poll_write_vectored(self: Pin<&mut Self>,
                           cx: &mut Context<'_>,
                           bufs: &[IoSlice<'_>])
                           -> Poll<io::Result<usize>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_write(cx, b);
            }
        }

        self.poll_write(cx, &[])
    }

    /// Attempt to flush the object, ensuring that any buffered data reach
    /// their destination.
    ///
    /// On success, returns `Poll::Ready(Ok(()))`.
    ///
    /// If flushing cannot immediately complete, this method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object can make
    /// progress towards flushing.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    ///
    /// It only makes sense to do anything here if you actually buffer data.
    fn poll_flush(self: Pin<&mut Self>,
                  cx: &mut Context<'_>)
                  -> Poll<io::Result<()>>;

    /// Attempt to close the object.
    ///
    /// On success, returns `Poll::Ready(Ok(()))`.
    ///
    /// If closing cannot immediately complete, this function returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object can make
    /// progress towards closing.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    fn poll_close(self: Pin<&mut Self>,
                  cx: &mut Context<'_>)
                  -> Poll<io::Result<()>>;
}

#[cfg(feature = "futures-lite")]
impl<T> AsyncWrite for T where T: futures_lite::io::AsyncWrite
{
    fn poll_write(self: Pin<&mut Self>,
                  cx: &mut Context<'_>,
                  buf: &[u8])
                  -> Poll<io::Result<usize>> {
        (self as Pin<&mut T>).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>,
                  cx: &mut Context<'_>)
                  -> Poll<io::Result<()>> {
        (self as Pin<&mut T>).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>,
                  cx: &mut Context<'_>)
                  -> Poll<io::Result<()>> {
        (self as Pin<&mut T>).poll_close(cx)
    }

    fn poll_write_vectored(self: Pin<&mut Self>,
                           cx: &mut Context<'_>,
                           bufs: &[IoSlice<'_>])
                           -> Poll<io::Result<usize>> {
        (self as Pin<&mut T>).poll_write_vectored(cx, bufs)
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
