//! This module provides API for managing files and folders of storages.

use crate::Result;
use crate::Storage;
use crate::ffi;
use crate::obj;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_char;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

/// A file or a folder on the storage.
#[derive(Clone, Hash, Debug)]
pub enum Object<'a> {
	/// The folder on the storage.
	Folder(Folder<'a>),
	/// The file on the storage.
	File(File<'a>),
}

impl<'a> Object<'a> {
	/// Constructs a new object.
	pub(crate) fn from_ffi(
		storage: &'a Storage,
		ptr: *mut ffi::LIBMTP_file_t,
		ownership: Ownership,
	) -> Self {
		let inner = unsafe { *ptr };
		if FileKind::from_ffi(inner.filetype).is_none() {
			return Self::Folder(Folder::from_ffi(storage, ptr, ownership));
		}
		Self::File(File::from_ffi(storage, ptr))
	}

	/// Changes the name of the object.
	///
	/// # Errors
	///
	/// An error is returned if a sibling object with the same name already exists.
	///
	/// # Panics
	///
	/// Panics if the name of the object contains a nul byte.
	pub fn rename(&self, name: &str) -> Result<()> {
		match self {
			Self::Folder(f) => f.rename(name),
			Self::File(f) => f.rename(name),
		}
	}

	/// Retrieves the ID of the object.
	pub fn id(&self) -> u32 {
		match self {
			Self::Folder(f) => f.id(),
			Self::File(f) => f.id(),
		}
	}

	/// Retrieves the name of the object.
	pub fn name(&self) -> &str {
		match self {
			Self::Folder(f) => f.name(),
			Self::File(f) => f.name(),
		}
	}
}

/// A folder on the storage.
#[derive(Clone, Hash)]
pub struct Folder<'a> {
	/// The storage to which the folder belongs.
	owner: &'a Storage<'a>,
	/// The underlying structure of the folder.
	inner: ffi::LIBMTP_file_t,
	/// The pointer to the underlying structure of the folder.
	inner_ptr: *mut ffi::LIBMTP_file_t,
	/// The responsibility of the folder for the pointer cleanup.
	ownership: Ownership,
}

impl<'a> Folder<'a> {
	/// Constructs a new folder.
	pub(crate) fn from_ffi(
		owner: &'a Storage,
		ptr: *mut ffi::LIBMTP_file_t,
		ownership: Ownership,
	) -> Self {
		Self { owner, inner: unsafe { *ptr }, inner_ptr: ptr, ownership }
	}

	/// Changes the name of the foler.
	///
	/// # Errors
	///
	/// An error is returned if a sibling object with the same name already exists.
	///
	/// # Panics
	///
	/// Panics if the name of the folder contains a nul byte.
	pub fn rename<'b>(&self, name: &'b str) -> Result<()> {
		let name = CString::new(name).expect("Folder name should not contain a nul byte");
		let name_ptr = name.as_ptr() as *mut c_char;
		let dev = self.owner().owner();

		let res = unsafe { ffi::LIBMTP_Set_File_Name(dev.inner_ptr(), self.inner_ptr, name_ptr) };
		if res != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Creates a new child folder inside the folder.
	/// Returns the ID of the created folder.
	///
	/// # Errors
	///
	/// An error is returned if a child object with the same name already exists.
	///
	/// # Panics
	///
	/// Panics if the name of the folder contains a nul byte.
	pub fn create_folder(&self, name: &str) -> Result<u32> {
		let name = CString::new(name).expect("Folder name should not contain a nul byte");
		let name_ptr = name.as_ptr() as *mut c_char;
		let storage = self.owner();
		let dev = storage.owner();
		let dev_ptr = dev.inner_ptr();

		let id = unsafe { ffi::LIBMTP_Create_Folder(dev_ptr, name_ptr, self.id(), storage.id()) };
		if id == 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(id)
	}

	/// Retrieves an iterator over the objects of the folder.
	pub fn iter(&self) -> obj::Iter {
		obj::Iter::new(self.owner, self.inner_ptr, Ownership::Borrows)
	}

	/// Retrieves the ID of the folder.
	pub fn id(&self) -> u32 {
		self.inner.item_id
	}

	/// Retrieves the name of the folder.
	///
	/// # Panics
	///
	/// Panics if the name of the folder is not a valid UTF-8.
	pub fn name(&self) -> &str {
		let name = self.inner.filename;
		unsafe { CStr::from_ptr(name).to_str().expect("Folder name should be a valid UTF-8") }
	}

	/// Retrieves the storage to which the folder belongs.
	pub(crate) fn owner(&self) -> &Storage {
		self.owner
	}
}

#[doc(hidden)]
impl<'a> Drop for Folder<'a> {
	fn drop(&mut self) {
		if self.ownership == Ownership::Borrows {
			return;
		}
		unsafe {
			ffi::LIBMTP_destroy_file_t(self.inner_ptr);
		}
	}
}

impl<'a> Debug for Folder<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Folder").field("id", &self.id()).field("name", &self.name()).finish()
	}
}

