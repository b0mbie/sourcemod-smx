//! Utilities for writing and reading SMX files.

use crate::size_of;

use super::smx_table::CStrTable;

use byteorder::{
	ByteOrder,
	BigEndian as Be,
	LittleEndian as Le,
	ReadBytesExt,
	WriteBytesExt,
};
use core::ffi::CStr;
use miniz_oxide::{
	deflate::compress_to_vec_zlib,
	inflate::{
		DecompressError,
		decompress_to_vec_zlib
	}
};
use std::{
	borrow::Cow,
	collections::HashMap,
	error::Error,
	ffi::CString,
	fmt,
	hash::Hash,
	io::{
		Cursor,
		Error as IoError,
		Read,
		Result as IoResult,
		Seek,
		SeekFrom,
	}
};

pub use miniz_oxide::deflate::CompressionLevel;

/// SMX file magic number.
pub const FILE_MAGIC: u32 = 0x53504646;

/// Targetting SourcePawn 1.2.
pub const TARGET_VERSION: u16 = 0x102;

/// Trait for objects which represent a section in an SMX file.
pub trait Section {
	/// Write this section's data to a vector.
	fn write_to(&self, data: &mut Vec<u8>);
}

impl<T: AsRef<[u8]>> Section for T {
	fn write_to(&self, data: &mut Vec<u8>) {
		data.extend_from_slice(self.as_ref())
	}
}

const SMX_HEADER_LEN: usize = size_of!(
	u32 // magic
	+ u16 // target version
	+ u8 // compression type
	+ u32 + u32 // disk size, image size
	+ u8 // section count
	+ u32 // string table offset
	+ u32 // section data offset
);

const SMX_SECTION_INFO_LEN: usize = size_of!(u32 + u32 + u32);

/// Helper trait for [`write_to`].
/// 
/// This is implemented for [`HashMap`]s of sections and [`BorrowedMap`]s.
/// 
/// # Safety
/// The iterator returned by [`SectionMap::iter`] must never yield duplicates.
pub unsafe trait SectionMap<'a> {
	type Name: 'a + AsRef<CStr>;
	type Section: 'a + Section;

	type Iter: Iterator<Item = (&'a Self::Name, &'a Self::Section)>;
	fn iter(&'a self) -> Self::Iter;

	fn len(&self) -> usize;
}

unsafe impl<'a, Name: 'a + AsRef<CStr>, Sect: 'a + Section>
	SectionMap<'a>
	for HashMap<Name, Sect>
{
	type Name = Name;
	type Section = Sect;

	type Iter = std::collections::hash_map::Iter<'a, Self::Name, Self::Section>;
	fn iter(&'a self) -> Self::Iter {
		HashMap::iter(self)
	}

	fn len(&self) -> usize {
		self.len()
	}
}

/// Structure that guarantees that it is holding a slice of pairs with no
/// duplicate keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowedMap<'a, K, V>(&'a [(K, V)]);

impl<'a, K, V> BorrowedMap<'a, K, V> {
	/// Create a new [`BorrowedMap`] without checking for duplicate keys.
	/// 
	/// See [`Self::new`] for a version that does check for duplicate keys
	/// beforehand, but is also very expensive.
	/// 
	/// # Safety
	/// `inner` must contain no duplicate keys.
	pub const unsafe fn new_unchecked(inner: &'a [(K, V)]) -> Self {
		Self(inner)
	}

	pub const fn as_slice(&self) -> &'a [(K, V)] {
		self.0
	}
}

impl<'a, K: PartialEq, V> BorrowedMap<'a, K, V> {
	/// Try to create a new [`BorrowedMap`], returning the pair of indices in
	/// the slice that have identical keys, if there are any.
	/// 
	/// # Note
	/// **This function is very expensive**.
	/// The worst case complexity (if there are indeed no duplicate keys) is 
	/// `O(n^2)`.
	/// Consider using [`Self::new_unchecked`] instead if you are sure there can
	/// never be pairs with duplicate keys.
	pub fn new(inner: &'a [(K, V)]) -> Result<Self, (usize, usize)> {
		for (idx, (key, ..)) in inner.into_iter().enumerate() {
			for (other_idx, (other_key, ..)) in inner.into_iter().enumerate() {
				if key == other_key {
					return Err((idx, other_idx))
				}
			}
		}
		Ok(unsafe { Self::new_unchecked(inner) })
	}
}

