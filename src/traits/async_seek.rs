//! TODO: Fill this in

use std::{
    io,
    io::SeekFrom,
    pin::Pin,
    task::{Context, Poll}
};

/// Seek bytes asynchronously.
///
/// This trait is analogous to the `std::io::Seek` trait, but integrates
/// with the asynchronous task system. In particular, the `poll_seek`
/// method, unlike `Seek::seek`, will automatically queue the current task
/// for wakeup and return if data is not yet available, rather than blocking
/// the calling thread.
pub trait AsyncSeek {
    /// Attempt to seek to an offset, in bytes, in a stream.
    ///
    /// A seek beyond the end of a stream is allowed, but behavior is defined
    /// by the implementation.
    ///
    /// If the seek operation completed successfully,
    /// this method returns the new position from the start of the stream.
    /// That position can be used later with [`SeekFrom::Start`].
    ///
    /// # Errors
    ///
    /// Seeking to a negative offset is considered an error.
    ///
    /// # Implementation
    ///
    /// This function may not return errors of kind `WouldBlock` or
    /// `Interrupted`.  Implementations must convert `WouldBlock` into
    /// `Poll::Pending` and either internally retry or convert
    /// `Interrupted` into another error kind.
    fn poll_seek(self: Pin<&mut Self>,
                 cx: &mut Context<'_>,
                 pos: SeekFrom)
                 -> Poll<io::Result<u64>>;
}

#[cfg(feature = "futures-lite")]
impl<T> AsyncSeek for T where T: futures_lite::io::AsyncSeek
{
    fn poll_seek(self: Pin<&mut Self>,
                 cx: &mut Context<'_>,
                 pos: SeekFrom)
                 -> Poll<io::Result<u64>> {
        (self as T).poll_seek(cx, pos)
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
