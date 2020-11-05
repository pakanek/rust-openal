use std::mem;
use std::marker::PhantomData;

use ffi::*;
use ::{Error, Sample};

/// A buffer holding sample data.
#[derive(PartialEq, Eq)]
pub struct Buffer<'a> {
	id: ALuint,

	_marker: PhantomData<&'a ()>,
}

impl<'a> Buffer<'a> {
	#[doc(hidden)]
	pub unsafe fn empty() -> Result<Self, Error> {
		let mut id = 0;
		al_try!(alGenBuffers(1, &mut id));

		Ok(Buffer { id: id, _marker: PhantomData })
	}

	#[doc(hidden)]
	pub unsafe fn new<T: Sample>(channels: u16, data: &[T], rate: u32) -> Result<Self, Error> {
		let mut buffer = (Buffer::empty())?;

		match buffer.fill(channels, data, rate) {
			Ok(..) =>
				Ok(buffer),

			Err(error) =>
				Err(error)
		}
	}

	#[doc(hidden)]
	pub unsafe fn fill<T: Sample>(&mut self, channels: u16, data: &[T], rate: u32) -> Result<(), Error> {
		al_try!(alBufferData(self.id, <T as Sample>::format(channels)?, data.as_ptr() as *const _,
			(mem::size_of::<T>() * data.len()) as ALsizei, rate as ALint));

		Ok(())
	}

	#[doc(hidden)]
	pub unsafe fn id(&self) -> ALuint {
		self.id
	}
}

impl<'a> Buffer<'a> {
	/// The sample rate of the data in the buffer.
	pub fn rate(&self) -> u32 {
		unsafe {
			let mut value = 0;
			alGetBufferi(self.id, AL_FREQUENCY, &mut value);

			value as u32
		}
	}

	/// The bit depth of the data in the buffer.
	pub fn bits(&self) -> u16 {
		unsafe {
			let mut value = 0;
			alGetBufferi(self.id, AL_BITS, &mut value);

			value as u16
		}
	}

	/// The number of channels of the data in the buffer.
	pub fn channels(&self) -> u16 {
		unsafe {
			let mut value = 0;
			alGetBufferi(self.id, AL_CHANNELS, &mut value);

			value as u16
		}
	}

	/// The number of samples in the buffer.
	pub fn len(&self) -> usize {
		unsafe {
			let mut value = 0;
			alGetBufferi(self.id, AL_SIZE, &mut value);

			value as usize
		}
	}
}

impl<'a> ::std::fmt::Debug for Buffer<'a> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		(f.write_str("openal::Buffer("))?;
		(f.write_str(&format!("{}; ", unsafe { self.id() })))?;
		(f.write_str(&format!("rate={} ", self.rate())))?;
		(f.write_str(&format!("bits={} ", self.bits())))?;
		(f.write_str(&format!("channels={} ", self.channels())))?;
		(f.write_str(&format!("len={}", self.len())))?;
		f.write_str(")")
	}
}

impl<'a> Drop for Buffer<'a> {
	fn drop(&mut self) {
		unsafe {
			alDeleteBuffers(1, &self.id);
			al_panic!();
		}
	}
}
