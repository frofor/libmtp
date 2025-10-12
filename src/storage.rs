//! This module provides API for managing storages of devices.

use crate::Device;
use crate::Object;
use crate::ObjectIter;
use crate::ObjectRecursiveIter;
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

	/// Retrieves the ID of the storage.
	#[must_use]
	pub fn id(&self) -> u32 {
		self.inner.id
	}

	/// Retrieves the friendly name of the storage.
	///
	/// # Panics
	///
	/// Panics if the friendly name of the storage is not a valid UTF-8.
	#[must_use]
	pub fn name(&self) -> Option<&str> {
		let ptr = self.inner.StorageDescription;
		if ptr.is_null() {
			return None;
		}
		Some(unsafe { CStr::from_ptr(ptr).to_str().expect("Storage name should be a valid UTF-8") })
	}

	/// Retrieves the kind of the storage.
	#[must_use]
	pub fn kind(&self) -> Option<StorageKind> {
		StorageKind::new(self.inner.StorageType)
	}

	/// Retrieves the filesystem of the storage.
	#[must_use]
	pub fn fs(&self) -> Option<Filesystem> {
		Filesystem::new(self.inner.FilesystemType)
	}

	/// Retrieves the access capability over the storage.
	#[must_use]
	pub fn access(&self) -> Option<StorageAccess> {
		StorageAccess::new(self.inner.AccessCapability)
	}

	/// Retrieves the total space in bytes of the storage.
	#[must_use]
	pub fn total_space(&self) -> u64 {
		self.inner.MaxCapacity
	}

	/// Retrieves the free space in bytes of the storage.
	#[must_use]
	pub fn free_space(&self) -> u64 {
		self.inner.FreeSpaceInBytes
	}

	/// Retrieves the device to which the storage belongs.
	pub(crate) fn owner(&self) -> &Device {
		self.owner
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

	/// Retrieves an iterator over the objects in the root of the storage.
	#[must_use]
	pub fn iter(&self) -> ObjectIter {
		let ptr = unsafe {
			ffi::LIBMTP_Get_Files_And_Folders(
				self.owner.inner_ptr(),
				self.id(),
				ffi::LIBMTP_FILES_AND_FOLDERS_ROOT,
			)
		};
		unsafe { ObjectIter::new_unchecked(self, None, ptr) }
	}

	/// Retrieves a recursive iterator over the objects of the storage.
	#[must_use]
	pub fn iter_recursive(&self) -> ObjectRecursiveIter {
		let ptr = unsafe {
			ffi::LIBMTP_Get_Files_And_Folders(
				self.owner.inner_ptr(),
				self.id(),
				ffi::LIBMTP_FILES_AND_FOLDERS_ROOT,
			)
		};
		unsafe { ObjectRecursiveIter::new_unchecked(self, None, ptr) }
	}
}

impl<'a> IntoIterator for &'a Storage<'a> {
	type Item = Object<'a>;
	type IntoIter = ObjectIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl Display for Storage<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name().unwrap_or("Unnamed storage"))
	}
}

impl Debug for Storage<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Storage")
			.field("id", &self.id())
			.field("name", &self.name())
			.field("kind", &self.kind())
			.field("fs", &self.fs())
			.field("access", &self.access())
			.field("total_space", &self.total_space())
			.field("free_space", &self.free_space())
			.finish()
	}
}

/// A kind of the storage.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum StorageKind {
	/// Read-only memory storage (ROM) that cannot be detached from device.
	FixedRom,
	/// Read-only memory storage (ROM) that can be detached from device.
	RemovableRom,
	/// Random-access memory (RAM) that cannot be detached from device.
	FixedRam,
	/// Random-access memory (RAM) that can be detached from device.
	RemovableRam,
	#[default]
	/// Other storage kind.
	Other,
}

impl StorageKind {
	/// Constructs a new storage kind.
	pub(crate) fn new(n: u16) -> Option<Self> {
		match n {
			ffi::PTP_ST_Undefined => Some(Self::Other),
			ffi::PTP_ST_FixedROM => Some(Self::FixedRom),
			ffi::PTP_ST_RemovableROM => Some(Self::RemovableRom),
			ffi::PTP_ST_FixedRAM => Some(Self::FixedRam),
			ffi::PTP_ST_RemovableRAM => Some(Self::RemovableRam),
			_ => None,
		}
	}
}

/// A filesystem of the storage.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum Filesystem {
	/// Filesystem with no hierarchical structure.
	Flat,
	/// Filesystem that organizes files in a tree structure.
	Tree,
	/// Design rule for Camera File system (DCF).
	Dcf,
	/// Other filesystem.
	#[default]
	Other,
}

impl Filesystem {
	/// Constructs a new filesystem.
	pub(crate) fn new(n: u16) -> Option<Self> {
		match n {
			ffi::PTP_FST_Undefined => Some(Self::Other),
			ffi::PTP_FST_GenericFlat => Some(Self::Flat),
			ffi::PTP_FST_GenericHierarchical => Some(Self::Tree),
			ffi::PTP_FST_DCF => Some(Self::Dcf),
			_ => None,
		}
	}
}

/// An access capability over the storage.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum StorageAccess {
	/// Allows both reading and writing operations on objects.
	ReadWrite,
	/// Allows only reading and deleting operations on objects.
	ReadDelete,
	/// Allows only reading objects.
	Read,
}

impl StorageAccess {
	/// Constructs a new storage access.
	pub(crate) fn new(n: u16) -> Option<Self> {
		match n {
			ffi::PTP_AC_ReadWrite => Some(Self::ReadWrite),
			ffi::PTP_AC_ReadOnly => Some(Self::Read),
			ffi::PTP_AC_ReadOnly_with_Object_Deletion => Some(Self::ReadDelete),
			_ => None,
		}
	}
}

/// An iterator over the storages of the device.
#[derive(Clone)]
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
