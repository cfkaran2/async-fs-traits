//! Asynchronous traits for filesystems.
//!
//! This crate defines a small set of asynchronous traits that can be
//! implemented on your code to model an asynchronous file system.  The traits
//! are intended to be sufficiently generic that you can implement wrapper
//! types in a manner analogous to [Tower's][1] [Service][2] trait, adding
//! capabilities that the original filesystem authors may not have considered
//! when developing their own code. In addition, this crate defines its own set
//! of wrapper types that may be of use to others when they are implementing
//! the traits defined within this crate.
//!
//! [1]: https://crates.io/crates/tower
//! [2]: https://docs.rs/tower/latest/tower/trait.Service.html

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
