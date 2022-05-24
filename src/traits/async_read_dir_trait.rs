//! TODO: Fill this in

pub use std::fs::{FileType, Metadata, Permissions};
use std::io;

use async_trait::async_trait;

#[doc(no_inline)]
use super::AsyncDirEntryTrait;
use super::Stream;

/// A stream of entries in a directory.
///
/// This stream is returned by[`read_dir()`][1] and yields items of type
/// [`io::Result`]`<`[`AsyncDirEntryTrait`]`>`. Each
/// [`super::AsyncDirEntryTrait`] can then retrieve information like entry's
/// path or metadata.
///
/// [1]: super::AsyncFsTrait::read_dir
#[async_trait]
pub trait AsyncReadDirTrait<T>:
    std::fmt::Debug + Stream<Item = io::Result<T>>
    where T: AsyncDirEntryTrait
{
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