/// Iterator over [`BorrowedMap`] pairs.
#[derive(Debug, Clone)]
pub struct BorrowedMapIter<'a, K, V> {
	inner: core::slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iterator for BorrowedMapIter<'a, K, V> {
	type Item = (&'a K, &'a V);
	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
			.map(move |(ref k, ref v)| (k, v))
	}
}

unsafe impl<'a, K: AsRef<CStr>, V: Section> SectionMap<'a> for BorrowedMap<'a, K, V> {
	type Name = K;
	type Section = V;

	type Iter = BorrowedMapIter<'a, K, V>;
	fn iter(&'a self) -> Self::Iter {
		BorrowedMapIter {
			inner: self.as_slice().iter()
		}
	}

	fn len(&self) -> usize {
		self.as_slice().len()
	}
}

/// Write the contents of an SMX file to a writer, with a specific
/// [`CompressionLevel`] and a [`SectionMap`].
pub fn write_to<'m_iter, 'm, E, M>(
	w: &mut impl WriteBytesExt,
	compression_level: CompressionLevel,
	sections: &'m M,
) -> IoResult<()>
where
	'm: 'm_iter,
	E: ByteOrder,
	M: SectionMap<'m_iter>,
{
	w.write_u32::<E>(FILE_MAGIC)?;
	w.write_u16::<E>(TARGET_VERSION)?;

	w.write_u8(if compression_level != CompressionLevel::NoCompression {
		1
	} else {
		0
	})?;

	let mut strings = CStrTable::new();

	struct SectionInfo {
		pub name_offset: usize,
		pub data_offset: usize,
		pub length: usize,
	}

	let (section_infos, sec_data) = {
		let mut section_infos = Vec::new();
		let mut smx_data = Vec::new();
		for (name, section) in sections.iter() {
			let data_offset = smx_data.len();
			section.write_to(&mut smx_data);
			let length = smx_data.len() - data_offset;

			section_infos.push(SectionInfo {
				name_offset: strings.insert(name),
				data_offset,
				length,
			});
		}
		(section_infos, smx_data)
	};

	debug_assert_eq!(section_infos.len(), sections.len());

	let disk_sec_data = match compression_level {
		CompressionLevel::NoCompression => Cow::Borrowed(&sec_data),
		_ => Cow::Owned(compress_to_vec_zlib(&sec_data, compression_level as _))
	};

	let string_tbl_offset = {
		SMX_HEADER_LEN +
			SMX_SECTION_INFO_LEN * sections.len()
	};
	let data_offset = string_tbl_offset + strings.len();

	w.write_u32::<E>((data_offset + disk_sec_data.len()) as _)?;
	w.write_u32::<E>((data_offset + sec_data.len()) as _)?;
	w.write_u8(sections.len() as _)?;
	w.write_u32::<E>(string_tbl_offset as _)?;
	w.write_u32::<E>(data_offset as _)?;

	for info in section_infos {
		w.write_u32::<E>(info.name_offset as _)?;
		w.write_u32::<E>((data_offset + info.data_offset) as _)?;
		w.write_u32::<E>(info.length as _)?;
	}

	strings.write_to(w)?;

	w.write_all(&disk_sec_data)?;

	Ok(())
}

/// Trait for objects which can represent SMX files.
pub trait WriteSmx {
	/// Type used for errors in the implementation's methods.
	type Error;

	/// Write a named section with data that was read from the SMX file.
	fn write_section(
		&mut self,
		name: CString, data: Vec<u8>
	) -> Result<(), Self::Error>;
}

impl<Name: From<CString> + Eq + Hash, Sect: From<Vec<u8>>>
	WriteSmx
	for HashMap<Name, Sect>
{
	type Error = never_say_never::Never;
	fn write_section(
		&mut self,
		name: CString, data: Vec<u8>
	) -> Result<(), Self::Error> {
		self.insert(name.into(), data.into());
		Ok(())
	}
}

