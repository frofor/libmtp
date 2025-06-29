//! This module provides API for managing devices.

use crate::Error;
use crate::Result;
use crate::Storage;
use crate::convert::ptr_to_string;
use crate::err;
use crate::ffi;
use crate::storage;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_void;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ptr;

/// Discovers the devices connected via USB, but not yet opened.
pub fn discover() -> Result<RawIter> {
	unsafe {
		ffi::LIBMTP_Init();
	};

	let mut ptr = ptr::null_mut();
	let mut len = 0;

	let n = unsafe { ffi::LIBMTP_Detect_Raw_Devices(&mut ptr, &mut len) };
	match err::Kind::from_number(n) {
		Some(kind) if kind != err::Kind::NoDeviceAttached => {
			Err(Error::new(kind, "Failed to discover raw devices"))
		}
		_ => Ok(RawIter::new(ptr, len as isize)),
	}
}

/// An opened device connected via USB.
#[derive(Clone, Hash)]
pub struct Device {
	/// The underlying structure of the device.
	inner: *mut ffi::LIBMTP_mtpdevice_t,
}

impl Device {
	/// Constructs a device from the underlying structure.
	fn new(inner: *mut ffi::LIBMTP_mtpdevice_t) -> Self {
		Self { inner }
	}

	/// Retieves the friendly name of the device.
	///
	/// # Errors
	///
	/// Returns an error if the device doesn't have a support for friendly names or if the friendly name was not found.
	pub fn name(&self) -> Result<String> {
		let ptr = unsafe { ffi::LIBMTP_Get_Friendlyname(self.inner) };
		if ptr.is_null() {
			return Err(self.pop_err().unwrap_or_default());
		}

		let name = unsafe { ptr_to_string(ptr) };
		unsafe {
			libc::free(ptr as *mut c_void);
		}
		Ok(name)
	}

	/// Changes the friendly name of the device.
	///
	/// # Errors
	///
	/// Returns an error if the device doesn't have a support for friendly names or if the friendly name was not found.
	///
	/// # Panics
	///
	/// Panics if `name` contains a nul byte.
	pub fn rename(&mut self, name: &str) -> Result<()> {
		let name = CString::new(name).expect("Name should not contain a nul byte");
		let n = unsafe { ffi::LIBMTP_Set_Friendlyname(self.inner, name.as_ptr()) };
		if n != 0 {
			return Err(self.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Retrieves the maximum battery percentage of the device.
	pub fn max_battery_percent(&self) -> u8 {
		(unsafe { *self.inner }).maximum_battery_level
	}

	/// Retrieves the ID of the default music folder of the device.
	///
	/// If the default music folder was not found, the ID of the root folder will be returned.
	pub fn music_folder_id(&self) -> u32 {
		(unsafe { *self.inner }).default_music_folder
	}

	/// Retrieves the ID of the default playlists folder of the device.
	///
	/// If the default playlists folder was not found, the ID of the root folder will be returned.
	pub fn playlist_folder_id(&self) -> u32 {
		(unsafe { *self.inner }).default_playlist_folder
	}

	/// Retrieves the ID of the default pictures folder of the device.
	///
	/// If the default pictures folder was not found, the ID of the root folder will be returned.
	pub fn picture_folder_id(&self) -> u32 {
		(unsafe { *self.inner }).default_picture_folder
	}

	/// Retrieves the ID of the default videos folder of the device.
	///
	/// If the default videos folder was not found, the ID of the root folder will be returned.
	pub fn video_folder_id(&self) -> u32 {
		(unsafe { *self.inner }).default_video_folder
	}

	/// Retrieves the ID of the default organizers folder of the device.
	///
	/// If the default organizers folder was not found, the ID of the root folder will be returned.
	pub fn organizer_folder_id(&self) -> u32 {
		(unsafe { *self.inner }).default_organizer_folder
	}

	/// Retrieves the ID of the default ZENcast folder of the device.
	///
	/// If the default ZENcast folder was not found, the ID of the root folder will be returned.
	pub fn zencast_folder_id(&self) -> u32 {
		(unsafe { *self.inner }).default_zencast_folder
	}

	/// Retrieves the ID of the default albums folder of the device.
	///
	/// If the default albums folder was not found, the ID of the root folder will be returned.
	pub fn album_folder_id(&self) -> u32 {
		(unsafe { *self.inner }).default_album_folder
	}

	/// Retrieves the ID of the default texts folder of the device.
	///
	/// If the default texts folder was not found, the ID of the root folder will be returned.
	pub fn text_folder_id(&self) -> u32 {
		(unsafe { *self.inner }).default_text_folder
	}

	/// Retrieves an iterator over the storages of the device.
	pub fn storages(&self) -> storage::Iter {
		storage::Iter::new(self, (unsafe { *self.inner }).storage)
	}

	/// Pops the last error from the error stack.
	///
	/// After the execution the error stack will be cleared.
	fn pop_err(&self) -> Option<Error> {
		let stack = unsafe { ffi::LIBMTP_Get_Errorstack(self.inner) };
		let err = Error::from_stack(stack);
		unsafe {
			ffi::LIBMTP_Clear_Errorstack(self.inner);
		}
		err
	}
}

#[doc(hidden)]
impl Drop for Device {
	fn drop(&mut self) {
		unsafe {
			ffi::LIBMTP_Release_Device(self.inner);
		}
	}
}

impl Debug for Device {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Device")
			.field("max_battery_percent", &self.max_battery_percent())
			.field("music_folder_id", &self.music_folder_id())
			.field("playlist_folder_id", &self.playlist_folder_id())
			.field("picture_folder_id", &self.picture_folder_id())
			.field("video_folder_id", &self.video_folder_id())
			.field("organizer_folder_id", &self.organizer_folder_id())
			.field("zencast_folder_id", &self.zencast_folder_id())
			.field("album_folder_id", &self.album_folder_id())
			.field("text_folder_id", &self.text_folder_id())
			.finish()
	}
}

impl<'a> IntoIterator for &'a Device {
	type Item = Storage<'a>;
	type IntoIter = storage::Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.storages()
	}
}

/// A device connected via USB, but not yet opened.
pub struct RawDevice {
	/// The underlying structure of the device.
	inner: ffi::LIBMTP_raw_device_t,
}

impl RawDevice {
	/// Constructs a raw device from the underlying structure.
	fn new(inner: ffi::LIBMTP_raw_device_t) -> Self {
		Self { inner }
	}

