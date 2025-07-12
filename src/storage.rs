//! This module provides API for managing storages of devices.

use crate::Device;
use crate::Object;
use crate::ObjectIter;
use crate::ObjectRecursiveIter;
use crate::Ownership;
use crate::Result;
use crate::ffi;
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
	/// The pointer to the underlying struture of the storage.
	inner_ptr: *mut ffi::LIBMTP_devicestorage_t,
}

impl<'a> Storage<'a> {
	/// Constructs a new storage.
	///
	/// # Safety
	///
	/// `ptr` should not be null.
	pub(crate) unsafe fn new_unchecked(
		owner: &'a Device,
		ptr: *mut ffi::LIBMTP_devicestorage_t,
	) -> Self {
		Self { owner, inner: unsafe { *ptr }, inner_ptr: ptr }
	}

	/// Erases all data on the storage and formats it.
	///
	/// <div class="warning">
	/// This function will permanently erase all data from the storage!
	/// </div>
	///
	/// # Errors
	///
	/// Returns an error if the device doesn't have a support for storage formatting or if the
	/// operation has failed.
	pub fn format(&self) -> Result<()> {
		let dev = self.owner();
		let n = unsafe { ffi::LIBMTP_Format_Storage(dev.inner_ptr(), self.inner_ptr) };
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
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

	/// Retrieves the total space in bytes of the storage.
	pub fn total_space(&self) -> u64 {
		self.inner.MaxCapacity
	}

	/// Retrieves the free space in bytes of the storage.
	pub fn free_space(&self) -> u64 {
		self.inner.FreeSpaceInBytes
	}

	/// Retrieves an iterator over the objects in the root of the storage.
	pub fn iter(&self) -> ObjectIter {
		let ptr = unsafe {
			ffi::LIBMTP_Get_Files_And_Folders(
				self.owner.inner_ptr(),
				self.id(),
				ffi::LIBMTP_FILES_AND_FOLDERS_ROOT,
			)
		};
		unsafe { ObjectIter::new_unchecked(self, ptr, Ownership::Owns) }
	}

	/// Retrieves a recursive iterator over the objects of the storage.
	pub fn iter_recursive(&self) -> ObjectRecursiveIter {
		let ptr = unsafe {
			ffi::LIBMTP_Get_Files_And_Folders(
				self.owner.inner_ptr(),
				self.id(),
				ffi::LIBMTP_FILES_AND_FOLDERS_ROOT,
			)
		};
		unsafe { ObjectRecursiveIter::new_unchecked(self, ptr, Ownership::Owns) }
	}

	/// Retrieves the device to which the storage belongs.
	pub(crate) fn owner(&self) -> &Device {
		self.owner
	}
}

impl<'a> IntoIterator for &'a Storage<'a> {
	type Item = Object<'a>;
	type IntoIter = ObjectIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> Display for Storage<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name().unwrap_or("Unnamed storage"))
	}
}

impl<'a> Debug for Storage<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Storage")
			.field("id", &self.id())
			.field("name", &self.name())
			.field("total_space", &self.total_space())
			.field("free_space", &self.free_space())
			.finish()
	}
}

/// An iterator over the storages of the device.
#[derive(Clone, Copy)]
pub struct StorageIter<'a> {
	/// The device to which the storage belongs.
	dev: &'a Device,
	/// The pointer to the underlying struture of the storage.
	ptr: *mut ffi::LIBMTP_devicestorage_t,
}

impl<'a> StorageIter<'a> {
	/// Constructs a new storage iterator.
	pub(crate) fn new(dev: &'a Device) -> Self {
		Self { dev, ptr: dev.inner().storage }
	}
}

impl<'a> Iterator for StorageIter<'a> {
	type Item = Storage<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ptr.is_null() {
			return None;
		}

		let storage = unsafe { Storage::new_unchecked(self.dev, self.ptr) };
		self.ptr = unsafe { *self.ptr }.next;
		Some(storage)
	}
}
