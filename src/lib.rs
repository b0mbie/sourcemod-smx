use byteorder::{
	ByteOrder,
	ReadBytesExt,
	WriteBytesExt,
};
use core::ffi::CStr;
use std::{
	collections::HashMap,
	ffi::CString,
	hash::Hash,
	io::{
		Result as IoResult, Seek
	}
};

pub use byteorder;

mod opcodes;
pub mod smx_table;
pub mod smx;
pub mod vm_types;

pub use opcodes::Instruction;
pub use smx::CompressionLevel;

use smx::{
	Endianness, Section, SmxError, WriteSmx
};

/// Helper structure that represents an SMX file.
/// 
/// This structure has the [`Smx::read_from`] and [`Smx::write_to`] functions
/// for reading and writing SMX files respectively.
/// If you want to enforce a strict meaning to sections, consider using
/// [`WriteSmx`].
#[derive(Default, Debug, Clone)]
pub struct Smx<Name, Sect> {
	pub sections: HashMap<Name, Sect>,
}

impl<Name: Eq + Hash, Sect: PartialEq> PartialEq for Smx<Name, Sect> {
	fn eq(&self, other: &Self) -> bool {
		self.sections == other.sections
	}
}

impl<Name: Eq + Hash, Sect: Eq> Eq for Smx<Name, Sect> {}

impl<Name, Sect> Smx<Name, Sect> {
	/// Create an empty [`Smx`].
	pub fn new() -> Self {
		Self {
			sections: HashMap::new(),
		}
	}

	/// Create an [`Smx`] with a specified [`HashMap`] of sections.
	pub const fn with_sections(sections: HashMap<Name, Sect>) -> Self {
		Self {
			sections,
		}
	}
}

impl<Name: AsRef<CStr>, Sect: Section> Smx<Name, Sect> {
	/// Write this SMX file to a writer.
	pub fn write_to<E: ByteOrder>(
		&self, w: &mut impl WriteBytesExt,
		compression_level: CompressionLevel,
	) -> IoResult<()> {
		smx::write_to::<E, HashMap<Name, Sect>>(
			w, compression_level, &self.sections
		)
	}
}

impl<Name: From<CString> + Eq + Hash, Sect: From<Vec<u8>>>
	WriteSmx
	for Smx<Name, Sect>
{
	type Error = never_say_never::Never;
	fn write_section(
		&mut self,
		name: CString, data: Vec<u8>
	) -> Result<(), Self::Error> {
		self.sections.insert(name.into(), data.into());
		Ok(())
	}
}

impl<Name: From<CString> + Eq + Hash, Sect: From<Vec<u8>>> Smx<Name, Sect> {
	/// Read an SMX file from a reader.
	pub fn read_from(
		r: &mut (impl ReadBytesExt + Seek)
	) -> Result<(Self, Endianness), SmxError<<Self as WriteSmx>::Error>> {
		let mut smx = Self::new();
		let endianness = smx::read_from(r, &mut smx)?;
		Ok((smx, endianness))
	}
}

/// Helper macro to calculate the size of a packed structure.
/// 
/// This macro accepts any input in the form of `first_type ('+' next_type)*`,
/// and uses [`size_of`](core::mem::size_of) to calculate each type's size,
/// summing the sizes afterwards.
#[macro_export]
macro_rules! size_of {
	($ty:ident $(+ $ty_2:ident)*) => {
		core::mem::size_of::<$ty>() $(+ core::mem::size_of::<$ty_2>())*
	};
}

#[cfg(test)]
mod helper_tests {
	use super::{
		byteorder::{
			BigEndian as Be,
			LittleEndian as Le,
		},
		smx::{
			CompressionLevel,
			Endianness,
		},
		Smx,
	};
	use std::{
		ffi::CString,
		error::Error,
	};

	type Sx = Smx<CString, Vec<u8>>;

	fn hex_dump(data: &[u8]) {
		for (offset, window) in
			((0usize..).map(move |offset| offset * 0x10))
				.zip(data.chunks(16))
		{
			print!("{offset:08x} ");

			for byte in window.iter().copied() {
				print!(
					"{}",
					char::from_u32(byte as _)
						.filter(move |ch| !ch.is_control())
						.unwrap_or('.')
				);
			}

			for _ in window.len()..16 {
				print!(" ");
			}

			print!(" | ");
			for byte in window.iter().copied() {
				print!("{byte:02x} ");
			}

			println!();
		}
	}

	#[test]
	fn empty_uncompressed() {
		use std::{
			collections::HashMap,
			io::Cursor,
		};
	
		let smx = Sx {
			sections: HashMap::new(),
		};
		
		let mut data = Vec::new();
		smx.write_to::<Le>(&mut data, CompressionLevel::NoCompression).unwrap();
		hex_dump(&data);
	
		assert_eq!(
			Sx::read_from(&mut Cursor::new(data)).unwrap(),
			(smx, Endianness::Little)
		);
	}
	
	#[test]
	fn empty() -> Result<(), Box<dyn Error>> {
		use std::{
			collections::HashMap,
			io::Cursor,
		};
	
		let smx = Sx {
			sections: HashMap::new(),
		};
		
		let mut data = Vec::new();
		smx.write_to::<Be>(&mut data, CompressionLevel::DefaultLevel)?;
		hex_dump(&data);
	
		assert_eq!(Sx::read_from(&mut Cursor::new(data))?, (smx, Endianness::Big));
	
		Ok(())
	}
	
	#[test]
	fn two_empty_sections() -> Result<(), Box<dyn Error>> {
		use std::{
			collections::HashMap,
			io::Cursor,
		};
	
		let mut smx = Sx {
			sections: HashMap::new(),
		};
		smx.sections.insert(CString::new(b".section_a")?, vec![]);
		smx.sections.insert(CString::new(b".section_b")?, vec![]);
	
		let mut data = Vec::new();
		smx.write_to::<Le>(&mut data, CompressionLevel::DefaultLevel)?;
		hex_dump(&data);
	
		assert_eq!(Smx::read_from(&mut Cursor::new(data))?, (smx, Endianness::Little));
	
		Ok(())
	}
	
	#[test]
	fn one_empty_section() -> Result<(), Box<dyn Error>> {
		use std::{
			collections::HashMap,
			io::Cursor,
		};
	
		let mut smx = Smx {
			sections: HashMap::new(),
		};
		smx.sections.insert(CString::new(b".section_a")?, vec![]);
		
		let mut data = Vec::new();
		smx.write_to::<Be>(&mut data, CompressionLevel::NoCompression)?;
		hex_dump(&data);
	
		assert_eq!(Smx::read_from(&mut Cursor::new(data))?, (smx, Endianness::Big));
	
		Ok(())
	}
	
	#[test]
	fn two_filled_sections() -> Result<(), Box<dyn Error>> {
		use std::{
			collections::HashMap,
			io::Cursor,
		};
	
		let mut smx = Smx {
			sections: HashMap::new(),
		};
		smx.sections.insert(CString::new(b".section_a")?, vec![4, 20, 133, 7]);
		smx.sections.insert(CString::new(b".section_b")?, vec![1, 2, 3, 4, 5, 6]);
		
		let mut data = Vec::new();
		smx.write_to::<Le>(&mut data, CompressionLevel::NoCompression)?;
		hex_dump(&data);
	
		assert_eq!(Smx::read_from(&mut Cursor::new(data))?, (smx, Endianness::Little));
	
		Ok(())
	}
}