	/// Retrieves the vendor of the device.
	///
	/// # Panics
	///
	/// Panics if the vendor name of the device is not a valid UTF-8.
	pub fn vendor(&self) -> Vendor {
		let id = self.inner.device_entry.vendor_id;
		let name = unsafe {
			CStr::from_ptr(self.inner.device_entry.vendor)
				.to_str()
				.expect("Vendor name should be a valid UTF-8")
		};
		Vendor::new(id, name)
	}

	/// Retrieves the product of the device.
	///
	/// # Panics
	///
	/// Panics if the product name of the device is not a valid UTF-8.
	pub fn product(&self) -> Product {
		let id = self.inner.device_entry.product_id;
		let name = unsafe {
			CStr::from_ptr(self.inner.device_entry.product)
				.to_str()
				.expect("Product name should be a valid UTF-8")
		};
		Product::new(id, name)
	}

	/// Attempts to open the device, with caching.
	pub fn open(&self) -> Option<Device> {
		let ptr = &self.inner as *const _;
		let dev_ptr = unsafe { ffi::LIBMTP_Open_Raw_Device(ptr as *mut _) };
		if dev_ptr.is_null() { None } else { Some(Device::new(dev_ptr)) }
	}

	/// Attempts to open the device, without caching.
	pub fn open_uncached(&self) -> Option<Device> {
		let ptr = &self.inner as *const _;
		let dev_ptr = unsafe { ffi::LIBMTP_Open_Raw_Device_Uncached(ptr as *mut _) };
		if dev_ptr.is_null() { None } else { Some(Device::new(dev_ptr)) }
	}
}

impl Debug for RawDevice {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("RawDevice")
			.field("vendor", &self.vendor())
			.field("product", &self.product())
			.finish()
	}
}

/// A vendor of the device.
#[derive(Copy, Clone, Hash, Debug)]
pub struct Vendor<'a> {
	/// The ID of the vendor.
	pub id: u16,
	/// The name of the vendor.
	pub name: &'a str,
}

impl<'a> Vendor<'a> {
	/// Constructs a new vendor from the id and the name.
	fn new(id: u16, name: &'a str) -> Self {
		Self { id, name }
	}
}

impl<'a> Display for Vendor<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

/// A product of the device.
#[derive(Copy, Clone, Hash, Debug)]
pub struct Product<'a> {
	/// The ID of the product.
	pub id: u16,
	/// The name of the product.
	pub name: &'a str,
}

impl<'a> Product<'a> {
	/// Constructs a new product from the id and the name.
	fn new(id: u16, name: &'a str) -> Self {
		Self { id, name }
	}
}

impl<'a> Display for Product<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

/// An iterator over the raw devices.
pub struct RawIter {
	ptr: *mut ffi::LIBMTP_raw_device_t,
	len: isize,
	off: isize,
}

impl RawIter {
	/// Constructs a new raw device iterator from the underlying structure.
	fn new(ptr: *mut ffi::LIBMTP_raw_device_t, len: isize) -> Self {
		Self { ptr, len, off: 0 }
	}
}

impl Iterator for RawIter {
	type Item = RawDevice;

	fn next(&mut self) -> Option<Self::Item> {
		if self.off == self.len {
			return None;
		}

		let ptr = unsafe { *self.ptr.offset(self.off) };
		let dev = RawDevice::new(ptr);
		self.off += 1;
		Some(dev)
	}
}

#[doc(hidden)]
impl Drop for RawIter {
	fn drop(&mut self) {
		unsafe {
			libc::free(self.ptr as *mut c_void);
		}
	}
}
