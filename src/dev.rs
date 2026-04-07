//! This module provides API for managing devices.

use crate::Error;
use crate::ErrorKind;
use crate::MtpErrorKind;
use crate::Result;
use crate::Storage;
use crate::StorageIter;
use crate::convert::ptr_to_string;
use crate::ffi;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ptr;

/// Searches the devices connected via USB, but not yet opened.
///
/// # Errors
///
/// Returns an error if the operation has failed.
///
/// # Examples
///
/// ```no_run
/// use libmtp::search_raw_devices;
///
/// # fn main() -> libmtp::Result<()> {
/// for raw_device in search_raw_devices()? {
///     println!("{raw_device:?}");
/// }
/// # Ok(())
/// # }
/// ```
pub fn search_raw_devices() -> Result<RawDeviceIter> {
	unsafe { ffi::LIBMTP_Init() };

	let mut ptr = ptr::null_mut();
	let mut len = 0;

	let n = unsafe { ffi::LIBMTP_Detect_Raw_Devices(&raw mut ptr, &raw mut len) };
	match MtpErrorKind::new(n) {
		Some(kind) if kind != MtpErrorKind::NoDeviceAttached => {
			Err(Error::new(ErrorKind::Mtp(kind), "Failed to discover raw devices"))
		}
		_ => Ok(unsafe { RawDeviceIter::new_unchecked(ptr, len as isize) }),
	}
}

/// An opened device connected via USB.
#[derive(Hash)]
pub struct Device {
	/// The raw representation of the device.
	raw: RawDevice,
	/// The underlying structure of the device.
	inner: ffi::LIBMTP_mtpdevice_t,
	/// The pointer to the underlying structure of the device.
	inner_ptr: *mut ffi::LIBMTP_mtpdevice_t,
}

