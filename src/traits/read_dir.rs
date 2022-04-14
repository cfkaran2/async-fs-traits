//! TODO: Fill this in

pub use std::fs::{FileType, Metadata, Permissions};
#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt as _;
use std::{
    fmt,
    io::{self},
    pin::Pin,
    task::{Context, Poll}
};

use blocking::unblock;
use futures_lite::{future, ready, stream::Stream};

#[doc(no_inline)]
use crate::DirEntry;

/// The state of an asynchronous `ReadDir`.
///
/// The `ReadDir` can be either idle or busy performing an asynchronous operation.
enum State {
    Idle(Option<std::fs::ReadDir>),
    Busy(future::Boxed<(std::fs::ReadDir,
                         Option<io::Result<std::fs::DirEntry>>)>)
}

/// A stream of entries in a directory.
///
/// This stream is returned by [`read_dir()`] and yields items of type
/// [`io::Result`]`<`[`DirEntry`]`>`. Each [`DirEntry`] can then retrieve information like entry's
/// path or metadata.
pub struct ReadDir(State);

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadDir").finish()
    }
}

impl Stream for ReadDir {
    type Item = io::Result<DirEntry>;

    fn poll_next(mut self: Pin<&mut Self>,
                 cx: &mut Context<'_>)
                 -> Poll<Option<Self::Item>> {
        loop {
            match &mut self.0 {
                State::Idle(opt) => {
                    let mut inner = opt.take().unwrap();

                    // Start the operation asynchronously.
                    self.0 = State::Busy(Box::pin(unblock(move || {
                                                      let next = inner.next();
                                                      (inner, next)
                                                  })));
                }
                // Poll the asynchronous operation the file is currently blocked on.
                State::Busy(task) => {
                    let (inner, opt) = ready!(task.as_mut().poll(cx));
                    self.0 = State::Idle(Some(inner));
                    return Poll::Ready(opt.map(|res| {
                                              res.map(|inner| {
                                                     DirEntry::new(inner)
                                                 })
                                          }));
                }
            }
        }
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
