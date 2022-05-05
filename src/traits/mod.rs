//! Asynchronous traits for filesystems.
//!
//! This module defines a small set of asynchronous traits that can be
//! implemented on your code to model an asynchronous file system.  The traits
//! are intended to be sufficiently generic that you can implement wrapper
//! types in a manner analogous to [Tower's][1] [Service][2] trait, adding
//! capabilities that the original filesystem authors may not have considered
//! when developing their own code.
//!
//! In particular, the return types of the traits are never `Self`; instead,
//! the return types are generic types with trait bounds on them.  This allows
//! the return type to be anything, which may be useful in certain situations.
//!
//! That said, all of the types that are returned are concrete; `&dyn Foo` is
//! never used.  There are several reasons for this:
//!
//! - If the compiler knows of the concrete types that are being returned at
//!   compile time, then it is able to do more optimization than it otherwise
//!   would (such as inlining code that couldn't otherwise be inlined).
//! - While downcasting via the [`std::any::Any`][3] trait is always possible,
//!   it requires you to know what type to downcast it to before it will work.
//!   There is no way to have the equivalent of a generic object, and then ask
//!   it what it can do like in Python or other languages.  In practice, this
//!   means that you can only truly rely on those methods that are defined
//!   with the trait that is being implemented, which means that such traits
//!   tend to get bloated with extra methods that only a few people will ever
//!   actually want or need.
//! - Knowing the concrete types at compile time greatly improves the ability of
//!   the compiler to catch errors.  Without this capability, certain classes of
//!   errors must be deferred to runtime checks, which inevitably means that
//!   your customer will catch the error before you do.
//! - Trait objects are always behind a reference in some manner or other.  This
//!   might be the `&dyn` directly, or it might be in a `Box` or some other
//!   container.  This indirection leads to a certain amount of pointer chasing.
//!   This may be good or bad, because the use of concrete generics results in
//!   a certain amount of duplicated code, which can overflow the cache.  The
//!   code for trait objects is smaller, but can defeat the cache logic via
//!   pointer chasing.
//!
//! [1]: https://crates.io/crates/tower
//! [2]: https://docs.rs/tower/latest/tower/trait.Service.html
//! [3]: https://doc.rust-lang.org/std/any/trait.Any.html

pub mod async_dir_builder_trait;
pub mod async_dir_entry_trait;
pub mod async_file_builder_trait;
pub mod async_file_trait;
pub mod async_fs_trait;
pub mod async_read_dir_trait;
pub mod async_sym_link_trait;

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
