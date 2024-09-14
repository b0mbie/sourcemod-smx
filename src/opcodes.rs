use crate::vm_types::{
	Cell,
	read_cell,
	write_cell
};

use byteorder::{
	ReadBytesExt,
	WriteBytesExt
};
use std::io::{
	Error as IoError,
	ErrorKind as IoErrorKind,
	Result as IoResult
};

/// Enumeration of every possible SourcePawn instruction.
/// 
/// This type is generated automatically by a script.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum Instruction {
	None,
	LoadPri {
		offset: Cell,
	},
	LoadAlt {
		offset: Cell,
	},
	LoadSPri {
		offset: Cell,
	},
	LoadSAlt {
		offset: Cell,
	},
	LrefSPri {
		offset: Cell,
	},
	LrefSAlt {
		offset: Cell,
	},
	LoadI,
	LodbI {
		width: Cell,
	},
	ConstPri {
		value: Cell,
	},
	ConstAlt {
		value: Cell,
	},
	AddrPri {
		offset: Cell,
	},
	AddrAlt {
		offset: Cell,
	},
	StorPri {
		offset: Cell,
	},
	StorAlt {
		offset: Cell,
	},
	StorSPri {
		offset: Cell,
	},
	StorSAlt {
		offset: Cell,
	},
	SrefSPri {
		offset: Cell,
	},
	SrefSAlt {
		offset: Cell,
	},
	StorI,
	StrbI {
		width: Cell,
	},
	Lidx,
	Idxaddr,
	MovePri,
	MoveAlt,
	Xchg,
	PushPri,
	PushAlt,
	PushC {
		const_1: Cell,
	},
	Push {
		addr_1: Cell,
	},
	PushS {
		stack_1: Cell,
	},
	PopPri,
	PopAlt,
	Stack {
		const_1: Cell,
	},
	Heap {
		const_1: Cell,
	},
	Proc,
	Retn,
	Call {
		func_1: Cell,
	},
	Jump {
		jump_1: Cell,
	},
	Jzer {
		jump_1: Cell,
	},
	Jnz {
		jump_1: Cell,
	},
	Jeq {
		jump_1: Cell,
	},
	Jneq {
		jump_1: Cell,
	},
	Jsless {
		jump_1: Cell,
	},
	Jsleq {
		jump_1: Cell,
	},
	Jsgrtr {
		jump_1: Cell,
	},
	Jsgeq {
		jump_1: Cell,
	},
	Shl,
	Shr,
	Sshr,
	ShlCPri {
		const_1: Cell,
	},
	ShlCAlt {
		const_1: Cell,
	},
	Smul,
	Sdiv,
	SdivAlt,
	Add,
	Sub,
	SubAlt,
	And,
	Or,
	Xor,
	Not,
	Neg,
	Invert,
	AddC {
		const_1: Cell,
	},
	SmulC {
		const_1: Cell,
	},
	ZeroPri,
	ZeroAlt,
	Zero {
		addr_1: Cell,
	},
	ZeroS {
		stack_1: Cell,
	},
	Eq,
	Neq,
	Sless,
	Sleq,
	Sgrtr,
	Sgeq,
	EqCPri {
		const_1: Cell,
	},
	EqCAlt {
		const_1: Cell,
	},
	IncPri,
	IncAlt,
	Inc {
		addr_1: Cell,
	},
	IncS {
		stack_1: Cell,
	},
	IncI,
	DecPri,
	DecAlt,
	Dec {
		addr_1: Cell,
	},
	DecS {
		stack_1: Cell,
	},
	DecI,
	Movs {
		const_1: Cell,
	},
	Fill {
		const_1: Cell,
	},
	Halt {
		const_1: Cell,
	},
	Bounds {
		const_1: Cell,
	},
	SysreqC {
		native_1: Cell,
	},
	Switch {
		jump_1: Cell,
	},
	Casetbl {
		const_1: Cell,
		jump_1: Cell,
	},
	SwapPri,
	SwapAlt,
	PushAdr {
		stack_1: Cell,
	},
	Nop,
	SysreqN {
		native: Cell,
		n_args: Cell,
	},
	Break,
	Push2C {
		const_1: Cell,
		const_2: Cell,
	},
	Push2 {
		addr_1: Cell,
		addr_2: Cell,
	},
	Push2S {
		stack_1: Cell,
		stack_2: Cell,
	},
	Push2Adr {
		stack_1: Cell,
		stack_2: Cell,
	},
	Push3C {
		const_1: Cell,
		const_2: Cell,
		const_3: Cell,
	},
	Push3 {
		addr_1: Cell,
		addr_2: Cell,
		addr_3: Cell,
	},
	Push3S {
		stack_1: Cell,
		stack_2: Cell,
		stack_3: Cell,
	},
	Push3Adr {
		stack_1: Cell,
		stack_2: Cell,
		stack_3: Cell,
	},
	Push4C {
		const_1: Cell,
		const_2: Cell,
		const_3: Cell,
		const_4: Cell,
	},
	Push4 {
		addr_1: Cell,
		addr_2: Cell,
		addr_3: Cell,
		addr_4: Cell,
	},
	Push4S {
		stack_1: Cell,
		stack_2: Cell,
		stack_3: Cell,
		stack_4: Cell,
	},
	Push4Adr {
		stack_1: Cell,
		stack_2: Cell,
		stack_3: Cell,
		stack_4: Cell,
	},
	Push5C {
		const_1: Cell,
		const_2: Cell,
		const_3: Cell,
		const_4: Cell,
		const_5: Cell,
	},
	Push5 {
		addr_1: Cell,
		addr_2: Cell,
		addr_3: Cell,
		addr_4: Cell,
		addr_5: Cell,
	},
	Push5S {
		stack_1: Cell,
		stack_2: Cell,
		stack_3: Cell,
		stack_4: Cell,
		stack_5: Cell,
	},
	Push5Adr {
		stack_1: Cell,
		stack_2: Cell,
		stack_3: Cell,
		stack_4: Cell,
		stack_5: Cell,
	},
	LoadBoth {
		addr_1: Cell,
		addr_2: Cell,
	},
	LoadSBoth {
		stack_1: Cell,
		stack_2: Cell,
	},
	Const {
		addr_1: Cell,
		const_1: Cell,
	},
	ConstS {
		stack_1: Cell,
		const_1: Cell,
	},
	TrackerPushC {
		const_1: Cell,
	},
	TrackerPopSetheap,
	Genarray {
		const_1: Cell,
	},
	GenarrayZ {
		const_1: Cell,
	},
	StradjustPri,
	Endproc,
	InitarrayPri {
		addr_1: Cell,
		const_1: Cell,
		const_2: Cell,
		const_3: Cell,
		const_4: Cell,
	},
	InitarrayAlt {
		addr_1: Cell,
		const_1: Cell,
		const_2: Cell,
		const_3: Cell,
		const_4: Cell,
	},
	HeapSave,
	HeapRestore,
	Fabs,
	Float,
	Floatadd,
	Floatsub,
	Floatmul,
	Floatdiv,
	RndToNearest,
	RndToFloor,
	RndToCeil,
	RndToZero,
	Floatcmp,
	FloatGt,
	FloatGe,
	FloatLt,
	FloatLe,
	FloatNe,
	FloatEq,
	FloatNot,
}

