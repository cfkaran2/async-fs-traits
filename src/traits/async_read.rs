//! TODO: Fill this in

use std::{
    io,
    io::IoSliceMut,
    pin::Pin,
    task::{Context, Poll}
};

/// Read bytes asynchronously.
///
/// This trait is analogous to the `std::io::Read` trait, but integrates
/// with the asynchronous task system. In particular, the `poll_read`
/// method, unlike `Read::read`, will automatically queue the current task
/// for wakeup and return if data is not yet available, rather than blocking
/// the calling thread.
pub trait AsyncRead {
    /// Attempt to read from the `AsyncRead` into `buf`.
    ///
    /// On success, returns `Poll::Ready(Ok(num_bytes_read))`.
    ///
    /// If no data is available for reading, the method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object becomes
    /// readable or is closed.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    fn poll_read(self: Pin<&mut Self>,
                 cx: &mut Context<'_>,
                 buf: &mut [u8])
                 -> Poll<io::Result<usize>>;

    /// Attempt to read from the `AsyncRead` into `bufs` using vectored
    /// IO operations.
    ///
    /// This method is similar to `poll_read`, but allows data to be read
    /// into multiple buffers using a single operation.
    ///
    /// On success, returns `Poll::Ready(Ok(num_bytes_read))`.
    ///
    /// If no data is available for reading, the method returns
    /// `Poll::Pending` and arranges for the current task (via
    /// `cx.waker().wake_by_ref()`) to receive a notification when the object becomes
    /// readable or is closed.
    /// By default, this method delegates to using `poll_read` on the first
    /// nonempty buffer in `bufs`, or an empty one if none exists. Objects which
    /// support vectored IO should override this method.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    fn poll_read_vectored(self: Pin<&mut Self>,
                          cx: &mut Context<'_>,
                          bufs: &mut [IoSliceMut<'_>])
                          -> Poll<io::Result<usize>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_read(cx, b);
            }
        }

        self.poll_read(cx, &mut [])
    }
}

#[cfg(feature = "futures-lite")]
impl<T> AsyncRead for T where T: futures_lite::io::AsyncRead
{
    fn poll_read(self: Pin<&mut Self>,
                 cx: &mut Context<'_>,
                 buf: &mut [u8])
                 -> Poll<io::Result<usize>> {
        (self as Pin<&mut T>).poll_read(cx, buf)
    }

    fn poll_read_vectored(self: Pin<&mut Self>,
                          cx: &mut Context<'_>,
                          bufs: &mut [IoSliceMut<'_>])
                          -> Poll<io::Result<usize>> {
        (self as Pin<&mut T>).poll_read_vectored(cx, bufs)
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
