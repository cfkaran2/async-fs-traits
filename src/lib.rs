//! Async filesystem primitives.
//!
//! This crate is an async version of [`std::fs`].
//!
//! # Implementation
//!
//! This crate uses [`blocking`] to offload blocking I/O onto a thread pool.
//!
//! [`blocking`]: https://docs.rs/blocking
//!
//! # Examples
//!
//! Create a new file and write some bytes to it:
//!
//! ```no_run
//! use async_fs::File;
//! use futures_lite::io::AsyncWriteExt;
//!
//! # futures_lite::future::block_on(async {
//! let mut file = File::create("a.txt").await?;
//! file.write_all(b"Hello, world!").await?;
//! file.flush().await?;
//! # std::io::Result::Ok(()) });
//! ```

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};

pub mod traits;
pub use traits::*;

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
