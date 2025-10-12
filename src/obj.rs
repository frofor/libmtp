//! This module provides API for managing files and folders of storages.

use crate::Result;
use crate::Storage;
use crate::convert::path_to_cstring;
use crate::ffi;
use libc::time_t;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs;
use std::path::Path;
use std::ptr;
use std::time::UNIX_EPOCH;

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
	///
	/// # Safety
	///
	/// `ptr` should not be null.
	pub(crate) unsafe fn new_unchecked(
		owner: &'a Storage,
		ptr: *mut ffi::LIBMTP_file_t,
		ownership: Ownership,
	) -> Self {
		let inner = unsafe { *ptr };
		if FileKind::new(inner.filetype).is_none() {
			return Self::Folder(unsafe { Folder::new_unchecked(owner, ptr, ownership) });
		}
		Self::File(unsafe { File::new_unchecked(owner, ptr) })
	}

	/// Retrieves the ID of the object.
	#[must_use]
	pub fn id(&self) -> u32 {
		match self {
			Self::Folder(f) => f.id(),
			Self::File(f) => f.id(),
		}
	}

	/// Retrieves the name of the object.
	#[must_use]
	pub fn name(&self) -> &str {
		match self {
			Self::Folder(f) => f.name(),
			Self::File(f) => f.name(),
		}
	}

	/// Checks if the object is folder.
	#[must_use]
	pub fn is_folder(&self) -> bool {
		matches!(self, Self::Folder(_))
	}

	/// Checks if the object is file.
	#[must_use]
	pub fn is_file(&self) -> bool {
		matches!(self, Self::File(_))
	}

	/// Changes the name of the object.
	///
	/// # Errors
	///
	/// Returns an error if a sibling object with the same name already exists or if the
	/// operation has failed.
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

	/// Moves the object to the other folder.
	///
	/// # Errors
	///
	/// Returns an error if an object with the same name already exists in the other folder or
	/// if the operation has failed.
	pub fn move_to(&self, parent: &Folder) -> Result<()> {
		match self {
			Self::Folder(f) => f.move_to(parent),
			Self::File(f) => f.move_to(parent),
		}
	}

	/// Copies the object to the other folder.
	///
	/// # Errors
	///
	/// Returns an error if an object with the same name already exists in the other folder or
	/// if the operation has failed.
	pub fn copy_to(&self, parent: &Folder) -> Result<()> {
		match self {
			Self::Folder(f) => f.copy_to(parent),
			Self::File(f) => f.copy_to(parent),
		}
	}

	/// Deletes the object from the storage.
	///
	/// # Errors
	///
	/// Returns an error if the operation has failed.
	pub fn delete(&self) -> Result<()> {
		match self {
			Self::Folder(f) => f.delete(),
			Self::File(f) => f.delete(),
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
	pub(crate) unsafe fn new_unchecked(
		owner: &'a Storage,
		ptr: *mut ffi::LIBMTP_file_t,
		ownership: Ownership,
	) -> Self {
		Self { owner, inner: unsafe { *ptr }, inner_ptr: ptr, ownership }
	}

	/// Retrieves the ID of the folder.
	#[must_use]
	pub fn id(&self) -> u32 {
		self.inner.item_id
	}

	/// Retrieves the name of the folder.
	///
	/// # Panics
	///
	/// Panics if the name of the folder is not a valid UTF-8.
	#[must_use]
	pub fn name(&self) -> &str {
		let name = self.inner.filename;
		unsafe { CStr::from_ptr(name).to_str().expect("Folder name should be a valid UTF-8") }
	}

	/// Retrieves the storage to which the folder belongs.
	pub(crate) fn owner(&self) -> &Storage {
		self.owner
	}

	/// Changes the name of the foler.
	///
	/// # Errors
	///
	/// Returns an error if a sibling object with the same name already exists or if the
	/// operation has failed.
	///
	/// # Panics
	///
	/// Panics if the name of the folder contains a nul byte.
	pub fn rename(&self, name: &str) -> Result<()> {
		let name = CString::new(name).expect("Folder name should not contain a nul byte");
		let name_ptr = name.as_ptr();
		let dev = self.owner().owner();

		let n = unsafe { ffi::LIBMTP_Set_File_Name(dev.inner_ptr(), self.inner_ptr, name_ptr) };
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Moves the folder to the other folder.
	///
	/// # Errors
	///
	/// Returns an error if an object with the same name already exists in the other folder or
	/// if the operation has failed.
	pub fn move_to(&self, parent: &Folder) -> Result<()> {
		let storage = self.owner();
		let dev = storage.owner();
		let dev_ptr = dev.inner_ptr();

		let n = unsafe { ffi::LIBMTP_Move_Object(dev_ptr, self.id(), storage.id(), parent.id()) };
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Copies the folder to the other folder.
	///
	/// # Errors
	///
	/// Returns an error if an object with the same name already exists in the other folder or
	/// if the operation has failed.
	pub fn copy_to(&self, parent: &Folder) -> Result<()> {
		let storage = self.owner();
		let dev = storage.owner();
		let dev_ptr = dev.inner_ptr();

		let n = unsafe { ffi::LIBMTP_Copy_Object(dev_ptr, self.id(), storage.id(), parent.id()) };
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Deletes the folder from the storage.
	///
	/// # Errors
	///
	/// Returns an error if the operation has failed.
	pub fn delete(&self) -> Result<()> {
		let dev = self.owner().owner();

		let n = unsafe { ffi::LIBMTP_Delete_Object(dev.inner_ptr(), self.id()) };
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Creates a new folder inside the folder. Returns the ID of the created folder.
	///
	/// # Errors
	///
	/// Returns an error if a child object with the same name already exists or if the
	/// operation has failed.
	///
	/// # Panics
	///
	/// Panics if the name of the folder contains a nul byte.
	pub fn create_folder(&self, name: &str) -> Result<u32> {
		let name = CString::new(name).expect("Folder name should not contain a nul byte");
		let name_ptr = name.as_ptr().cast_mut();
		let storage = self.owner();
		let dev = storage.owner();
		let dev_ptr = dev.inner_ptr();

		let id = unsafe { ffi::LIBMTP_Create_Folder(dev_ptr, name_ptr, self.id(), storage.id()) };
		if id == 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(id)
	}

	/// Copies the file from the host computer to the folder.
	///
	/// # Errors
	///
	/// Returns an error if reading the file's metadata has failed or if the operation has
	/// failed.
	///
	/// # Panics
	///
	/// Panics if the path doesn't end in file name, or if file name is not a valid UTF-8.
	pub fn copy_file_from_host<P>(&self, path: P, kind: FileKind) -> Result<()>
	where
		P: AsRef<Path>,
	{
		let path = path.as_ref();

		let name = CString::new(
			path.file_name()
				.expect("Path should end in filename")
				.to_str()
				.expect("File name should be a valid UTF-8"),
		)
		.expect("Path should not contain a nul byte");

		let metadata = fs::metadata(path)?;

		let file = unsafe { &mut *ffi::LIBMTP_new_file_t() };
		file.parent_id = self.id();
		file.storage_id = self.owner().id();
		file.filename = name.as_ptr().cast_mut();
		file.filesize = metadata.len();
		#[allow(clippy::cast_possible_wrap)]
		{
			file.modificationdate = metadata
				.modified()
				.unwrap_or(UNIX_EPOCH)
				.duration_since(UNIX_EPOCH)
				.expect("Modification date should not be before Unix epoch")
				.as_secs() as time_t;
		}
		file.filetype = kind.to_ffi();

		let dev = self.owner().owner();
		let path = path_to_cstring(path);

		let n = unsafe {
			ffi::LIBMTP_Send_File_From_File(dev.inner_ptr(), path.as_ptr(), file, None, ptr::null())
		};
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Retrieves an iterator over the objects of the folder.
	#[must_use]
	pub fn iter(&self) -> ObjectIter {
		let ptr = unsafe {
			ffi::LIBMTP_Get_Files_And_Folders(
				self.owner.owner().inner_ptr(),
				self.owner.id(),
				self.id(),
			)
		};
		unsafe { ObjectIter::new_unchecked(self.owner, ptr, Ownership::Borrows) }
	}

	/// Retrieves a recursive iterator over the objects of the folder.
	#[must_use]
	pub fn iter_recursive(&self) -> ObjectRecursiveIter {
		let ptr = unsafe {
			ffi::LIBMTP_Get_Files_And_Folders(
				self.owner.owner().inner_ptr(),
				self.owner.id(),
				self.id(),
			)
		};
		unsafe { ObjectRecursiveIter::new_unchecked(self.owner, ptr, Ownership::Borrows) }
	}
}

#[doc(hidden)]
impl Drop for Folder<'_> {
	fn drop(&mut self) {
		if self.ownership == Ownership::Borrows {
			return;
		}
		unsafe { ffi::LIBMTP_destroy_file_t(self.inner_ptr) };
	}
}

impl Debug for Folder<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Folder").field("id", &self.id()).field("name", &self.name()).finish()
	}
}

impl<'a> IntoIterator for &'a Folder<'a> {
	type Item = Object<'a>;
	type IntoIter = ObjectIter<'a>;

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
	///
	/// # Safety
	///
	/// `ptr` should not be null.
	pub(crate) unsafe fn new_unchecked(owner: &'a Storage, ptr: *mut ffi::LIBMTP_file_t) -> Self {
		Self { owner, inner: unsafe { *ptr }, inner_ptr: ptr }
	}

	/// Retrieves the ID of the file.
	#[must_use]
	pub fn id(&self) -> u32 {
		self.inner.item_id
	}

	/// Retrieves the name of the file.
	///
	/// # Panics
	///
	/// Panics if the name of the file is not a valid UTF-8.
	#[must_use]
	pub fn name(&self) -> &str {
		let name = self.inner.filename;
		unsafe { CStr::from_ptr(name).to_str().expect("File name should be a valid UTF-8") }
	}

	/// Retrieves the kind of the file.
	///
	/// # Panics
	///
	/// Panics if the kind of the file is a folder, an album or a playlist.
	#[must_use]
	pub fn kind(&self) -> FileKind {
		FileKind::new(self.inner.filetype)
			.expect("File type should not be a folder, an album or a playlist")
	}

	/// Retrieves the total size in bytes of the file.
	#[must_use]
	pub fn size(&self) -> u64 {
		self.inner.filesize
	}

	/// Retrieves the storage to which the file belongs.
	pub(crate) fn owner(&self) -> &Storage {
		self.owner
	}

	/// Changes the name of the file.
	///
	/// # Errors
	///
	/// Returns an error if a sibling object with the same name already exists or if the
	/// operation has failed.
	///
	/// # Panics
	///
	/// Panics if the name of the file contains a nul byte.
	pub fn rename(&self, name: &str) -> Result<()> {
		let name = CString::new(name).expect("File name should not contain a nul byte");
		let name_ptr = name.as_ptr();
		let dev = self.owner().owner();

		let n = unsafe { ffi::LIBMTP_Set_File_Name(dev.inner_ptr(), self.inner_ptr, name_ptr) };
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Moves the file to the other folder.
	///
	/// # Errors
	///
	/// Returns an error if an object with the same name already exists in the other folder or
	/// if the operation has failed.
	pub fn move_to(&self, parent: &Folder) -> Result<()> {
		let storage = self.owner();
		let dev = storage.owner();
		let dev_ptr = dev.inner_ptr();

		let n = unsafe { ffi::LIBMTP_Move_Object(dev_ptr, self.id(), storage.id(), parent.id()) };
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Copies the file to the other folder.
	///
	/// # Errors
	///
	/// Returns an error if an object with the same name already exists in the other folder or
	/// if the operation has failed.
	pub fn copy_to(&self, parent: &Folder) -> Result<()> {
		let storage = self.owner();
		let dev = storage.owner();
		let dev_ptr = dev.inner_ptr();

		let n = unsafe { ffi::LIBMTP_Copy_Object(dev_ptr, self.id(), storage.id(), parent.id()) };
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Copies the file to the host computer.
	///
	/// # Errors
	///
	/// Returns an error if the operation has failed.
	pub fn copy_to_host<P>(&self, path: P) -> Result<()>
	where
		P: AsRef<Path>,
	{
		let dev = self.owner().owner();
		let path = path_to_cstring(path.as_ref());

		let n = unsafe {
			ffi::LIBMTP_Get_File_To_File(
				dev.inner_ptr(),
				self.id(),
				path.as_ptr(),
				None,
				ptr::null(),
			)
		};
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}

	/// Deletes the file from the storage.
	///
	/// # Errors
	///
	/// Returns an error if the operation has failed.
	pub fn delete(&self) -> Result<()> {
		let dev = self.owner().owner();

		let n = unsafe { ffi::LIBMTP_Delete_Object(dev.inner_ptr(), self.id()) };
		if n != 0 {
			return Err(dev.pop_err().unwrap_or_default());
		}
		Ok(())
	}
}

#[doc(hidden)]
impl Drop for File<'_> {
	fn drop(&mut self) {
		unsafe { ffi::LIBMTP_destroy_file_t(self.inner_ptr) };
	}
}

impl Debug for File<'_> {
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
	#[allow(clippy::doc_markdown)]
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
	#[allow(clippy::doc_markdown)]
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
	/// Constructs a new file kind.
	pub(crate) fn new(filetype: ffi::LIBMTP_filetype_t) -> Option<Self> {
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

	/// Converts the file kind to the underlying enumerator.
	pub(crate) fn to_ffi(self) -> ffi::LIBMTP_filetype_t {
		match self {
			Self::Wav => ffi::LIBMTP_filetype_t::Wav,
			Self::Mp3 => ffi::LIBMTP_filetype_t::Mp3,
			Self::Wma => ffi::LIBMTP_filetype_t::Wma,
			Self::Ogg => ffi::LIBMTP_filetype_t::Ogg,
			Self::Audible => ffi::LIBMTP_filetype_t::Audible,
			Self::Mp4 => ffi::LIBMTP_filetype_t::Mp4,
			Self::OtherAudio => ffi::LIBMTP_filetype_t::UndefAudio,
			Self::Wmv => ffi::LIBMTP_filetype_t::Wmv,
			Self::Avi => ffi::LIBMTP_filetype_t::Avi,
			Self::Mpeg => ffi::LIBMTP_filetype_t::Mpeg,
			Self::Asf => ffi::LIBMTP_filetype_t::Asf,
			Self::Qt => ffi::LIBMTP_filetype_t::Qt,
			Self::OtherVideo => ffi::LIBMTP_filetype_t::UndefVideo,
			Self::Jpeg => ffi::LIBMTP_filetype_t::Jpeg,
			Self::Jfif => ffi::LIBMTP_filetype_t::Jfif,
			Self::Tiff => ffi::LIBMTP_filetype_t::Tiff,
			Self::Bmp => ffi::LIBMTP_filetype_t::Bmp,
			Self::Gif => ffi::LIBMTP_filetype_t::Gif,
			Self::Pict => ffi::LIBMTP_filetype_t::Pict,
			Self::Png => ffi::LIBMTP_filetype_t::Png,
			Self::VCalendar1 => ffi::LIBMTP_filetype_t::VCalendar1,
			Self::VCalendar2 => ffi::LIBMTP_filetype_t::VCalendar2,
			Self::VCard2 => ffi::LIBMTP_filetype_t::VCard2,
			Self::VCard3 => ffi::LIBMTP_filetype_t::VCard3,
			Self::Wim => ffi::LIBMTP_filetype_t::WindowsImageFormat,
			Self::Batch => ffi::LIBMTP_filetype_t::WinExec,
			Self::Text => ffi::LIBMTP_filetype_t::Text,
			Self::Html => ffi::LIBMTP_filetype_t::Html,
			Self::Firmware => ffi::LIBMTP_filetype_t::Firmware,
			Self::Aac => ffi::LIBMTP_filetype_t::Aac,
			Self::MediaCard => ffi::LIBMTP_filetype_t::MediaCard,
			Self::Flac => ffi::LIBMTP_filetype_t::Flac,
			Self::Mp2 => ffi::LIBMTP_filetype_t::Mp2,
			Self::M4a => ffi::LIBMTP_filetype_t::M4a,
			Self::Doc => ffi::LIBMTP_filetype_t::Doc,
			Self::Xml => ffi::LIBMTP_filetype_t::Xml,
			Self::Xls => ffi::LIBMTP_filetype_t::Xls,
			Self::Ppt => ffi::LIBMTP_filetype_t::Ppt,
			Self::Mht => ffi::LIBMTP_filetype_t::Mht,
			Self::Jp2 => ffi::LIBMTP_filetype_t::Jp2,
			Self::Jpx => ffi::LIBMTP_filetype_t::Jpx,
			Self::Other => ffi::LIBMTP_filetype_t::Unknown,
		}
	}
}

/// An iterator over the objects of the folder.
#[derive(Clone)]
pub struct ObjectIter<'a> {
	/// The storage to which the object belongs.
	storage: &'a Storage<'a>,
	/// The pointer to the underlying structure of the object.
	ptr: *mut ffi::LIBMTP_file_t,
	/// The responsibility of the object for the pointer cleanup.
	ownership: Ownership,
}

impl<'a> ObjectIter<'a> {
	/// Constructs a new objects iterator.
	///
	/// # Safety
	///
	/// `ptr` should not be null.
	pub(crate) unsafe fn new_unchecked(
		storage: &'a Storage,
		ptr: *mut ffi::LIBMTP_file_t,
		ownership: Ownership,
	) -> Self {
		Self { storage, ptr, ownership }
	}
}

impl<'a> Iterator for ObjectIter<'a> {
	type Item = Object<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ptr.is_null() {
			return None;
		}

		let obj = unsafe { Object::new_unchecked(self.storage, self.ptr, self.ownership) };
		self.ptr = unsafe { *self.ptr }.next;
		Some(obj)
	}
}

/// A recursive iterator over the objects of the folder.
#[derive(Clone)]
pub struct ObjectRecursiveIter<'a> {
	/// The storage to which the object belongs.
	storage: &'a Storage<'a>,
	/// The stack that holds the IDs of unvisited folders.
	stack: Vec<u32>,
	/// The pointer to the underlying structure of the object.
	ptr: *mut ffi::LIBMTP_file_t,
	/// The responsibility of the object for the pointer cleanup.
	ownership: Ownership,
}

impl<'a> ObjectRecursiveIter<'a> {
	/// Constructs a new recursive objects iterator.
	///
	/// # Safety
	///
	/// `ptr` should not be null.
	pub(crate) unsafe fn new_unchecked(
		storage: &'a Storage,
		ptr: *mut ffi::LIBMTP_file_t,
		ownership: Ownership,
	) -> Self {
		Self { storage, stack: Vec::new(), ptr, ownership }
	}
}

impl<'a> Iterator for ObjectRecursiveIter<'a> {
	type Item = Object<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.ptr.is_null() {
			let id = self.stack.pop()?;
			let dev_ptr = self.storage.owner().inner_ptr();
			let storage_id = self.storage.id();
			self.ptr = unsafe { ffi::LIBMTP_Get_Files_And_Folders(dev_ptr, storage_id, id) };
		}

		let obj = unsafe { Object::new_unchecked(self.storage, self.ptr, self.ownership) };
		if obj.is_folder() {
			self.stack.push(obj.id());
		}

		self.ptr = unsafe { *self.ptr }.next;
		Some(obj)
	}
}