impl Device {
	/// Searches the device by the serial number.
	///
	/// # Errors
	///
	/// Returns an error if the operation has failed.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// if let Some(device) = Device::from_serial("GVEV4I3E0WU1")? {
	///     println!("{device:?}");
	/// } else {
	///     println!("Device not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	pub fn from_serial(serial: &str) -> Result<Option<Self>> {
		for dev in search_raw_devices()?.filter_map(RawDevice::open_uncached) {
			if dev.serial()? == serial {
				return Ok(Some(dev));
			}
		}
		Ok(None)
	}

	/// Constructs a new device.
	///
	/// # Safety
	///
	/// `ptr` should not be null.
	#[must_use]
	pub(crate) unsafe fn new_unchecked(raw: RawDevice, ptr: *mut ffi::LIBMTP_mtpdevice_t) -> Self {
		Self { raw, inner: unsafe { *ptr }, inner_ptr: ptr }
	}

	/// Retrieves the serial number of the device.
	///
	/// # Panics
	///
	/// Panics if the serial number of the device is not a valid UTF-8.
	///
	/// # Errors
	///
	/// Returns an error if the operation has failed.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	/// assert_eq!(device.serial()?, "GVEV4I3E0WU1");
	/// # Ok(())
	/// # }
	/// ```
	pub fn serial(&self) -> Result<String> {
		let ptr = unsafe { ffi::LIBMTP_Get_Serialnumber(self.inner_ptr) };
		if ptr.is_null() {
			return Err(self.pop_err().unwrap_or_default());
		}

		Ok(unsafe { CStr::from_ptr(ptr) }
			.to_str()
			.expect("Serial number should be a valid UTF-8")
			.to_owned())
	}

	/// Retieves the friendly name of the device.
	///
	/// # Errors
	///
	/// Returns an error if the device doesn't have a support for friendly names, if the
	/// friendly name was not found or if the operation has failed.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	/// println!("{}", device.name()?);
	/// # Ok(())
	/// # }
	/// ```
	pub fn name(&self) -> Result<String> {
		let ptr = unsafe { ffi::LIBMTP_Get_Friendlyname(self.inner_ptr) };
		if ptr.is_null() {
			return Err(self.pop_err().unwrap_or_default());
		}

		let name = unsafe { ptr_to_string(ptr) };
		unsafe { libc::free(ptr.cast()) };
		Ok(name)
	}

	/// Retrieves the vendor of the device.
	///
	/// # Panics
	///
	/// Panics if the vendor name of the device is not a valid UTF-8.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	/// println!("{:?}", device.vendor());
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn vendor(&self) -> Vendor {
		self.raw.vendor()
	}

	/// Retrieves the product of the device.
	///
	/// # Panics
	///
	/// Panics if the product name of the device is not a valid UTF-8.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	/// println!("{:?}", device.product());
	/// # Ok(())
	/// # }
	#[must_use]
	pub fn product(&self) -> Product {
		self.raw.product()
	}

	/// Retrieves the battery of the device.
	///
	/// # Errors
	///
	/// Returns an error if the operation has failed.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	/// println!("{:?}", device.battery()?);
	/// # Ok(())
	/// # }
	/// ```
	pub fn battery(&self) -> Result<Battery> {
		let mut now = 0;
		let mut max = 0;

		let n = unsafe { ffi::LIBMTP_Get_Batterylevel(self.inner_ptr, &raw mut max, &raw mut now) };
		if n != 0 {
			return Err(self.pop_err().unwrap_or_default());
		}
		Ok(Battery::new(now, max))
	}

	/// Retrieves the ID of the default music folder of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// if let Some(id) = device.music_folder_id() {
	///     println!("Music folder ID: {id}");
	/// } else {
	///     println!("Music folder not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn music_folder_id(&self) -> Option<u32> {
		let id = self.inner.default_music_folder;
		if id == ffi::LIBMTP_FILES_AND_FOLDERS_ROOT {
			return None;
		}
		Some(id)
	}

	/// Retrieves the ID of the default playlists folder of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// if let Some(id) = device.playlist_folder_id() {
	///     println!("Playlists folder ID: {id}");
	/// } else {
	///     println!("Playlists folder not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn playlist_folder_id(&self) -> Option<u32> {
		let id = self.inner.default_playlist_folder;
		if id == ffi::LIBMTP_FILES_AND_FOLDERS_ROOT {
			return None;
		}
		Some(id)
	}

	/// Retrieves the ID of the default pictures folder of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// if let Some(id) = device.picture_folder_id() {
	///     println!("Pictures folder ID: {id}");
	/// } else {
	///     println!("Pictures folder not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn picture_folder_id(&self) -> Option<u32> {
		let id = self.inner.default_picture_folder;
		if id == ffi::LIBMTP_FILES_AND_FOLDERS_ROOT {
			return None;
		}
		Some(id)
	}

	/// Retrieves the ID of the default videos folder of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// if let Some(id) = device.video_folder_id() {
	///     println!("Videos folder ID: {id}");
	/// } else {
	///     println!("Videos folder not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn video_folder_id(&self) -> Option<u32> {
		let id = self.inner.default_video_folder;
		if id == ffi::LIBMTP_FILES_AND_FOLDERS_ROOT {
			return None;
		}
		Some(id)
	}

	/// Retrieves the ID of the default organizers folder of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// if let Some(id) = device.organizer_folder_id() {
	///     println!("Organizers folder ID: {id}");
	/// } else {
	///     println!("Organizers folder not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn organizer_folder_id(&self) -> Option<u32> {
		let id = self.inner.default_organizer_folder;
		if id == ffi::LIBMTP_FILES_AND_FOLDERS_ROOT {
			return None;
		}
		Some(id)
	}

	/// Retrieves the ID of the default ZENcast folder of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// if let Some(id) = device.zencast_folder_id() {
	///     println!("ZENcast folder ID: {id}");
	/// } else {
	///     println!("ZENcast folder not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	#[allow(clippy::doc_markdown)]
	pub fn zencast_folder_id(&self) -> Option<u32> {
		let id = self.inner.default_zencast_folder;
		if id == ffi::LIBMTP_FILES_AND_FOLDERS_ROOT {
			return None;
		}
		Some(id)
	}

	/// Retrieves the ID of the default albums folder of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// if let Some(id) = device.album_folder_id() {
	///     println!("Albums folder ID: {id}");
	/// } else {
	///     println!("Albums folder not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn album_folder_id(&self) -> Option<u32> {
		let id = self.inner.default_album_folder;
		if id == ffi::LIBMTP_FILES_AND_FOLDERS_ROOT {
			return None;
		}
		Some(id)
	}

	/// Retrieves the ID of the default texts folder of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// if let Some(id) = device.text_folder_id() {
	///     println!("Texts folder ID: {id}");
	/// } else {
	///     println!("Texts folder not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn text_folder_id(&self) -> Option<u32> {
		let id = self.inner.default_text_folder;
		if id == ffi::LIBMTP_FILES_AND_FOLDERS_ROOT {
			return None;
		}
		Some(id)
	}

	/// Retrieves the underlying structure of the device.
	#[must_use]
	pub(crate) fn inner(&self) -> ffi::LIBMTP_mtpdevice_t {
		self.inner
	}

	/// Retrieves the pointer to the underlying structure of the device.
	#[must_use]
	pub(crate) fn inner_ptr(&self) -> *mut ffi::LIBMTP_mtpdevice_t {
		self.inner_ptr
	}

	/// Refreshes storages information for the device.
	///
	/// # Errors
	///
	/// Returns an error if the operation has failed.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	/// let storage = device.find_storage(65537).expect("Storage should exist");
	///
	/// println!("Before: {}", storage.free_space());
	/// device.refresh()?;
	/// println!("After: {}", storage.free_space());
	/// # Ok(())
	/// # }
	/// ```
	pub fn refresh(&self) -> Result<()> {
		let n = unsafe {
			ffi::LIBMTP_Get_Storage(self.inner_ptr, ffi::LIBMTP_STORAGE_SORTBY_NOTSORTED)
		};
		if n != 0 {
			return Err(self.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Changes the friendly name of the device.
	///
	/// # Errors
	///
	/// Returns an error if the device doesn't have a support for friendly names or if the
	/// operation has failed.
	///
	/// # Panics
	///
	/// Panics if the friendly name of the storage contains a nul byte.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	/// device.rename("Bob's Phone")?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn rename(&self, name: &str) -> Result<()> {
		let name = CString::new(name).expect("Name should not contain a nul byte");
		let n = unsafe { ffi::LIBMTP_Set_Friendlyname(self.inner_ptr, name.as_ptr()) };
		if n != 0 {
			return Err(self.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Retrieves an iterator over the storages of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// for storage in &device {
	///     println!("{storage:?}");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn iter(&self) -> StorageIter {
		StorageIter::new(self)
	}

	/// Searches the storage of the device by the ID.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	///
	/// if let Some(storage) = device.find_storage(65537) {
	///     println!("{storage:?}");
	/// } else {
	///     println!("Storage not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn find_storage(&self, id: u32) -> Option<Storage> {
		self.iter().find(|s| s.id() == id)
	}

	/// Pops the last error from the error stack.
	#[must_use]
	pub(crate) fn pop_err(&self) -> Option<Error> {
		let stack = unsafe { ffi::LIBMTP_Get_Errorstack(self.inner_ptr) };
		let err = Error::from_ffi(stack);
		unsafe { ffi::LIBMTP_Clear_Errorstack(self.inner_ptr) };
		err
	}
}

#[doc(hidden)]
impl Drop for Device {
	fn drop(&mut self) {
		unsafe { ffi::LIBMTP_Release_Device(self.inner_ptr) };
	}
}

impl Debug for Device {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Device")
			.field("serial", &self.serial())
			.field("name", &self.name())
			.field("vendor", &self.vendor())
			.field("product", &self.product())
			.field("battery", &self.battery())
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

/// The battery of the device.
#[derive(Clone, Copy, Hash, Debug)]
pub struct Battery {
	/// The current percentage of the battery.
	now: u8,
	/// The maximum percentage of the battery.
	max: u8,
}

impl Battery {
	/// Constructs a new battery.
	#[must_use]
	pub(crate) fn new(now: u8, max: u8) -> Self {
		Self { now, max }
	}

	/// Retrieves the current percentage of the battery.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	/// let battery = device.battery()?;
	/// println!("{}", battery.now());
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn now(&self) -> u8 {
		self.now
	}

	/// Retrieves the maximum percentage of the battery.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::Device;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	/// let battery = device.battery()?;
	/// println!("{}", battery.max());
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn max(&self) -> u8 {
		self.max
	}
}

impl<'a> IntoIterator for &'a Device {
	type Item = Storage<'a>;
	type IntoIter = StorageIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

/// A device connected via USB, but not yet opened.
#[derive(Clone, Hash)]
pub struct RawDevice {
	/// The underlying structure of the device.
	inner: ffi::LIBMTP_raw_device_t,
	/// The pointer to the underlying structure of the device.
	inner_ptr: *mut ffi::LIBMTP_raw_device_t,
}

impl RawDevice {
	/// Searches the device by the order number.
	///
	/// # Errors
	///
	/// Returns an error if the operation has failed.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// if let Some(raw_device) = RawDevice::from_order(1)? {
	///     println!("{raw_device:?}");
	/// } else {
	///     println!("Device not found");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	pub fn from_order(order: u8) -> Result<Option<Self>> {
		for raw_dev in search_raw_devices()? {
			if raw_dev.order() == order {
				return Ok(Some(raw_dev));
			}
		}
		Ok(None)
	}

	/// Constructs a new raw device.
	///
	/// # Safety
	///
	/// `ptr` should not be null.
	#[must_use]
	pub(crate) unsafe fn new_unchecked(ptr: *mut ffi::LIBMTP_raw_device_t) -> Self {
		Self { inner: unsafe { *ptr }, inner_ptr: ptr }
	}

	/// Retrieves the order number of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// assert_eq!(raw_device.order(), 1);
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn order(&self) -> u8 {
		self.inner.devnum
	}

	/// Retrieves the bus number of the device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// println!("{:?}", raw_device.bus());
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn bus(&self) -> u32 {
		self.inner.bus_location
	}

	/// Retrieves the vendor of the device.
	///
	/// # Panics
	///
	/// Panics if the vendor name of the device is not a valid UTF-8.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// println!("{:?}", raw_device.vendor());
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn vendor(&self) -> Vendor {
		Vendor::new(self.inner.device_entry)
	}

	/// Retrieves the product of the device.
	///
	/// # Panics
	///
	/// Panics if the product name of the device is not a valid UTF-8.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// println!("{:?}", raw_device.product());
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn product(&self) -> Product {
		Product::new(self.inner.device_entry)
	}

	/// Attempts to open the device, with caching.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// if let Some(device) = raw_device.open() {
	///     println!("{device:?}");
	/// } else {
	///     println!("Failed to open device");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn open(self) -> Option<Device> {
		let ptr = unsafe { ffi::LIBMTP_Open_Raw_Device(self.inner_ptr) };
		if ptr.is_null() { None } else { Some(unsafe { Device::new_unchecked(self, ptr) }) }
	}

	/// Attempts to open the device, without caching.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// if let Some(device) = raw_device.open_uncached() {
	///     println!("{device:?}");
	/// } else {
	///     println!("Failed to open device");
	/// }
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn open_uncached(self) -> Option<Device> {
		let ptr = unsafe { ffi::LIBMTP_Open_Raw_Device_Uncached(self.inner_ptr) };
		if ptr.is_null() { None } else { Some(unsafe { Device::new_unchecked(self, ptr) }) }
	}
}

#[doc(hidden)]
impl Drop for RawDevice {
	fn drop(&mut self) {
		unsafe { libc::free(self.inner_ptr.cast()) };
	}
}

impl Debug for RawDevice {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("RawDevice")
			.field("order", &self.order())
			.field("vendor", &self.vendor())
			.field("product", &self.product())
			.finish()
	}
}

/// A vendor of the device.
#[derive(Clone, Copy, Hash, Debug)]
pub struct Vendor<'a> {
	/// The ID of the vendor.
	id: u16,
	/// The name of the vendor.
	name: &'a str,
}

