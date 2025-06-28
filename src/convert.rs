//! This module provides functions for converting between data types.

use std::ffi::c_char;
use std::slice;

/// Converts a C-style string pointer to a `String`.
///
/// # Safety
///
/// The caller must ensure that `ptr` is a valid pointer to a null-terminated string.
pub(crate) unsafe fn ptr_to_string(ptr: *const c_char) -> String {
	let mut len = 0;
	let mut b = unsafe { *ptr.offset(len) };

	while b != 0 {
		len += 1;
		b = unsafe { *ptr.offset(len) };
	}

	let bytes = unsafe { slice::from_raw_parts(ptr as *const u8, len as usize) };
	String::from_utf8_lossy(bytes).into_owned()
}
