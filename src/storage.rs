//! This module provides API for managing storages of devices.

use crate::Device;
use crate::Object;
use crate::ffi;
use crate::obj;
use crate::obj::Ownership;
use std::ffi::CStr;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

/// A storage of the device.
#[derive(Clone, Copy, Hash)]
pub struct Storage<'a> {
	/// The device to which the storage belongs.
	owner: &'a Device,
	/// The underlying struture of the storage.
	inner: ffi::LIBMTP_devicestorage_t,
}

impl<'a> Storage<'a> {
	/// Constructs a new storage.
	pub(crate) fn new(owner: &'a Device, inner: ffi::LIBMTP_devicestorage_t) -> Self {
		Self { owner, inner }
	}

	/// Retrieves the ID of the storage.
	pub fn id(&self) -> u32 {
		self.inner.id
	}

	/// Retrieves the friendly name of the storage.
	///
	/// # Panics
	///
	/// Panics if the friendly name of the storage is not a valid UTF-8.
	pub fn name(&self) -> Option<&str> {
		let ptr = self.inner.StorageDescription;
		if ptr.is_null() {
			return None;
		}

		Some(unsafe { CStr::from_ptr(ptr).to_str().expect("Storage name should be a valid UTF-8") })
	}

	/// Retrieves an iterator over the objects of the storage.
	pub fn iter(&self) -> obj::Iter {
		let ptr = unsafe {
			ffi::LIBMTP_Get_Files_And_Folders(
				self.owner.inner_ptr(),
				self.id(),
				ffi::LIBMTP_FILES_AND_FOLDERS_ROOT,
			)
		};
		obj::Iter::new(self, ptr, Ownership::Owns)
	}

	/// Retrieves the device to which the storage belongs.
	pub(crate) fn owner(&self) -> &Device {
		self.owner
	}

	/// Retrieves the underlying structure of the storage.
	pub(crate) fn inner(&self) -> ffi::LIBMTP_devicestorage_struct {
		self.inner
	}
}

impl<'a> Display for Storage<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name().unwrap_or("Unnamed storage"))
	}
}

impl<'a> Debug for Storage<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Storage").field("id", &self.id()).field("name", &self.name()).finish()
	}
}

impl<'a> IntoIterator for &'a Storage<'a> {
	type Item = Object<'a>;
	type IntoIter = obj::Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

/// An iterator over the storages of the device.
#[derive(Clone, Copy)]
pub struct Iter<'a> {
	/// The device to which the storage belongs.
	dev: &'a Device,
	/// The pointer to the underlying struture of the storage.
	ptr: *mut ffi::LIBMTP_devicestorage_t,
}

impl<'a> Iter<'a> {
	/// Constructs a new storage iterator.
	pub(crate) fn new(dev: &'a Device) -> Self {
		Self { dev, ptr: dev.inner().storage }
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = Storage<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ptr.is_null() {
			return None;
		}

		let inner = unsafe { *self.ptr };
		let storage = Storage::new(self.dev, inner);
		self.ptr = inner.next;
		Some(storage)
	}
}