/// Read a [`u32`] from a reader and treat is as the SMX magic number, inferring
/// the [`Endianness`] that was used to encode it.
pub fn infer_endianness<R: ReadBytesExt>(
	r: &mut R
) -> IoResult<Result<Endianness, [u8; core::mem::size_of::<u32>()]>> {
	let mut magic_buf = [0u8; core::mem::size_of::<u32>()];
	r.read_exact(&mut magic_buf)?;

	if FILE_MAGIC == u32::from_le_bytes(magic_buf) {
		Ok(Ok(Endianness::Little))
	} else if FILE_MAGIC == u32::from_be_bytes(magic_buf) {
		Ok(Ok(Endianness::Big))
	} else {
		Ok(Err(magic_buf))
	}
}

/// Read an SMX file, inferring its endianness from the [`u32`] magic number.
/// 
/// Section data is received with an object implementing the [`WriteSmx`] trait.
/// 
/// The endianness inference is done through the [`infer_endianness`] function.
pub fn read_from<S: WriteSmx>(
	r: &mut (impl ReadBytesExt + Seek),
	smx: &mut S,
) -> Result<Endianness, SmxError<S::Error>> {
	let endianness = infer_endianness(r)?.map_err(SmxError::Magic)?;
	match endianness {
		Endianness::Little => read_no_magic_from::<Le, S>(r, smx),
		Endianness::Big => read_no_magic_from::<Be, S>(r, smx)
	}?;
	Ok(endianness)
}

/// Read an SMX file _without_ also reading the [`u32`] magic number, requiring
/// an explicit endianness annotation.
/// 
/// Section data is received with an object implementing the [`WriteSmx`] trait.
/// 
/// This is called from [`read_from`]; in most cases it is a good idea to use
/// that function instead.
/// However, you may also use the [`infer_endianness`] function to do so
/// manually.
pub fn read_no_magic_from<E: ByteOrder, S: WriteSmx>(
	r: &mut (impl ReadBytesExt + Seek),
	smx: &mut S,
) -> Result<(), SmxError<S::Error>> {
	match r.read_u16::<E>()? {
		TARGET_VERSION => {}
		version => return Err(SmxError::Version(version))
	}

	enum CompressionType {
		None,
		Gz,
	}

	let compression = match r.read_u8()? {
		0 => CompressionType::None,
		1 => CompressionType::Gz,
		byte => return Err(SmxError::Compression(byte))
	};

	let disk_size = r.read_u32::<E>()?;
	let image_size = r.read_u32::<E>()?;

	let n_sections = r.read_u8()?;

	let string_tbl_offset = r.read_u32::<E>()?;
	let data_offset = r.read_u32::<E>()?;
	let pos_sections = r.stream_position()?;

	r.seek(SeekFrom::Start(string_tbl_offset as _))?;
	let strings = {
		let mut blob = Vec::new();
		blob.resize((data_offset - string_tbl_offset) as _, 0);
		r.read_exact(&mut blob)?;
		CStrTable::from_blob(blob)
	};

	let mut r = {
		let data = match compression {
			CompressionType::None => MurData::Uncomp,
			CompressionType::Gz => {
				r.seek(SeekFrom::Start(data_offset as _))?;

				let mut compressed = Vec::new();
				r.read_to_end(&mut compressed)?;
				if compressed.len() != (disk_size - data_offset) as _ {
					return Err(SmxError::NotAtDiskSize(compressed.len()))
				}

				MurData::Gz(Cursor::new(decompress_to_vec_zlib(&compressed)?))
			}
		};

		MaybeUncompressedReader {
			uncomp: r,
			data,
			data_offset: data_offset as _,
		}
	};

	match r.seek(SeekFrom::End(0))? {
		actual if actual == image_size as _ => {}
		actual => return Err(SmxError::NotAtImageSize {
			declared: image_size,
			actual,
		})
	}

	r.seek(SeekFrom::Start(pos_sections))?;
	for section in 0..n_sections {
		let name_offset = r.read_u32::<E>()?;
		let Some(name) = strings.get_c_string(name_offset as _) else {
			return Err(SmxError::SectionNameOffset {
				section,
				name_offset,
				string_table_size: strings.len()
			})
		};

		let data_offset = r.read_u32::<E>()?;
		let data_size = r.read_u32::<E>()?;

		let pos_last = r.stream_position()?;
		r.seek(SeekFrom::Start(data_offset as _))?;
		let data = {
			let mut buffer = Vec::new();
			buffer.resize(data_size as _, 0);
			r.read_exact(&mut buffer)?;
			buffer
		};
		r.seek(SeekFrom::Start(pos_last))?;

		smx.write_section(name, data).map_err(SmxError::Writer)?;
	}

	Ok(())
}

