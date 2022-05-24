//! [`AsyncSymLinkTrait`] is an optional trait for creating symlinks.
//!
//! Some, but not all, file system like objects have the ability to create
//! symbolic links.  In addition, some file systems only permit connections to
//! file objects, while others permit links to directories and other objects as
//! well.  It is up to the implementor to decide if it is possible to create a
//! symlink for the given location and connection.  If it isn't, then an
//! appropriate error must be returned.  It is always valid to implement this
//! trait but then always return an error.

use std::{io, path::Path};

use async_trait::async_trait;

/// [`AsyncSymLinkTrait`] is an optional trait for creating symlinks.
///
/// Some, but not all, file system like objects have the ability to create
/// symbolic links.  In addition, some file systems only permit connections to
/// file objects, while others permit links to directories and other objects as
/// well.  It is up to the implementor to decide if it is possible to create a
/// symlink for the given location and connection.  If it isn't, then an
/// appropriate error must be returned.  It is always valid to implement this
/// trait but then always return an error.
#[async_trait]
pub trait AsyncSymLinkTrait: std::fmt::Debug {
    /// Creates a symlink at `src` that points to `dst`.
    ///
    /// The symlink itself will be located at the path `src`.  The path at `dst`
    /// might not exist when the symlink is created; this trait doesn't require
    /// it to be.  File system objects may choose to prevent this by refusing to
    /// create a link to an invalid path or object, but if this choice is made,
    /// then file systems should be able to detect invalid symlinks at any time.
    /// E.g., if you create a valid symlink to some file, and later on delete
    /// the file without deleting the symlink, the symlink will no longer be
    /// valid.  Your file system will need to be able to handle this case
    /// without crashing.
    async fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P,
                                                     dst: Q)
                                                     -> io::Result<()>;
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
