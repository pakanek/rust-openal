/*!
Helpers related to `Listener`.
*/

mod listener;
pub use self::listener::Listener;

mod attributes;
pub use self::attributes::Attributes;

use std::ptr;

use ffi::*;
use {Error, extension};

/// Opens the default output device.
pub fn default<'a>(attributes: &Attributes) -> Result<Listener<'a>, Error> {
	Listener::default(attributes)
}

/// Opens the named output device.
pub fn open<'a>(name: &str, attributes: &Attributes) -> Result<Listener<'a>, Error> {
	Listener::open(name, attributes)
}

/// Gets a list of available output device names.
pub fn devices() -> Vec<&'static str> {
	use std::ffi::CStr;
	use std::str::from_utf8_unchecked;
	use libc::strlen;

	let mut result = Vec::new();

	unsafe {
		if extension::device::is_supported("ALC_ENUMERATION_EXT") {
			let mut ptr = alcGetString(ptr::null(), ALC_DEVICE_SPECIFIER);

			while *ptr != 0 {
				result.push(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()));

				ptr = ptr.offset(strlen(ptr) as isize + 1);
			}
		}
	}

	result
}
