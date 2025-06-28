//! A memory safe API for interacting with libmtp library.

#![warn(missing_docs)]

pub(crate) mod convert;
pub mod dev;
pub mod err;
pub(crate) mod ffi;
pub mod storage;

pub use dev::Device;
pub use err::Error;
pub use err::Result;
pub use storage::Storage;