impl<'a> IntoIterator for &'a Folder<'a> {
	type Item = Object<'a>;
	type IntoIter = obj::Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

/// A responsibility for the data cleanup.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Ownership {
	/// Instance owns the data and is responsible for its cleanup.
	Owns,
	/// Instance borrows the data and is not responsible for its cleanup.
	Borrows,
}

/// A file on the storage.
#[derive(Clone, Hash)]
pub struct File<'a> {
	/// The storage to which the file belongs.
	owner: &'a Storage<'a>,
	/// The underlying structure of the file.
	inner: ffi::LIBMTP_file_t,
	/// The pointer to the underlying structure of the file.
	inner_ptr: *mut ffi::LIBMTP_file_t,
}

impl<'a> File<'a> {
	/// Constructs a new file.
	pub(crate) fn from_ffi(owner: &'a Storage, ptr: *mut ffi::LIBMTP_file_t) -> Self {
		Self { owner, inner: unsafe { *ptr }, inner_ptr: ptr }
	}

	/// Changes the name of the file.
	///
	/// # Errors
	///
	/// An error is returned if a sibling object with the same name already exists.
	///
	/// # Panics
	///
	/// Panics if the name of the file contains a nul byte.
	pub fn rename<'b>(&self, name: &'b str) -> Result<()> {
		let name = CString::new(name).expect("File name should not contain a nul byte");
		let name_ptr = name.as_ptr() as *mut c_char;
		let dev = self.owner().owner();

		let res = unsafe { ffi::LIBMTP_Set_File_Name(dev.inner_ptr(), self.inner_ptr, name_ptr) };
		if res != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Retrieves the ID of the file.
	pub fn id(&self) -> u32 {
		self.inner.item_id
	}

	/// Retrieves the name of the file.
	///
	/// # Panics
	///
	/// Panics if the name of the file is not a valid UTF-8.
	pub fn name(&self) -> &str {
		let name = self.inner.filename;
		unsafe { CStr::from_ptr(name).to_str().expect("File name should be a valid UTF-8") }
	}

	/// Retrieves the kind of the file.
	pub fn kind(&self) -> FileKind {
		FileKind::from_ffi(self.inner.filetype)
			.expect("File type should not be a folder, an album or a playlist")
	}

	/// Retrieves the total size in bytes of the file.
	pub fn size(&self) -> u64 {
		self.inner.filesize
	}

	/// Retrieves the storage to which the file belongs.
	pub(crate) fn owner(&self) -> &Storage {
		self.owner
	}
}

#[doc(hidden)]
impl<'a> Drop for File<'a> {
	fn drop(&mut self) {
		unsafe {
			ffi::LIBMTP_destroy_file_t(self.inner_ptr);
		}
	}
}

impl<'a> Debug for File<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("File")
			.field("id", &self.id())
			.field("name", &self.name())
			.field("kind", &self.kind())
			.field("size", &self.size())
			.finish()
	}
}

/// The kind of the file.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum FileKind {
	/// Waveform Audio format.
	Wav,
	/// MPEG Audio Layer III.
	Mp3,
	/// Windows Media Audio.
	Wma,
	/// Ogg Vorbis Audio.
	Ogg,
	/// Audible Audio.
	Audible,
	/// MPEG-4 Part 14.
	Mp4,
	/// Other audio format.
	OtherAudio,
	/// Windows Media Video.
	Wmv,
	/// Audio Video Interleave.
	Avi,
	/// Moving Picture Experts Group.
	Mpeg,
	/// Advanced Streaming Format.
	Asf,
	/// QuickTime.
	Qt,
	/// Other video format.
	OtherVideo,
	/// Joint Photographic Experts Group.
	Jpeg,
	/// JPEG File Interchange Format.
	Jfif,
	/// Tagged Image File Format.
	Tiff,
	/// Bitmap.
	Bmp,
	/// Graphics Interchange Format.
	Gif,
	/// PICT.
	Pict,
	/// Portable Network Graphics.
	Png,
	/// vCalendar 1.0.
	VCalendar1,
	/// vCalendar 2.0.
	VCalendar2,
	/// vCard 1.0.
	VCard2,
	/// vCard 2.0.
	VCard3,
	/// Windows Imaging Format.
	Wim,
	/// Windows Batch file.
	Batch,
	/// Plain text.
	Text,
	/// Hypertext Markup Language.
	Html,
	/// Firmware file.
	Firmware,
	/// Advanced Audio Codec.
	Aac,
	/// Media Card Format.
	MediaCard,
	/// Free Lossless Audio Codec.
	Flac,
	/// MPEG Audio Layer II.
	Mp2,
	/// MPEG-4 Audio.
	M4a,
	/// Microsoft Word Document.
	Doc,
	/// Extensible Markup Language.
	Xml,
	/// Microsoft Excel Spreadsheet.
	Xls,
	/// Microsoft PowerPoint Presentation.
	Ppt,
	/// MIME encapsulation of aggregate HTML documents.
	Mht,
	/// JPEG 2000 Image Format.
	Jp2,
	/// JPEG 2000 Image Format (Part 1).
	Jpx,
	/// Other file kind.
	#[default]
	Other,
}