impl Vendor<'_> {
	/// Constructs a new vendor.
	///
	/// # Panics
	///
	/// Panics if the name of the vendor is not a valid UTF-8.
	#[must_use]
	pub(crate) fn new(inner: ffi::LIBMTP_device_entry_t) -> Self {
		let id = inner.vendor_id;
		let name = unsafe {
			CStr::from_ptr(inner.vendor).to_str().expect("Vendor name should be a valid UTF-8")
		};
		Self { id, name }
	}

	/// Retrieves the ID of the vendor.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// println!("{}", raw_device.vendor().id());
	/// # Ok(())
	/// # }
	/// ```
	#[must_use]
	pub fn id(&self) -> u16 {
		self.id
	}

	/// Retrieves the name of the vendor.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// println!("{}", raw_device.vendor().name());
	/// # Ok(())
	/// # }
	#[must_use]
	pub fn name(&self) -> &str {
		self.name
	}
}

impl Display for Vendor<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

/// A product of the device.
#[derive(Clone, Copy, Hash, Debug)]
pub struct Product<'a> {
	/// The ID of the product.
	id: u16,
	/// The name of the product.
	name: &'a str,
}

impl Product<'_> {
	/// Constructs a new product.
	///
	/// # Panics
	///
	/// Panics if the name of the product is not a valid UTF-8.
	#[must_use]
	pub(crate) fn new(inner: ffi::LIBMTP_device_entry_t) -> Self {
		let id = inner.product_id;
		let name = unsafe {
			CStr::from_ptr(inner.product).to_str().expect("Product name should be a valid UTF-8")
		};
		Self { id, name }
	}

	/// Retrieves the ID of the product.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// println!("{}", raw_device.product().id());
	/// # Ok(())
	/// # }
	#[must_use]
	pub fn id(&self) -> u16 {
		self.id
	}

	/// Retrieves the name of the product.
	///
	/// # Examples
	///
	/// ```no_run
	/// use libmtp::RawDevice;
	///
	/// # fn main() -> libmtp::Result<()> {
	/// let raw_device = RawDevice::from_order(1)?.expect("Device should exist");
	/// println!("{}", raw_device.product().name());
	/// # Ok(())
	/// # }
	#[must_use]
	pub fn name(&self) -> &str {
		self.name
	}
}

impl Display for Product<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

/// An iterator over the raw devices.
#[derive(Clone)]
pub struct RawDeviceIter {
	/// The pointer to the underlying structure of the raw device.
	ptr: *mut ffi::LIBMTP_raw_device_t,
	/// Total number of the raw devices.
	len: isize,
	/// Offset to the current raw device.
	off: isize,
}

impl RawDeviceIter {
	/// Constructs a new raw device iterator.
	///
	/// # Safety
	///
	/// `ptr` should not be null.
	#[must_use]
	pub(crate) unsafe fn new_unchecked(ptr: *mut ffi::LIBMTP_raw_device_t, len: isize) -> Self {
		Self { ptr, len, off: 0 }
	}
}

impl Iterator for RawDeviceIter {
	type Item = RawDevice;

	fn next(&mut self) -> Option<Self::Item> {
		if self.off == self.len {
			return None;
		}

		let ptr = unsafe { self.ptr.offset(self.off) };
		let dev = unsafe { RawDevice::new_unchecked(ptr) };
		self.off += 1;
		Some(dev)
	}
}
