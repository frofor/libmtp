//! A memory safe API for interacting with libmtp library.
//!
//! [![crates.io](https://img.shields.io/crates/v/libmtp)](https://crates.io/crates/libmtp)
//!
//! ## Install
//!
//! To install the latest version of the crate from [crates.io](https://crates.io/crates/libmtp),
//! run:
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
//! use libmtp::RawDevice;
//! use libmtp::search_raw_devices;
//!
//! fn main() -> libmtp::Result<()> {
//!     for device in search_raw_devices()?.filter_map(RawDevice::open_uncached) {
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
//! For more examples, see [examples](https://codeberg.org/frofor/libmtp/src/branch/main/examples).
//!
//! ## Changelog
//!
//! For a release history, see
//! [CHANGELOG.md](https://codeberg.org/frofor/libmtp/src/branch/main/doc/CHANGELOG.md).
//!
//! ## Contributing
//!
//! For a contibuting guide, see
//! [CONTRIBUTING.md](https://codeberg.org/frofor/libmtp/src/branch/main/doc/CONTRIBUTING.md).
//!
//! ## License
//!
//! This crate is distributed under the terms of MIT License.
//!
//! See [LICENSE](https://codeberg.org/frofor/libmtp/src/branch/main/LICENSE) for details.

#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

pub(crate) mod convert;
mod dev;
mod err;
pub(crate) mod ffi;
mod obj;
mod storage;

pub use dev::*;
pub use err::*;
pub use obj::*;
pub use storage::*;