/// Structure for an error that has occurred while reading an SMX file.
#[derive(Debug)]
pub enum SmxError<E> {
	/// I/O error.
	Io(IoError),
	/// Writer-indicated error.
	Writer(E),
	/// Invalid SMX magic number.
	Magic([u8; core::mem::size_of::<u32>()]),
	/// Unsupported SMX version.
	Version(u16),
	/// Unsupported SMX compression.
	Compression(u8),
	/// SMX data decompression error.
	Decompress(DecompressError),
	/// Compressed SMX data was not at the size indicated in the header.
	NotAtDiskSize(usize),
	/// Uncompressed SMX data was not at the size indicated in the header.
	NotAtImageSize {
		declared: u32,
		actual: u64,
	},
	/// A section's offset into the SMX file's string table was invalid.
	SectionNameOffset {
		section: u8,
		name_offset: u32,
		string_table_size: usize,
	},
}

impl<E: fmt::Display> fmt::Display for SmxError<E> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Io(e) => write!(f, "I/O error: {e}"),
			Self::Writer(e) => write!(f, "writer-indicated error: {e}"),
			Self::Decompress(e) => write!(f, "decompression error: {e}"),
			Self::Magic([a, b, c, d]) => {
				write!(f, "wrong magic number: {a:02x} {b:02x} {c:02x} {d:02x}")
			},
			Self::Version(version) =>
				write!(f, "unsupported version: 0x{version:04x}"),
			Self::Compression(byte) =>
				write!(f, "unsupported compression type: 0x{byte:02x}"),
			Self::NotAtDiskSize(size) =>
				write!(f, "compressed payload is not at disk size 0x{size:08x}"),
			Self::NotAtImageSize { declared, actual } => {
				write!(
					f,
					concat!(
						"whole file is not at declared image size ",
						"(declared 0x{:08x}, actual 0x{:08x})"
					),
					declared,
					actual
				)
			}
			Self::SectionNameOffset {
				section,
				name_offset,
				string_table_size
			} => {
				write!(
					f,
					concat!(
						"section #{} has invalid offset 0x{:04x} ",
						"into string table of size 0x{:04x}"
					),
					section, name_offset, string_table_size,
				)
			}
		}
	}
}

impl<E: fmt::Debug + fmt::Display> Error for SmxError<E> {}

impl<E> From<IoError> for SmxError<E> {
	fn from(value: IoError) -> Self {
		Self::Io(value)
	}
}

impl<E> From<DecompressError> for SmxError<E> {
	fn from(value: DecompressError) -> Self {
		Self::Decompress(value)
	}
}

#[derive(Debug)]
enum MurData {
	Uncomp,
	Gz(Cursor<Vec<u8>>)
}

#[derive(Debug)]
struct MaybeUncompressedReader<R> {
	pub uncomp: R,
	pub data: MurData,
	pub data_offset: u64,
}

impl<R: Read + Seek> Read for MaybeUncompressedReader<R> {
	fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
		match self.data {
			MurData::Uncomp => self.uncomp.read(buf),
			MurData::Gz(ref mut cursor) => {
				let pos = self.uncomp.stream_position()?;
				if pos < self.data_offset {
					let to_read = ((self.data_offset - pos) as usize)
						.min(buf.len());
					self.uncomp.read(&mut buf[0..to_read])
				} else {
					cursor.read(buf)
				}
			}
		}
	}
}

impl<R: Seek> Seek for MaybeUncompressedReader<R> {
	fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
		match self.data {
			MurData::Uncomp => self.uncomp.seek(pos),
			MurData::Gz(ref mut cursor) => match pos {
				SeekFrom::End(..) => {
					let pos = cursor.seek(pos)?;
					Ok(self.data_offset + pos)
				}
				pos => {
					let pos = self.uncomp.seek(pos)?;
					if pos >= self.data_offset {
						cursor.set_position(pos - self.data_offset);
					}
					Ok(pos)
				}
			}
		}
	}
}

/// Endianness of an SMX file.
/// 
/// This is the result of [`infer_endianness`] and [`read_from`].
/// Read the documentation for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
	/// File is encoded in little-endian.
	Little,
	/// File is encoded in big-endian.
	Big,
}
