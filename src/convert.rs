//! This module provides functions for converting between data types.

use std::ffi::CString;
use std::ffi::c_char;
use std::path::Path;
use std::slice;

/// Converts a C-style string pointer to a `String`.
///
/// # Safety
///
/// `ptr` should be a valid pointer to a null-terminated string.
pub unsafe fn ptr_to_string(ptr: *const c_char) -> String {
	let mut len = 0;
	let mut c = unsafe { *ptr.offset(len) };

	while c != 0 {
		len += 1;
		c = unsafe { *ptr.offset(len) };
	}

	#[allow(clippy::cast_sign_loss)]
	let bytes = unsafe { slice::from_raw_parts(ptr.cast(), len as usize) };
	String::from_utf8_lossy(bytes).into_owned()
}

#[cfg(any(unix, windows))]
pub fn path_to_cstring(path: &Path) -> CString {
	#[cfg(unix)]
	{
		use std::os::unix::ffi::OsStrExt;
		CString::new(path.as_os_str().as_bytes()).expect("Path should not contain a nul byte")
	}

	#[cfg(windows)]
	{
		use std::os::windows::ffi::OsStrExt;
		let wide: Vec<u16> = path.as_os_str().encode_with().chain(once(0)).collect();
		CString::new(wide.iter().flat_map(|&w| w.to_ne_bytes()).collect::<Vec<u8>>())
			.expect("Path should not contain a nul byte")
	}
}
