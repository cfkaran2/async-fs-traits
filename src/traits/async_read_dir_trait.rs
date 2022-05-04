//! TODO: Fill this in

pub use std::fs::{FileType, Metadata, Permissions};
use std::io;

use async_trait::async_trait;

#[doc(no_inline)]
use crate::AsyncDirEntryTrait;
use crate::Stream;

/// A stream of entries in a directory.
///
/// This stream is returned by
/// [`read_dir()`][crate::async_fs_trait::AsyncFsTrait::read_dir] and yields
/// items of type [`io::Result`]`<`[`AsyncDirEntryTrait`]`>`. Each
/// [`AsyncDirEntryTrait`] can
/// then retrieve information like entry's path or metadata.
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