impl Instruction {
	pub fn read_from(r: &mut impl ReadBytesExt) -> IoResult<Self> {
		match read_cell(r)? {
			0 => Ok(Self::None),
			1 => {
				let offset = read_cell(r)?;
				Ok(Self::LoadPri {
					offset,
				})
			}
			2 => {
				let offset = read_cell(r)?;
				Ok(Self::LoadAlt {
					offset,
				})
			}
			3 => {
				let offset = read_cell(r)?;
				Ok(Self::LoadSPri {
					offset,
				})
			}
			4 => {
				let offset = read_cell(r)?;
				Ok(Self::LoadSAlt {
					offset,
				})
			}
			7 => {
				let offset = read_cell(r)?;
				Ok(Self::LrefSPri {
					offset,
				})
			}
			8 => {
				let offset = read_cell(r)?;
				Ok(Self::LrefSAlt {
					offset,
				})
			}
			9 => Ok(Self::LoadI),
			10 => {
				let width = read_cell(r)?;
				Ok(Self::LodbI {
					width,
				})
			}
			11 => {
				let value = read_cell(r)?;
				Ok(Self::ConstPri {
					value,
				})
			}
			12 => {
				let value = read_cell(r)?;
				Ok(Self::ConstAlt {
					value,
				})
			}
			13 => {
				let offset = read_cell(r)?;
				Ok(Self::AddrPri {
					offset,
				})
			}
			14 => {
				let offset = read_cell(r)?;
				Ok(Self::AddrAlt {
					offset,
				})
			}
			15 => {
				let offset = read_cell(r)?;
				Ok(Self::StorPri {
					offset,
				})
			}
			16 => {
				let offset = read_cell(r)?;
				Ok(Self::StorAlt {
					offset,
				})
			}
			17 => {
				let offset = read_cell(r)?;
				Ok(Self::StorSPri {
					offset,
				})
			}
			18 => {
				let offset = read_cell(r)?;
				Ok(Self::StorSAlt {
					offset,
				})
			}
			21 => {
				let offset = read_cell(r)?;
				Ok(Self::SrefSPri {
					offset,
				})
			}
			22 => {
				let offset = read_cell(r)?;
				Ok(Self::SrefSAlt {
					offset,
				})
			}
			23 => Ok(Self::StorI),
			24 => {
				let width = read_cell(r)?;
				Ok(Self::StrbI {
					width,
				})
			}
			25 => Ok(Self::Lidx),
			27 => Ok(Self::Idxaddr),
			33 => Ok(Self::MovePri),
			34 => Ok(Self::MoveAlt),
			35 => Ok(Self::Xchg),
			36 => Ok(Self::PushPri),
			37 => Ok(Self::PushAlt),
			39 => {
				let const_1 = read_cell(r)?;
				Ok(Self::PushC {
					const_1,
				})
			}
			40 => {
				let addr_1 = read_cell(r)?;
				Ok(Self::Push {
					addr_1,
				})
			}
			41 => {
				let stack_1 = read_cell(r)?;
				Ok(Self::PushS {
					stack_1,
				})
			}
			42 => Ok(Self::PopPri),
			43 => Ok(Self::PopAlt),
			44 => {
				let const_1 = read_cell(r)?;
				Ok(Self::Stack {
					const_1,
				})
			}
			45 => {
				let const_1 = read_cell(r)?;
				Ok(Self::Heap {
					const_1,
				})
			}
			46 => Ok(Self::Proc),
			48 => Ok(Self::Retn),
			49 => {
				let func_1 = read_cell(r)?;
				Ok(Self::Call {
					func_1,
				})
			}
			51 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Jump {
					jump_1,
				})
			}
			53 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Jzer {
					jump_1,
				})
			}
			54 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Jnz {
					jump_1,
				})
			}
			55 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Jeq {
					jump_1,
				})
			}
			56 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Jneq {
					jump_1,
				})
			}
			61 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Jsless {
					jump_1,
				})
			}
			62 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Jsleq {
					jump_1,
				})
			}
			63 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Jsgrtr {
					jump_1,
				})
			}
			64 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Jsgeq {
					jump_1,
				})
			}
			65 => Ok(Self::Shl),
			66 => Ok(Self::Shr),
			67 => Ok(Self::Sshr),
			68 => {
				let const_1 = read_cell(r)?;
				Ok(Self::ShlCPri {
					const_1,
				})
			}
			69 => {
				let const_1 = read_cell(r)?;
				Ok(Self::ShlCAlt {
					const_1,
				})
			}
			72 => Ok(Self::Smul),
			73 => Ok(Self::Sdiv),
			74 => Ok(Self::SdivAlt),
			78 => Ok(Self::Add),
			79 => Ok(Self::Sub),
			80 => Ok(Self::SubAlt),
			81 => Ok(Self::And),
			82 => Ok(Self::Or),
			83 => Ok(Self::Xor),
			84 => Ok(Self::Not),
			85 => Ok(Self::Neg),
			86 => Ok(Self::Invert),
			87 => {
				let const_1 = read_cell(r)?;
				Ok(Self::AddC {
					const_1,
				})
			}
			88 => {
				let const_1 = read_cell(r)?;
				Ok(Self::SmulC {
					const_1,
				})
			}
			89 => Ok(Self::ZeroPri),
			90 => Ok(Self::ZeroAlt),
			91 => {
				let addr_1 = read_cell(r)?;
				Ok(Self::Zero {
					addr_1,
				})
			}
			92 => {
				let stack_1 = read_cell(r)?;
				Ok(Self::ZeroS {
					stack_1,
				})
			}
			95 => Ok(Self::Eq),
			96 => Ok(Self::Neq),
			101 => Ok(Self::Sless),
			102 => Ok(Self::Sleq),
			103 => Ok(Self::Sgrtr),
			104 => Ok(Self::Sgeq),
			105 => {
				let const_1 = read_cell(r)?;
				Ok(Self::EqCPri {
					const_1,
				})
			}
			106 => {
				let const_1 = read_cell(r)?;
				Ok(Self::EqCAlt {
					const_1,
				})
			}
			107 => Ok(Self::IncPri),
			108 => Ok(Self::IncAlt),
			109 => {
				let addr_1 = read_cell(r)?;
				Ok(Self::Inc {
					addr_1,
				})
			}
			110 => {
				let stack_1 = read_cell(r)?;
				Ok(Self::IncS {
					stack_1,
				})
			}
			111 => Ok(Self::IncI),
			112 => Ok(Self::DecPri),
			113 => Ok(Self::DecAlt),
			114 => {
				let addr_1 = read_cell(r)?;
				Ok(Self::Dec {
					addr_1,
				})
			}
			115 => {
				let stack_1 = read_cell(r)?;
				Ok(Self::DecS {
					stack_1,
				})
			}
			116 => Ok(Self::DecI),
			117 => {
				let const_1 = read_cell(r)?;
				Ok(Self::Movs {
					const_1,
				})
			}
			119 => {
				let const_1 = read_cell(r)?;
				Ok(Self::Fill {
					const_1,
				})
			}
			120 => {
				let const_1 = read_cell(r)?;
				Ok(Self::Halt {
					const_1,
				})
			}
			121 => {
				let const_1 = read_cell(r)?;
				Ok(Self::Bounds {
					const_1,
				})
			}
			123 => {
				let native_1 = read_cell(r)?;
				Ok(Self::SysreqC {
					native_1,
				})
			}
			129 => {
				let jump_1 = read_cell(r)?;
				Ok(Self::Switch {
					jump_1,
				})
			}
			130 => {
				let const_1 = read_cell(r)?;
				let jump_1 = read_cell(r)?;
				Ok(Self::Casetbl {
					const_1,
					jump_1,
				})
			}
			131 => Ok(Self::SwapPri),
			132 => Ok(Self::SwapAlt),
			133 => {
				let stack_1 = read_cell(r)?;
				Ok(Self::PushAdr {
					stack_1,
				})
			}
			134 => Ok(Self::Nop),
			135 => {
				let native = read_cell(r)?;
				let n_args = read_cell(r)?;
				Ok(Self::SysreqN {
					native,
					n_args,
				})
			}
			137 => Ok(Self::Break),
			138 => {
				let const_1 = read_cell(r)?;
				let const_2 = read_cell(r)?;
				Ok(Self::Push2C {
					const_1,
					const_2,
				})
			}
			139 => {
				let addr_1 = read_cell(r)?;
				let addr_2 = read_cell(r)?;
				Ok(Self::Push2 {
					addr_1,
					addr_2,
				})
			}
			140 => {
				let stack_1 = read_cell(r)?;
				let stack_2 = read_cell(r)?;
				Ok(Self::Push2S {
					stack_1,
					stack_2,
				})
			}
			141 => {
				let stack_1 = read_cell(r)?;
				let stack_2 = read_cell(r)?;
				Ok(Self::Push2Adr {
					stack_1,
					stack_2,
				})
			}
			142 => {
				let const_1 = read_cell(r)?;
				let const_2 = read_cell(r)?;
				let const_3 = read_cell(r)?;
				Ok(Self::Push3C {
					const_1,
					const_2,
					const_3,
				})
			}
			143 => {
				let addr_1 = read_cell(r)?;
				let addr_2 = read_cell(r)?;
				let addr_3 = read_cell(r)?;
				Ok(Self::Push3 {
					addr_1,
					addr_2,
					addr_3,
				})
			}
			144 => {
				let stack_1 = read_cell(r)?;
				let stack_2 = read_cell(r)?;
				let stack_3 = read_cell(r)?;
				Ok(Self::Push3S {
					stack_1,
					stack_2,
					stack_3,
				})
			}
			145 => {
				let stack_1 = read_cell(r)?;
				let stack_2 = read_cell(r)?;
				let stack_3 = read_cell(r)?;
				Ok(Self::Push3Adr {
					stack_1,
					stack_2,
					stack_3,
				})
			}
			146 => {
				let const_1 = read_cell(r)?;
				let const_2 = read_cell(r)?;
				let const_3 = read_cell(r)?;
				let const_4 = read_cell(r)?;
				Ok(Self::Push4C {
					const_1,
					const_2,
					const_3,
					const_4,
				})
			}
			147 => {
				let addr_1 = read_cell(r)?;
				let addr_2 = read_cell(r)?;
				let addr_3 = read_cell(r)?;
				let addr_4 = read_cell(r)?;
				Ok(Self::Push4 {
					addr_1,
					addr_2,
					addr_3,
					addr_4,
				})
			}
			148 => {
				let stack_1 = read_cell(r)?;
				let stack_2 = read_cell(r)?;
				let stack_3 = read_cell(r)?;
				let stack_4 = read_cell(r)?;
				Ok(Self::Push4S {
					stack_1,
					stack_2,
					stack_3,
					stack_4,
				})
			}
			149 => {
				let stack_1 = read_cell(r)?;
				let stack_2 = read_cell(r)?;
				let stack_3 = read_cell(r)?;
				let stack_4 = read_cell(r)?;
				Ok(Self::Push4Adr {
					stack_1,
					stack_2,
					stack_3,
					stack_4,
				})
			}
			150 => {
				let const_1 = read_cell(r)?;
				let const_2 = read_cell(r)?;
				let const_3 = read_cell(r)?;
				let const_4 = read_cell(r)?;
				let const_5 = read_cell(r)?;
				Ok(Self::Push5C {
					const_1,
					const_2,
					const_3,
					const_4,
					const_5,
				})
			}
			151 => {
				let addr_1 = read_cell(r)?;
				let addr_2 = read_cell(r)?;
				let addr_3 = read_cell(r)?;
				let addr_4 = read_cell(r)?;
				let addr_5 = read_cell(r)?;
				Ok(Self::Push5 {
					addr_1,
					addr_2,
					addr_3,
					addr_4,
					addr_5,
				})
			}
			152 => {
				let stack_1 = read_cell(r)?;
				let stack_2 = read_cell(r)?;
				let stack_3 = read_cell(r)?;
				let stack_4 = read_cell(r)?;
				let stack_5 = read_cell(r)?;
				Ok(Self::Push5S {
					stack_1,
					stack_2,
					stack_3,
					stack_4,
					stack_5,
				})
			}
			153 => {
				let stack_1 = read_cell(r)?;
				let stack_2 = read_cell(r)?;
				let stack_3 = read_cell(r)?;
				let stack_4 = read_cell(r)?;
				let stack_5 = read_cell(r)?;
				Ok(Self::Push5Adr {
					stack_1,
					stack_2,
					stack_3,
					stack_4,
					stack_5,
				})
			}
			154 => {
				let addr_1 = read_cell(r)?;
				let addr_2 = read_cell(r)?;
				Ok(Self::LoadBoth {
					addr_1,
					addr_2,
				})
			}
			155 => {
				let stack_1 = read_cell(r)?;
				let stack_2 = read_cell(r)?;
				Ok(Self::LoadSBoth {
					stack_1,
					stack_2,
				})
			}
			156 => {
				let addr_1 = read_cell(r)?;
				let const_1 = read_cell(r)?;
				Ok(Self::Const {
					addr_1,
					const_1,
				})
			}
			157 => {
				let stack_1 = read_cell(r)?;
				let const_1 = read_cell(r)?;
				Ok(Self::ConstS {
					stack_1,
					const_1,
				})
			}
			160 => {
				let const_1 = read_cell(r)?;
				Ok(Self::TrackerPushC {
					const_1,
				})
			}
			161 => Ok(Self::TrackerPopSetheap),
			162 => {
				let const_1 = read_cell(r)?;
				Ok(Self::Genarray {
					const_1,
				})
			}
			163 => {
				let const_1 = read_cell(r)?;
				Ok(Self::GenarrayZ {
					const_1,
				})
			}
			164 => Ok(Self::StradjustPri),
			166 => Ok(Self::Endproc),
			169 => {
				let addr_1 = read_cell(r)?;
				let const_1 = read_cell(r)?;
				let const_2 = read_cell(r)?;
				let const_3 = read_cell(r)?;
				let const_4 = read_cell(r)?;
				Ok(Self::InitarrayPri {
					addr_1,
					const_1,
					const_2,
					const_3,
					const_4,
				})
			}
			170 => {
				let addr_1 = read_cell(r)?;
				let const_1 = read_cell(r)?;
				let const_2 = read_cell(r)?;
				let const_3 = read_cell(r)?;
				let const_4 = read_cell(r)?;
				Ok(Self::InitarrayAlt {
					addr_1,
					const_1,
					const_2,
					const_3,
					const_4,
				})
			}
			171 => Ok(Self::HeapSave),
			172 => Ok(Self::HeapRestore),
			174 => Ok(Self::Fabs),
			175 => Ok(Self::Float),
			176 => Ok(Self::Floatadd),
			177 => Ok(Self::Floatsub),
			178 => Ok(Self::Floatmul),
			179 => Ok(Self::Floatdiv),
			180 => Ok(Self::RndToNearest),
			181 => Ok(Self::RndToFloor),
			182 => Ok(Self::RndToCeil),
			183 => Ok(Self::RndToZero),
			184 => Ok(Self::Floatcmp),
			185 => Ok(Self::FloatGt),
			186 => Ok(Self::FloatGe),
			187 => Ok(Self::FloatLt),
			188 => Ok(Self::FloatLe),
			189 => Ok(Self::FloatNe),
			190 => Ok(Self::FloatEq),
			191 => Ok(Self::FloatNot),
			opcode => Err(IoError::new(
				IoErrorKind::InvalidData, format!("invalid opcode: {opcode:?}")
			))
		}
	}

	pub fn write_to(&self, w: &mut impl WriteBytesExt) -> IoResult<()> {
		match self {
			Self::None => write_cell(w, 0),
			Self::LoadPri { offset, } => {
				write_cell(w, 1)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::LoadAlt { offset, } => {
				write_cell(w, 2)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::LoadSPri { offset, } => {
				write_cell(w, 3)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::LoadSAlt { offset, } => {
				write_cell(w, 4)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::LrefSPri { offset, } => {
				write_cell(w, 7)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::LrefSAlt { offset, } => {
				write_cell(w, 8)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::LoadI => write_cell(w, 9),
			Self::LodbI { width, } => {
				write_cell(w, 10)?;
				write_cell(w, *width)?;
				Ok(())
			}
			Self::ConstPri { value, } => {
				write_cell(w, 11)?;
				write_cell(w, *value)?;
				Ok(())
			}
			Self::ConstAlt { value, } => {
				write_cell(w, 12)?;
				write_cell(w, *value)?;
				Ok(())
			}
			Self::AddrPri { offset, } => {
				write_cell(w, 13)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::AddrAlt { offset, } => {
				write_cell(w, 14)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::StorPri { offset, } => {
				write_cell(w, 15)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::StorAlt { offset, } => {
				write_cell(w, 16)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::StorSPri { offset, } => {
				write_cell(w, 17)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::StorSAlt { offset, } => {
				write_cell(w, 18)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::SrefSPri { offset, } => {
				write_cell(w, 21)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::SrefSAlt { offset, } => {
				write_cell(w, 22)?;
				write_cell(w, *offset)?;
				Ok(())
			}
			Self::StorI => write_cell(w, 23),
			Self::StrbI { width, } => {
				write_cell(w, 24)?;
				write_cell(w, *width)?;
				Ok(())
			}
			Self::Lidx => write_cell(w, 25),
			Self::Idxaddr => write_cell(w, 27),
			Self::MovePri => write_cell(w, 33),
			Self::MoveAlt => write_cell(w, 34),
			Self::Xchg => write_cell(w, 35),
			Self::PushPri => write_cell(w, 36),
			Self::PushAlt => write_cell(w, 37),
			Self::PushC { const_1, } => {
				write_cell(w, 39)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::Push { addr_1, } => {
				write_cell(w, 40)?;
				write_cell(w, *addr_1)?;
				Ok(())
			}
			Self::PushS { stack_1, } => {
				write_cell(w, 41)?;
				write_cell(w, *stack_1)?;
				Ok(())
			}
			Self::PopPri => write_cell(w, 42),
			Self::PopAlt => write_cell(w, 43),
			Self::Stack { const_1, } => {
				write_cell(w, 44)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::Heap { const_1, } => {
				write_cell(w, 45)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::Proc => write_cell(w, 46),
			Self::Retn => write_cell(w, 48),
			Self::Call { func_1, } => {
				write_cell(w, 49)?;
				write_cell(w, *func_1)?;
				Ok(())
			}
			Self::Jump { jump_1, } => {
				write_cell(w, 51)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Jzer { jump_1, } => {
				write_cell(w, 53)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Jnz { jump_1, } => {
				write_cell(w, 54)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Jeq { jump_1, } => {
				write_cell(w, 55)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Jneq { jump_1, } => {
				write_cell(w, 56)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Jsless { jump_1, } => {
				write_cell(w, 61)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Jsleq { jump_1, } => {
				write_cell(w, 62)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Jsgrtr { jump_1, } => {
				write_cell(w, 63)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Jsgeq { jump_1, } => {
				write_cell(w, 64)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Shl => write_cell(w, 65),
			Self::Shr => write_cell(w, 66),
			Self::Sshr => write_cell(w, 67),
			Self::ShlCPri { const_1, } => {
				write_cell(w, 68)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::ShlCAlt { const_1, } => {
				write_cell(w, 69)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::Smul => write_cell(w, 72),
			Self::Sdiv => write_cell(w, 73),
			Self::SdivAlt => write_cell(w, 74),
			Self::Add => write_cell(w, 78),
			Self::Sub => write_cell(w, 79),
			Self::SubAlt => write_cell(w, 80),
			Self::And => write_cell(w, 81),
			Self::Or => write_cell(w, 82),
			Self::Xor => write_cell(w, 83),
			Self::Not => write_cell(w, 84),
			Self::Neg => write_cell(w, 85),
			Self::Invert => write_cell(w, 86),
			Self::AddC { const_1, } => {
				write_cell(w, 87)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::SmulC { const_1, } => {
				write_cell(w, 88)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::ZeroPri => write_cell(w, 89),
			Self::ZeroAlt => write_cell(w, 90),
			Self::Zero { addr_1, } => {
				write_cell(w, 91)?;
				write_cell(w, *addr_1)?;
				Ok(())
			}
			Self::ZeroS { stack_1, } => {
				write_cell(w, 92)?;
				write_cell(w, *stack_1)?;
				Ok(())
			}
			Self::Eq => write_cell(w, 95),
			Self::Neq => write_cell(w, 96),
			Self::Sless => write_cell(w, 101),
			Self::Sleq => write_cell(w, 102),
			Self::Sgrtr => write_cell(w, 103),
			Self::Sgeq => write_cell(w, 104),
			Self::EqCPri { const_1, } => {
				write_cell(w, 105)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::EqCAlt { const_1, } => {
				write_cell(w, 106)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::IncPri => write_cell(w, 107),
			Self::IncAlt => write_cell(w, 108),
			Self::Inc { addr_1, } => {
				write_cell(w, 109)?;
				write_cell(w, *addr_1)?;
				Ok(())
			}
			Self::IncS { stack_1, } => {
				write_cell(w, 110)?;
				write_cell(w, *stack_1)?;
				Ok(())
			}
			Self::IncI => write_cell(w, 111),
			Self::DecPri => write_cell(w, 112),
			Self::DecAlt => write_cell(w, 113),
			Self::Dec { addr_1, } => {
				write_cell(w, 114)?;
				write_cell(w, *addr_1)?;
				Ok(())
			}
			Self::DecS { stack_1, } => {
				write_cell(w, 115)?;
				write_cell(w, *stack_1)?;
				Ok(())
			}
			Self::DecI => write_cell(w, 116),
			Self::Movs { const_1, } => {
				write_cell(w, 117)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::Fill { const_1, } => {
				write_cell(w, 119)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::Halt { const_1, } => {
				write_cell(w, 120)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::Bounds { const_1, } => {
				write_cell(w, 121)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::SysreqC { native_1, } => {
				write_cell(w, 123)?;
				write_cell(w, *native_1)?;
				Ok(())
			}
			Self::Switch { jump_1, } => {
				write_cell(w, 129)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::Casetbl { const_1, jump_1, } => {
				write_cell(w, 130)?;
				write_cell(w, *const_1)?;
				write_cell(w, *jump_1)?;
				Ok(())
			}
			Self::SwapPri => write_cell(w, 131),
			Self::SwapAlt => write_cell(w, 132),
			Self::PushAdr { stack_1, } => {
				write_cell(w, 133)?;
				write_cell(w, *stack_1)?;
				Ok(())
			}
			Self::Nop => write_cell(w, 134),
			Self::SysreqN { native, n_args, } => {
				write_cell(w, 135)?;
				write_cell(w, *native)?;
				write_cell(w, *n_args)?;
				Ok(())
			}
			Self::Break => write_cell(w, 137),
			Self::Push2C { const_1, const_2, } => {
				write_cell(w, 138)?;
				write_cell(w, *const_1)?;
				write_cell(w, *const_2)?;
				Ok(())
			}
			Self::Push2 { addr_1, addr_2, } => {
				write_cell(w, 139)?;
				write_cell(w, *addr_1)?;
				write_cell(w, *addr_2)?;
				Ok(())
			}
			Self::Push2S { stack_1, stack_2, } => {
				write_cell(w, 140)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *stack_2)?;
				Ok(())
			}
			Self::Push2Adr { stack_1, stack_2, } => {
				write_cell(w, 141)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *stack_2)?;
				Ok(())
			}
			Self::Push3C { const_1, const_2, const_3, } => {
				write_cell(w, 142)?;
				write_cell(w, *const_1)?;
				write_cell(w, *const_2)?;
				write_cell(w, *const_3)?;
				Ok(())
			}
			Self::Push3 { addr_1, addr_2, addr_3, } => {
				write_cell(w, 143)?;
				write_cell(w, *addr_1)?;
				write_cell(w, *addr_2)?;
				write_cell(w, *addr_3)?;
				Ok(())
			}
			Self::Push3S { stack_1, stack_2, stack_3, } => {
				write_cell(w, 144)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *stack_2)?;
				write_cell(w, *stack_3)?;
				Ok(())
			}
			Self::Push3Adr { stack_1, stack_2, stack_3, } => {
				write_cell(w, 145)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *stack_2)?;
				write_cell(w, *stack_3)?;
				Ok(())
			}
			Self::Push4C { const_1, const_2, const_3, const_4, } => {
				write_cell(w, 146)?;
				write_cell(w, *const_1)?;
				write_cell(w, *const_2)?;
				write_cell(w, *const_3)?;
				write_cell(w, *const_4)?;
				Ok(())
			}
			Self::Push4 { addr_1, addr_2, addr_3, addr_4, } => {
				write_cell(w, 147)?;
				write_cell(w, *addr_1)?;
				write_cell(w, *addr_2)?;
				write_cell(w, *addr_3)?;
				write_cell(w, *addr_4)?;
				Ok(())
			}
			Self::Push4S { stack_1, stack_2, stack_3, stack_4, } => {
				write_cell(w, 148)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *stack_2)?;
				write_cell(w, *stack_3)?;
				write_cell(w, *stack_4)?;
				Ok(())
			}
			Self::Push4Adr { stack_1, stack_2, stack_3, stack_4, } => {
				write_cell(w, 149)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *stack_2)?;
				write_cell(w, *stack_3)?;
				write_cell(w, *stack_4)?;
				Ok(())
			}
			Self::Push5C { const_1, const_2, const_3, const_4, const_5, } => {
				write_cell(w, 150)?;
				write_cell(w, *const_1)?;
				write_cell(w, *const_2)?;
				write_cell(w, *const_3)?;
				write_cell(w, *const_4)?;
				write_cell(w, *const_5)?;
				Ok(())
			}
			Self::Push5 { addr_1, addr_2, addr_3, addr_4, addr_5, } => {
				write_cell(w, 151)?;
				write_cell(w, *addr_1)?;
				write_cell(w, *addr_2)?;
				write_cell(w, *addr_3)?;
				write_cell(w, *addr_4)?;
				write_cell(w, *addr_5)?;
				Ok(())
			}
			Self::Push5S { stack_1, stack_2, stack_3, stack_4, stack_5, } => {
				write_cell(w, 152)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *stack_2)?;
				write_cell(w, *stack_3)?;
				write_cell(w, *stack_4)?;
				write_cell(w, *stack_5)?;
				Ok(())
			}
			Self::Push5Adr { stack_1, stack_2, stack_3, stack_4, stack_5, } => {
				write_cell(w, 153)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *stack_2)?;
				write_cell(w, *stack_3)?;
				write_cell(w, *stack_4)?;
				write_cell(w, *stack_5)?;
				Ok(())
			}
			Self::LoadBoth { addr_1, addr_2, } => {
				write_cell(w, 154)?;
				write_cell(w, *addr_1)?;
				write_cell(w, *addr_2)?;
				Ok(())
			}
			Self::LoadSBoth { stack_1, stack_2, } => {
				write_cell(w, 155)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *stack_2)?;
				Ok(())
			}
			Self::Const { addr_1, const_1, } => {
				write_cell(w, 156)?;
				write_cell(w, *addr_1)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::ConstS { stack_1, const_1, } => {
				write_cell(w, 157)?;
				write_cell(w, *stack_1)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::TrackerPushC { const_1, } => {
				write_cell(w, 160)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::TrackerPopSetheap => write_cell(w, 161),
			Self::Genarray { const_1, } => {
				write_cell(w, 162)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::GenarrayZ { const_1, } => {
				write_cell(w, 163)?;
				write_cell(w, *const_1)?;
				Ok(())
			}
			Self::StradjustPri => write_cell(w, 164),
			Self::Endproc => write_cell(w, 166),
			Self::InitarrayPri { addr_1, const_1, const_2, const_3, const_4, } => {
				write_cell(w, 169)?;
				write_cell(w, *addr_1)?;
				write_cell(w, *const_1)?;
				write_cell(w, *const_2)?;
				write_cell(w, *const_3)?;
				write_cell(w, *const_4)?;
				Ok(())
			}
			Self::InitarrayAlt { addr_1, const_1, const_2, const_3, const_4, } => {
				write_cell(w, 170)?;
				write_cell(w, *addr_1)?;
				write_cell(w, *const_1)?;
				write_cell(w, *const_2)?;
				write_cell(w, *const_3)?;
				write_cell(w, *const_4)?;
				Ok(())
			}
			Self::HeapSave => write_cell(w, 171),
			Self::HeapRestore => write_cell(w, 172),
			Self::Fabs => write_cell(w, 174),
			Self::Float => write_cell(w, 175),
			Self::Floatadd => write_cell(w, 176),
			Self::Floatsub => write_cell(w, 177),
			Self::Floatmul => write_cell(w, 178),
			Self::Floatdiv => write_cell(w, 179),
			Self::RndToNearest => write_cell(w, 180),
			Self::RndToFloor => write_cell(w, 181),
			Self::RndToCeil => write_cell(w, 182),
			Self::RndToZero => write_cell(w, 183),
			Self::Floatcmp => write_cell(w, 184),
			Self::FloatGt => write_cell(w, 185),
			Self::FloatGe => write_cell(w, 186),
			Self::FloatLt => write_cell(w, 187),
			Self::FloatLe => write_cell(w, 188),
			Self::FloatNe => write_cell(w, 189),
			Self::FloatEq => write_cell(w, 190),
			Self::FloatNot => write_cell(w, 191),
		}
	}
}
