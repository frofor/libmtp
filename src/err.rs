//! This module provides specialized error types used for error handling.

use crate::convert::ptr_to_string;
use crate::ffi;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::result;

/// Specialized Result type used within the crate.
pub type Result<T> = result::Result<T, Error>;

/// Specialized Error type used within the crate.
#[derive(Clone, Hash, Debug)]
pub struct Error {
	/// The category of the error.
	pub kind: Kind,
	/// The message of the error.
	pub(crate) msg: String,
}

impl Error {
	/// Constructs a new error from the kind and the message.
	pub(crate) fn new(kind: Kind, msg: &str) -> Self {
		Self { kind, msg: msg.to_owned() }
	}

	/// Constructs a new error from the underlying structure.
	pub(crate) fn from_stack(stack: *const ffi::LIBMTP_error_t) -> Option<Self> {
		if stack.is_null() {
			return None;
		}

		let mut stack = stack;
		let mut err = unsafe { *stack };

		while !err.next.is_null() {
			stack = err.next;
			err = unsafe { *stack };
		}

		let kind = Kind::from_number(err.errornumber)?;
		let msg = unsafe { ptr_to_string(err.error_text) };
		Some(Self { kind, msg })
	}
}

impl error::Error for Error {}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.msg)
	}
}

impl Default for Error {
	fn default() -> Self {
		Self { kind: Default::default(), msg: "Unknown error".to_owned() }
	}
}

/// Category for an error used within the crate.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum Kind {
	/// General error.
	General,
	/// An error occured at the PTP (Picture Transfer Protocol) layer.
	PtpLayer,
	/// An error occured at the USB layer.
	UsbLayer,
	/// An error due to memory allocation failure.
	MemoryAllocation,
	/// No device is attached.
	NoDeviceAttached,
	/// Error due to insufficient space on the storage of the device.
	StorageFull,
	/// An error occured while connecting to the device.
	Connecting,
	/// The operation was cancelled.
	Cancelled,
	/// Unknown error.
	#[default]
	Unknown,
}

impl Kind {
	/// Constructs an error kind from the underlying structure.
	pub(crate) fn from_number(n: ffi::LIBMTP_error_number_t) -> Option<Self> {
		match n {
			ffi::LIBMTP_error_number_t::None => None,
			ffi::LIBMTP_error_number_t::General => Some(Self::General),
			ffi::LIBMTP_error_number_t::PtpLayer => Some(Self::PtpLayer),
			ffi::LIBMTP_error_number_t::UsbLayer => Some(Self::UsbLayer),
			ffi::LIBMTP_error_number_t::MemoryAllocation => Some(Self::MemoryAllocation),
			ffi::LIBMTP_error_number_t::NoDeviceAttached => Some(Self::NoDeviceAttached),
			ffi::LIBMTP_error_number_t::StorageFull => Some(Self::StorageFull),
			ffi::LIBMTP_error_number_t::Connecting => Some(Self::Connecting),
			ffi::LIBMTP_error_number_t::Cancelled => Some(Self::Cancelled),
		}
	}
}
