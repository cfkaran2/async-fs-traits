//! TODO: Fill this in

use std::{
    pin::Pin,
    task::{Context, Poll}
};

/// A stream of values produced asynchronously.
///
/// If `Future<Output = T>` is an asynchronous version of `T`, then `Stream<Item
/// = T>` is an asynchronous version of `Iterator<Item = T>`. A stream
/// represents a sequence of value-producing events that occur asynchronously
/// to the caller.
///
/// The trait is modeled after `Future`, but allows `poll_next` to be called
/// even after a value has been produced, yielding `None` once the stream has
/// been fully exhausted.
#[must_use = "streams do nothing unless polled"]
pub trait Stream {
    /// Values yielded by the stream.
    type Item;

    /// Attempt to pull out the next value of this stream, registering the
    /// current task for wakeup if the value is not yet available, and
    /// returning `None` if the stream is exhausted.
    ///
    /// # Return value
    ///
    /// There are several possible return values, each indicating a distinct
    /// stream state:
    ///
    /// - `Poll::Pending` means that this stream's next value is not ready yet.
    ///   Implementations will ensure that the current task will be notified
    ///   when the next value may be ready.
    ///
    /// - `Poll::Ready(Some(val))` means that the stream has successfully
    ///   produced a value, `val`, and may produce further values on subsequent
    ///   `poll_next` calls.
    ///
    /// - `Poll::Ready(None)` means that the stream has terminated, and
    ///   `poll_next` should not be invoked again.
    ///
    /// # Panics
    ///
    /// Once a stream has finished (returned `Ready(None)` from `poll_next`),
    /// calling its `poll_next` method again may panic, block forever, or cause
    /// other kinds of problems; the `Stream` trait places no requirements on
    /// the effects of such a call. However, as the `poll_next` method is not
    /// marked `unsafe`, Rust's usual rules apply: calls must never cause
    /// undefined behavior(memory corruption, incorrect use of `unsafe`
    /// functions, or the like), regardless of the stream's state.
    ///
    /// If this is difficult to guard against then the [`fuse`] adapter can be
    /// used to ensure that `poll_next` always returns `Ready(None)` in
    /// subsequent calls.
    ///
    /// [`fuse`]: https://docs.rs/futures/0.3/futures/stream/trait.StreamExt.html#method.fuse
    fn poll_next(self: Pin<&mut Self>,
                 cx: &mut Context<'_>)
                 -> Poll<Option<Self::Item>>;

    /// Returns the bounds on the remaining length of the stream.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element
    /// is the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an [`Option`]`<`
    /// [`usize`]`>`. A [`None`] here means that either there is no known upper
    /// bound, or the upper bound is larger than [`usize`].
    ///
    /// # Implementation notes
    ///
    /// It is not enforced that a stream implementation yields the declared
    /// number of elements. A buggy stream may yield less than the lower bound
    /// or more than the upper bound of elements.
    ///
    /// `size_hint()` is primarily intended to be used for optimizations such as
    /// reserving space for the elements of the stream, but must not be trusted
    /// to e.g., omit bounds checks in unsafe code. An incorrect implementation
    /// of `size_hint()` should not lead to memory safety violations.
    ///
    /// That said, the implementation should provide a correct estimation,
    /// because otherwise it would be a violation of the trait's protocol.
    ///
    /// The default implementation returns `(0, `[`None`]`)` which is correct
    /// for any stream.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

#[cfg(futures_lite)]
impl Stream for T where T: futures_lite::stream::Stream
{
    type Item = T::Item;

    fn poll_next(self: Pin<&mut Self>,
                 cx: &mut Context<'_>)
                 -> Poll<Option<Self::Item>> {
        (self as T).poll_next(cx)
    }
}

#[cfg(futures_lite)]
impl futures_lite::stream::StreamExt for T where T: Stream
{
    // add code here
}