impl FileKind {
	pub(crate) fn from_ffi(filetype: ffi::LIBMTP_filetype_t) -> Option<Self> {
		match filetype {
			ffi::LIBMTP_filetype_t::Wav => Some(Self::Wav),
			ffi::LIBMTP_filetype_t::Mp3 => Some(Self::Mp3),
			ffi::LIBMTP_filetype_t::Wma => Some(Self::Wma),
			ffi::LIBMTP_filetype_t::Ogg => Some(Self::Ogg),
			ffi::LIBMTP_filetype_t::Audible => Some(Self::Audible),
			ffi::LIBMTP_filetype_t::Mp4 => Some(Self::Mp4),
			ffi::LIBMTP_filetype_t::UndefAudio => Some(Self::OtherAudio),
			ffi::LIBMTP_filetype_t::Wmv => Some(Self::Wmv),
			ffi::LIBMTP_filetype_t::Avi => Some(Self::Avi),
			ffi::LIBMTP_filetype_t::Mpeg => Some(Self::Mpeg),
			ffi::LIBMTP_filetype_t::Asf => Some(Self::Asf),
			ffi::LIBMTP_filetype_t::Qt => Some(Self::Qt),
			ffi::LIBMTP_filetype_t::UndefVideo => Some(Self::OtherVideo),
			ffi::LIBMTP_filetype_t::Jpeg => Some(Self::Jpeg),
			ffi::LIBMTP_filetype_t::Jfif => Some(Self::Jfif),
			ffi::LIBMTP_filetype_t::Tiff => Some(Self::Tiff),
			ffi::LIBMTP_filetype_t::Bmp => Some(Self::Bmp),
			ffi::LIBMTP_filetype_t::Gif => Some(Self::Gif),
			ffi::LIBMTP_filetype_t::Pict => Some(Self::Pict),
			ffi::LIBMTP_filetype_t::Png => Some(Self::Png),
			ffi::LIBMTP_filetype_t::VCalendar1 => Some(Self::VCalendar1),
			ffi::LIBMTP_filetype_t::VCalendar2 => Some(Self::VCalendar2),
			ffi::LIBMTP_filetype_t::VCard2 => Some(Self::VCard2),
			ffi::LIBMTP_filetype_t::VCard3 => Some(Self::VCard3),
			ffi::LIBMTP_filetype_t::WindowsImageFormat => Some(Self::Wim),
			ffi::LIBMTP_filetype_t::WinExec => Some(Self::Batch),
			ffi::LIBMTP_filetype_t::Text => Some(Self::Text),
			ffi::LIBMTP_filetype_t::Html => Some(Self::Html),
			ffi::LIBMTP_filetype_t::Firmware => Some(Self::Firmware),
			ffi::LIBMTP_filetype_t::Aac => Some(Self::Aac),
			ffi::LIBMTP_filetype_t::MediaCard => Some(Self::MediaCard),
			ffi::LIBMTP_filetype_t::Flac => Some(Self::Flac),
			ffi::LIBMTP_filetype_t::Mp2 => Some(Self::Mp2),
			ffi::LIBMTP_filetype_t::M4a => Some(Self::M4a),
			ffi::LIBMTP_filetype_t::Doc => Some(Self::Doc),
			ffi::LIBMTP_filetype_t::Xml => Some(Self::Xml),
			ffi::LIBMTP_filetype_t::Xls => Some(Self::Xls),
			ffi::LIBMTP_filetype_t::Ppt => Some(Self::Ppt),
			ffi::LIBMTP_filetype_t::Mht => Some(Self::Mht),
			ffi::LIBMTP_filetype_t::Jp2 => Some(Self::Jp2),
			ffi::LIBMTP_filetype_t::Jpx => Some(Self::Jpx),
			ffi::LIBMTP_filetype_t::Unknown => Some(Self::Other),
			_ => None,
		}
	}
}

/// An iterator over the objects of the folder.
#[derive(Clone, Copy)]
pub struct Iter<'a> {
	/// The storage to which the object belongs.
	storage: &'a Storage<'a>,
	/// The pointer to the underlying structure of the object.
	ptr: *mut ffi::LIBMTP_file_t,
	/// The responsibility of the object for the pointer cleanup.
	ownership: Ownership,
}

impl<'a> Iter<'a> {
	/// Constructs a new objects iterator.
	pub(crate) fn new(
		storage: &'a Storage,
		ptr: *mut ffi::LIBMTP_file_t,
		ownership: Ownership,
	) -> Self {
		Self { storage, ptr, ownership }
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = Object<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ptr.is_null() {
			return None;
		}

		let obj = Object::from_ffi(self.storage, self.ptr, self.ownership);
		self.ptr = unsafe { *self.ptr }.next;
		Some(obj)
	}
}
