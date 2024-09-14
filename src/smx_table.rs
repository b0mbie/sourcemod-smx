//! See [`CStrTable`].

use byteorder::WriteBytesExt;
use std::{
	ffi::{
		CStr, CString
	},
	io::Result as IoResult,
	iter::once
};

/// Structure that holds an owned binary blob of C strings.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct CStrTable {
	blob: Vec<u8>,
}

/// Iterator over C strings in a [`CStrTable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Iter<'a> {
	table: &'a CStrTable,
	offset: usize,
}

impl<'a> Iterator for Iter<'a> {
	type Item = (usize, &'a [u8]);
	fn next(&mut self) -> Option<Self::Item> {
		let begin = self.offset;
		let blob_len = self.table.blob.len();
		if begin < blob_len {
			let nul = loop {
				let prev_offset = self.offset;
				self.offset += 1;

				if prev_offset >= blob_len {
					break prev_offset
				}

				if self.table.blob[prev_offset] == 0 {
					break prev_offset
				}
			};
			Some((begin, &self.table.blob[begin..nul]))
		} else {
			None
		}
	}
}

impl CStrTable {
	/// Create an empty [`CStrTable`].
	pub const fn new() -> Self {
		Self {
			blob: Vec::new(),
		}
	}

	/// Create a new [`CStrTable`] from a blob of C strings.
	pub const fn from_blob(blob: Vec<u8>) -> Self {
		Self {
			blob,
		}
	}

	/// Return a reference to this table's C string blob.
	pub const fn blob(&self) -> &Vec<u8> {
		&self.blob
	}

	/// Write this table to a writer.
	pub fn write_to(&self, w: &mut impl WriteBytesExt) -> IoResult<()> {
		w.write_all(&self.blob)
	}

	/// Return the length of this table's C string blob.
	pub fn len(&self) -> usize {
		self.blob.len()
	}

	/// Return `true` if this table is empty.
	pub fn is_empty(&self) -> bool {
		self.blob.is_empty()
	}

	/// Create an iterator over all C strings in this table with the associated
	/// offset to each one.
	pub const fn iter(&self) -> Iter<'_> {
		Iter {
			table: self,
			offset: 0,
		}
	}

	/// Try to create a [`CString`] from an offset into this table.
	pub fn get_c_string(&self, offset: usize) -> Option<CString> {
		if offset < self.blob.len() {
			let buffer = self.blob[offset..].iter()
				.copied()
				.take_while(move |b| *b != 0)
				.chain(once(0))
				.collect();
	
			Some(unsafe { CString::from_vec_with_nul_unchecked(buffer) })
		} else {
			None
		}
	}

	/// Put a C string into this table.
	pub fn insert(&mut self, data: impl AsRef<CStr>) -> usize {
		self.iter()
			.find_map(|(offset, piece)| {
				(piece == data.as_ref().to_bytes()).then_some(offset)
			})
			.unwrap_or_else(move || {
				let offset = self.blob.len();
				self.blob.extend_from_slice(data.as_ref().to_bytes_with_nul());
				offset
			})
	}
}

impl AsRef<[u8]> for CStrTable {
	fn as_ref(&self) -> &[u8] {
		&self.blob
	}
}

#[test]
fn empty() -> Result<(), Box<dyn std::error::Error>> {
	let table = CStrTable::new();
	let mut data = Vec::new();
	table.write_to(&mut data)?;
	assert_eq!(&data, &[]);
	Ok(())
}

#[test]
fn one_entry() -> Result<(), Box<dyn std::error::Error>> {
	let mut table = CStrTable::new();
	table.insert(CStr::from_bytes_with_nul(b".code\0")?);
	let mut data = Vec::new();
	table.write_to(&mut data)?;
	assert_eq!(&data, b".code\0");
	Ok(())
}

#[test]
fn dup_entry() -> Result<(), Box<dyn std::error::Error>> {
	let mut table = CStrTable::new();
	let key_1 = table.insert(CStr::from_bytes_with_nul(b".code\0")?);
	let key_2 = table.insert(CStr::from_bytes_with_nul(b".code\0")?);
	assert_eq!(key_1, key_2);

	let mut data = Vec::new();
	table.write_to(&mut data)?;
	assert_eq!(&data, b".code\0");
	Ok(())
}

#[test]
fn entries_and_dup() -> Result<(), Box<dyn std::error::Error>> {
	let mut table = CStrTable::new();
	table.insert(CStr::from_bytes_with_nul(b"OnPluginStart\0")?);
	let key_1 = table.insert(CStr::from_bytes_with_nul(b"LogMessage\0")?);
	table.insert(CStr::from_bytes_with_nul(b"OnPluginEnd\0")?);
	let key_2 = table.insert(CStr::from_bytes_with_nul(b"LogMessage\0")?);
	assert_eq!(key_1, key_2);
	table.insert(CStr::from_bytes_with_nul(b"OnPluginStart\0")?);

	let mut data = Vec::new();
	table.write_to(&mut data)?;
	assert_eq!(&data, b"OnPluginStart\0LogMessage\0OnPluginEnd\0");
	Ok(())
}
