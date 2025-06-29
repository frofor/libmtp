//! This module provides API for managing storages of devices.

use std::ffi::CStr;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::Device;
use crate::ffi;

/// A storage of the device.
#[derive(Clone, Hash)]
pub struct Storage<'a> {
	/// The device that owns the storage.
	owner: &'a Device,
	/// The underlying struture of the storage.
	inner: *mut ffi::LIBMTP_devicestorage_t,
}

impl<'a> Storage<'a> {
	/// Constructs a new storage from the underlying structure.
	pub(crate) fn new(owner: &'a Device, inner: *mut ffi::LIBMTP_devicestorage_t) -> Self {
		Self { owner, inner }
	}

	/// Retrieves the friendly name of the storage.
	///
	/// # Panics
	///
	/// Panics if the friendly name of the storage is not a valid UTF-8.
	pub fn name(&self) -> Option<&str> {
		let ptr = (unsafe { *self.inner }).StorageDescription;
		if ptr.is_null() {
			return None;
		}

		Some(unsafe { CStr::from_ptr(ptr).to_str().expect("Storage name should be a valid UTF-8") })
	}
}

impl<'a> Display for Storage<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name().unwrap_or("Unnamed storage"))
	}
}

impl<'a> Debug for Storage<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Storage").field("name", &self.name()).finish()
	}
}

/// An iterator over the storages of the device.
pub struct Iter<'a> {
	dev: &'a Device,
	ptr: *mut ffi::LIBMTP_devicestorage_t,
}

impl<'a> Iter<'a> {
	/// Constructs a new storage iterator from the device and underlying storage struture.
	pub(crate) fn new(dev: &'a Device, ptr: *mut ffi::LIBMTP_devicestorage_t) -> Self {
		Self { dev, ptr }
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = Storage<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ptr.is_null() {
			return None;
		}

		let storage = Storage::new(self.dev, self.ptr);
		self.ptr = (unsafe { *self.ptr }).next;
		Some(storage)
	}
}
