//! This module provides API for managing MTP devices.

use crate::Error;
use crate::Result;
use crate::convert::ptr_to_string;
use crate::err;
use crate::ffi;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_void;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ptr;

/// Discovers the devices connected via USB, but not yet opened.
pub fn discover() -> Result<Vec<ClosedDevice>> {
	unsafe {
		ffi::LIBMTP_Init();
	};

	let mut ptr = ptr::null_mut();
	let mut len = 0;

	let res = unsafe { ffi::LIBMTP_Detect_Raw_Devices(&mut ptr, &mut len) };
	match err::Kind::from_number(res) {
		Some(err::Kind::NoDeviceAttached) => return Ok(Vec::new()),
		Some(kind) => return Err(Error::new(kind, "Failed to detect raw devices")),
		None => {}
	}

	let mut devs = Vec::with_capacity(len as usize);
	for i in 0..(len as isize) {
		let dev_ptr = unsafe { ptr.offset(i) };
		devs.push(ClosedDevice::from_inner(unsafe { *dev_ptr }));
	}

	unsafe {
		libc::free(ptr as *mut c_void);
	};

	Ok(devs)
}

/// An opened device connected via USB.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Device {
	pub(crate) inner: *mut ffi::LIBMTP_mtpdevice_t,
}

impl Device {
	/// Constructs an opened device from the pointer to LIBMTP_mtpdevice_t.
	pub(crate) fn from_inner(inner: *mut ffi::LIBMTP_mtpdevice_t) -> Self {
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
		let name = CString::new(name).expect("Name should not contain nul byte");
		let code = unsafe { ffi::LIBMTP_Set_Friendlyname(self.inner, name.as_ptr()) };
		if code != 0 {
			return Err(self.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Retrieves the maximum battery percentage of the device.
	pub fn max_battery_percent(&self) -> u8 {
		unsafe { (*self.inner).maximum_battery_level }
	}

	/// Retrieves the ID of the default music folder of the device.
	///
	/// <div class="warning">
	/// If the default music folder was not found, the ID of the root folder will be returned.
	/// </div>
	pub fn music_folder_id(&self) -> u32 {
		unsafe { (*self.inner).default_music_folder }
	}

	/// Retrieves the ID of the default playlists folder of the device.
	///
	/// <div class="warning">
	/// If the default playlists folder was not found, the ID of the root folder will be returned.
	/// </div>
	pub fn playlist_folder_id(&self) -> u32 {
		unsafe { (*self.inner).default_playlist_folder }
	}

	/// Retrieves the ID of the default pictures folder of the device.
	///
	/// <div class="warning">
	/// If the default pictures folder was not found, the ID of the root folder will be returned.
	/// </div>
	pub fn picture_folder_id(&self) -> u32 {
		unsafe { (*self.inner).default_picture_folder }
	}

	/// Retrieves the ID of the default videos folder of the device.
	///
	/// <div class="warning">
	/// If the default videos folder was not found, the ID of the root folder will be returned.
	/// </div>
	pub fn video_folder_id(&self) -> u32 {
		unsafe { (*self.inner).default_video_folder }
	}

	/// Retrieves the ID of the default organizers folder of the device.
	///
	/// <div class="warning">
	/// If the default organizers folder was not found, the ID of the root folder will be returned.
	/// </div>
	pub fn organizer_folder_id(&self) -> u32 {
		unsafe { (*self.inner).default_organizer_folder }
	}

	/// Retrieves the ID of the default ZENcast folder of the device.
	///
	/// <div class="warning">
	/// If the default ZENcast folder was not found, the ID of the root folder will be returned.
	/// </div>
	pub fn zencast_folder_id(&self) -> u32 {
		unsafe { (*self.inner).default_zencast_folder }
	}

	/// Retrieves the ID of the default albums folder of the device.
	///
	/// <div class="warning">
	/// If the default albums folder was not found, the ID of the root folder will be returned.
	/// </div>
	pub fn album_folder_id(&self) -> u32 {
		unsafe { (*self.inner).default_album_folder }
	}

	/// Retrieves the ID of the default texts folder of the device.
	///
	/// <div class="warning">
	/// If the default texts folder was not found, the ID of the root folder will be returned.
	/// </div>
	pub fn text_folder_id(&self) -> u32 {
		unsafe { (*self.inner).default_text_folder }
	}

	/// Pops the last error from the error stack.
	///
	/// After the execution the error stack will be cleared.
	pub(crate) fn pop_err(&self) -> Option<Error> {
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

/// A device connected via USB, but not yet opened.
pub struct ClosedDevice {
	pub(crate) inner: ffi::LIBMTP_raw_device_t,
}

impl ClosedDevice {
	/// Constructs a closed device from the pointer to LIBMTP_raw_device_t.
	pub(crate) fn from_inner(inner: ffi::LIBMTP_raw_device_t) -> Self {
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
	///
	/// Initial call might take longer due to caching overhead.
	/// May provide less accurate results compared to [`ClosedDevice::open`].
	pub fn open(&self) -> Option<Device> {
		let ptr = &self.inner as *const _;
		let dev_ptr = unsafe { ffi::LIBMTP_Open_Raw_Device(ptr as *mut _) };
		if dev_ptr.is_null() { None } else { Some(Device::from_inner(dev_ptr)) }
	}

	/// Attempts to open the device, without caching.
	///
	/// May provide more accurate results compared to [`ClosedDevice::open`].
	pub fn open_uncached(&self) -> Option<Device> {
		let ptr = &self.inner as *const _;
		let dev_ptr = unsafe { ffi::LIBMTP_Open_Raw_Device_Uncached(ptr as *mut _) };
		if dev_ptr.is_null() { None } else { Some(Device::from_inner(dev_ptr)) }
	}
}

impl Debug for ClosedDevice {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("ClosedDevice")
			.field("vendor", &self.vendor())
			.field("product", &self.product())
			.finish()
	}
}

/// A vendor of the device.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Vendor<'a> {
	/// The ID of the vendor.
	id: u16,
	/// The name of the vendor.
	name: &'a str,
}

impl<'a> Vendor<'a> {
	/// Constructs a new vendor from `id` and `name`.
	pub(crate) fn new(id: u16, name: &'a str) -> Self {
		Self { id, name }
	}
}

/// A product of the device.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Product<'a> {
	/// The ID of the product.
	id: u16,
	/// The name of the product.
	name: &'a str,
}

impl<'a> Product<'a> {
	/// Constructs a new product from `id` and `name`.
	pub(crate) fn new(id: u16, name: &'a str) -> Self {
		Self { id, name }
	}
}
