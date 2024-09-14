use byteorder::{
	ByteOrder,
	NativeEndian as Ne,
	ReadBytesExt,
	WriteBytesExt,
};
use std::{
	error::Error,
	ffi::{
		CStr, CString
	},
	fs::File,
	io::{
		Cursor,
		Result as IoResult, Seek,
	},
	mem::size_of
};
use sourcemod_smx::{
	CompressionLevel,
	Instruction,
	size_of,
	Smx,
	smx_table::CStrTable,
	vm_types::Cell
};

/// Targetting version 13, with feature flags in code headers.
pub const TARGET_CODE_VERSION: u8 = 13;

pub const TARGET_CELL_SIZE: u8 = size_of::<Cell>() as _;

pub const TARGET_FLAGS: u16 = 0;
/*
pub const TARGET_FEATURES: u32 =
	code_features::HEAP_SCOPES |
	code_features::NULL_FUNCTIONS |
	code_features::DIRECT_ARRAYS;
*/
pub const TARGET_FEATURES: u32 = 0;

pub mod code_flags {
	pub const DEBUG: u16 = 0x00000001;
}

pub mod code_features {
	/// Support `INIT_ARRAY` opcode, and require that multi-dimensional arrays
	/// use direct internal addressing.
	pub const DIRECT_ARRAYS: u32 = 1 << 1;

	/// Support `HEAP_SAVE` and `HEAP_RESTORE` opcodes.
	pub const HEAP_SCOPES: u32 = 1 << 2;

	/// Treat null (`0`) as an invalid function instead of `-1`.
	pub const NULL_FUNCTIONS: u32 = 1 << 3;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
	pub extra_memory: u32,
	pub bytes: Vec<u8>,
}

impl Data {
	const HEADER_LEN: usize = size_of!(u32 + u32 + u32);

	pub const fn new(extra_memory: u32) -> Self {
		Self {
			extra_memory,
			bytes: Vec::new(),
		}
	}

	pub fn from_vec<E: ByteOrder>(bytes: Vec<u8>) -> IoResult<Self> {
		let mut r = Cursor::new(bytes);
		let _data_size = r.read_u32::<E>()?;
		let total_mem_size = r.read_u32::<E>()?;
		let mut bytes = r.into_inner();
		bytes.drain(0..Self::HEADER_LEN);
		Ok(Self {
			extra_memory: total_mem_size - bytes.len() as u32,
			bytes,
		})
	}

	pub fn write_to<E: ByteOrder>(
		&self, w: &mut (impl WriteBytesExt + Seek)
	) -> IoResult<()> {
		w.write_u32::<E>(self.bytes.len() as _)?;
		w.write_u32::<E>((self.bytes.len() as u32) + self.extra_memory)?;
		w.write_u32::<E>(Self::HEADER_LEN as _)?;
		w.write_all(&self.bytes)
	}

	pub fn to_vec<E: ByteOrder>(&self) -> Vec<u8> {
		let mut buffer = Vec::new();
		let _ = buffer.write_u32::<E>(self.bytes.len() as _);
		let _ = buffer.write_u32::<E>(
			(self.bytes.len() as u32) + self.extra_memory
		);
		let _ = buffer.write_u32::<E>(Self::HEADER_LEN as _);
		buffer.extend_from_slice(&self.bytes);
		buffer
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	const CODE_HEADER_LEN: usize = size_of!(u32 + u8 + u8 + u16 + u32 + u32 + u32);

	let mut names = CStrTable::new();
	let on_plugin_start = names.insert(CStr::from_bytes_with_nul(b"OnPluginStart\0")?);
	let log_message = names.insert(CStr::from_bytes_with_nul(b"LogMessage\0")?);

	let mut smx = Smx::new();
	smx.sections.insert(CString::new(b".data")?, {
		let mut data = Data::new(0);
		data.bytes.extend_from_slice(b"I am a plugin from outer space\0");
		data.to_vec::<Ne>()
	});
	smx.sections.insert(CString::new(b".names")?, names.blob().clone());
	smx.sections.insert(CString::new(b".publics")?, {
		let mut section = Vec::new();
		section.write_u32::<Ne>((CODE_HEADER_LEN + 0) as _)?;
		section.write_u32::<Ne>(on_plugin_start as _)?;
		section
	});
	smx.sections.insert(CString::new(b".natives")?, {
		let mut section = Vec::new();
		section.write_u32::<Ne>(log_message as _)?;
		section
	});
	smx.sections.insert(CString::new(b".code")?, {
		let mut code = Vec::new();
		Instruction::Proc.write_to(&mut code)?;
		Instruction::Break.write_to(&mut code)?;
		Instruction::PushC { const_1: 0x00000000 }.write_to(&mut code)?;
		Instruction::SysreqN { native: 0, n_args: 0x1 }.write_to(&mut code)?;
		Instruction::ZeroPri.write_to(&mut code)?;
		Instruction::Retn.write_to(&mut code)?;
		Instruction::Endproc.write_to(&mut code)?;

		let mut section = Vec::new();
		section.write_u32::<Ne>((CODE_HEADER_LEN + code.len()) as _)?;
		section.write_u8(size_of::<Cell>() as _)?;
		section.write_u8(TARGET_CODE_VERSION)?;
		section.write_u16::<Ne>(TARGET_FLAGS)?;
		section.write_u32::<Ne>(0)?;
		section.write_u32::<Ne>(0)?;
		section.write_u32::<Ne>(TARGET_FEATURES)?;
		section.extend_from_slice(&code);
		section
	});

	smx.write_to::<Ne>(
		&mut File::create("examples/example_code.smx")?,
		CompressionLevel::DefaultCompression
	)?;

	Ok(())
}
