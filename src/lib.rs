//! A memory safe API for interacting with libmtp library.
//!
//! [![crates.io](https://img.shields.io/crates/v/libmtp)](https://crates.io/crates/libmtp)
//!
//! ## Install
//!
//! To install the latest version of `libmtp` from [crates.io](https://crates.io/crates/libmtp), run:
//!
//! ```sh
//! $ cargo add libmtp
//! ```
//!
//! ## Getting started
//!
//! To get started, create a new program that prints all objects in the root folder of your storage:
//!
//! ```no_run
//! use libmtp::search_raw_devices;
//!
//! fn main() -> libmtp::Result<()> {
//!     for device in search_raw_devices()?.filter_map(|r| r.open_uncached()) {
//!         for storage in &device {
//!             for object in &storage {
//!                 println!("{object:?}");
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! For more examples, see [examples](https://codeberg.org/frofor/libmtp/src/branch/stable/examples).

#![warn(missing_docs)]

pub(crate) mod convert;
mod device;
mod error;
pub(crate) mod ffi;
mod object;
mod storage;

pub use device::*;
pub use error::*;
pub use object::*;
pub use storage::*;
