//! Asynchronous traits and utility types for filesystems.
//!
//! This crate defines a small set of asynchronous traits that can be
//! implemented on your code to model an asynchronous file system.  The traits
//! are intended to be sufficiently generic that you can implement wrapper
//! types in a manner analogous to [Tower's][1] [Service][2] trait, adding
//! capabilities that the original filesystem authors may not have considered
//! when developing their own code.
//!
//! In addition, this crate defines its own set of utility types that may be of
//! use to others.  These utility types provide a number of potentially useful
//! capabilities, such as the ability to compress and decompress files on the
//! fly, log the behavior of wrapped objects, etc.  Each of the utility types
//! is designed to be independent of the other types, while also being able to
//! be composed with the other types.  In addition, each and every utility type
//! is defined within its own module, which is itself feature gated.  This
//! ensures that end users need only pay for those features that they wish to
//! use.
//!
//! [1]: https://crates.io/crates/tower
//! [2]: https://docs.rs/tower/latest/tower/trait.Service.html

#![warn(missing_docs,
        missing_debug_implementations,
        rust_2018_idioms,
        rustdoc::missing_crate_level_docs)]

#[doc(no_inline)]
pub use std::fs::{FileType, Metadata, Permissions};

pub mod traits;
#[doc(no_inline)]
pub use std::io::{Error, ErrorKind, IoSlice, IoSliceMut, Result, SeekFrom};

#[doc(inline)]
pub use async_dir_builder_trait::AsyncDirBuilderTrait;
#[doc(inline)]
pub use async_dir_entry_trait::AsyncDirEntryTrait;
#[doc(inline)]
pub use async_file_builder_trait::AsyncFileBuilderTrait;
#[doc(inline)]
pub use async_file_trait::AsyncFileTrait;
#[doc(inline)]
pub use async_fs_trait::AsyncFsTrait;
#[doc(inline)]
pub use async_read_dir_trait::AsyncReadDirTrait;
#[doc(inline)]
pub use async_sym_link_trait::AsyncSymLinkTrait;
#[doc(no_inline)]
pub use futures_core::stream::Stream;
#[doc(no_inline)]
pub use futures_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};
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
