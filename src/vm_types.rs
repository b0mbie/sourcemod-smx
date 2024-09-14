//! SourcePawn VM types.

use byteorder::{
	NativeEndian as Ne,
	ReadBytesExt, WriteBytesExt,
};
use std::io::Result as IoResult;

/// `ucell_t`.
pub type Ucell = u32;

/// Read a [`Ucell`] from a reader.
pub fn read_ucell(reader: &mut impl ReadBytesExt) -> IoResult<Ucell> {
	reader.read_u32::<Ne>()
}

/// `cell_t`.
pub type Cell = i32;

/// Read a [`Cell`] from a reader.
pub fn read_cell(reader: &mut impl ReadBytesExt) -> IoResult<Cell> {
	reader.read_i32::<Ne>()
}

/// Write a [`Cell`] to a writer.
pub fn write_cell(writer: &mut impl WriteBytesExt, cell: Cell) -> IoResult<()> {
	writer.write_i32::<Ne>(cell)
}

/// `funcid_t`.
pub type FuncId = u32;

/// Read a [`FuncId`] from a reader.
pub fn read_func_id(reader: &mut impl ReadBytesExt) -> IoResult<FuncId> {
	reader.read_u32::<Ne>()
}

/// Maximum number of parameters for a SourcePawn function.
pub const MAX_PARAMS: usize = 32;

/// Maximum number of arguments for a native or SourcePawn function.
pub const MAX_ARGS: usize = 127;
