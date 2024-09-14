-- Script to parse the SourcePawn opcode list from stdin and generate a Rust
-- source file.
-- See also
-- <https://github.com/alliedmodders/sourcepawn/blob/master/include/smx/smx-v1-opcodes.h>.

--- @class Instruction: table<integer, string>
--- @field description? string

--- See `Interpreter::visit*` methods in
--- <https://github.com/alliedmodders/sourcepawn/blob/master/vm/interpreter.cpp>,
--- <https://github.com/alliedmodders/sourcepawn/blob/master/vm/pcode-reader.h>,
--- <https://github.com/peace-maker/sourcepawn-disassembler-js/blob/master/src/disassembly/v1disassembler.ts>.
--- @type table<string, Instruction>
local OPCODE_MAP = {
	NONE = {
		description = "`;`";
	};

	LOAD_PRI = {
		description = "`R[0] = R[offset]`";
		"offset";
	};
	LOAD_ALT = {
		description = "`R[1] = R[offset]`";
		"offset";
	};
	LOAD_S_PRI = {
		description = "`R[0] = Frame[offset]`";
		"offset";
	};
	LOAD_S_ALT = {
		description = "`R[1] = Frame[offset]`";
		"offset";
	};
	LREF_PRI = {};
	LREF_ALT = {};
	LREF_S_PRI = {
		description = "`R[0] = &Memory[offset]`";
		"offset";
	};
	LREF_S_ALT = {
		description = "`R[1] = &Memory[offset]`";
		"offset";
	};
	LOAD_I = {
		description = "`R[0] = Memory[R[0]]`";
	};
	LODB_I = {
		description = "`let value = Memory[R[0]]; R[0] = match width { 1 => value & 0xff, 2 => value & 0xffff, 4 => value, _ => panic!() };`";
		"width";
	};
	CONST_PRI = {
		description = "`R[0] = value;`";
		"value";
	};
	CONST_ALT = {
		description = "`R[1] = value;`";
		"value";
	};
	ADDR_PRI = {
		description = "`R[0] = &Frame[offset];`";
		"offset";
	};
	ADDR_ALT = {
		description = "`R[1] = &Frame[offset];`";
		"offset";
	};
	STOR_PRI = {
		description = "`Memory[offset] = R[0];`";
		"offset";
	};
	STOR_ALT = {
		description = "`Memory[offset] = R[1];`";
		"offset";
	};
	STOR_S_PRI = {
		description = "`Frame[offset] = R[0];`";
		"offset";
	};
	STOR_S_ALT = {
		description = "`Frame[offset] = R[1];`";
		"offset";
	};
	SREF_PRI = {};
	SREF_ALT = {};
	SREF_S_PRI = {
		description = "`Memory[offset] = R[0]`";
		"offset";
	};
	SREF_S_ALT = {
		description = "`Memory[offset] = R[1]`";
		"offset";
	};
	STOR_I = {
		description = "`R[1] = R[0]`";
	};
	STRB_I = {
		description = "`Memory[R[1]] = match width { 1 => R[0] & 0xff, 2 => R[0] & 0xffff, 4 => R[0], _ => panic!() };`";
		"width";
	};
	LIDX = {
		description = "``";
	};
	LIDX_B = {};
	IDXADDR = {
		description = "``";
	};
	IDXADDR_B = {};
	ALIGN_PRI = {};
	ALIGN_ALT = {};
	LCTRL = {};
	SCTRL = {};
	MOVE_PRI = {
		description = "``";
	};
	MOVE_ALT = {
		description = "``";
	};
	XCHG = {
		description = "``";
	};
	PUSH_PRI = {
		description = "``";
	};
	PUSH_ALT = {
		description = "``";
	};
	PUSH_R = {};
	PUSH_C = {
		description = "``";
		"const_1";
	};
	PUSH = {
		description = "``";
		"addr_1";
	};
	PUSH_S = {
		description = "``";
		"stack_1";
	};
	POP_PRI = {
		description = "``";
	};
	POP_ALT = {
		description = "``";
	};
	STACK = {
		description = "``";
		"const_1";
	};
	HEAP = {
		description = "``";
		"const_1";
	};
	PROC = {
		description = "Indicates the start of a function (or \"procedure\").";
	};
	RET = {};
	RETN = {
		description = "``";
	};
	CALL = {
		description = "``";
		"func_1";
	};
	CALL_PRI = {};
	JUMP = {
		description = "``";
		"jump_1";
	};
	JREL = {};
	JZER = {
		description = "``";
		"jump_1";
	};
	JNZ = {
		description = "``";
		"jump_1";
	};
	JEQ = {
		description = "``";
		"jump_1";
	};
	JNEQ = {
		description = "``";
		"jump_1";
	};
	JLESS = {};
	JLEQ = {};
	JGRTR = {};
	JGEQ = {};
	JSLESS = {
		description = "``";
		"jump_1";
	};
	JSLEQ = {
		description = "``";
		"jump_1";
	};
	JSGRTR = {
		description = "``";
		"jump_1";
	};
	JSGEQ = {
		description = "``";
		"jump_1";
	};
	SHL = {
		description = "``";
	};
	SHR = {
		description = "``";
	};
	SSHR = {
		description = "``";
	};
	SHL_C_PRI = {
		description = "``";
		"const_1";
	};
	SHL_C_ALT = {
		description = "``";
		"const_1";
	};
	SHR_C_PRI = {};
	SHR_C_ALT = {};
	SMUL = {
		description = "``";
	};
	SDIV = {
		description = "``";
	};
	SDIV_ALT = {
		description = "``";
	};
	UMUL = {};
	UDIV = {};
	UDIV_ALT = {};
	ADD = {
		description = "``";
	};
	SUB = {
		description = "``";
	};
	SUB_ALT = {
		description = "``";
	};
	AND = {
		description = "``";
	};
	OR = {
		description = "``";
	};
	XOR = {
		description = "``";
	};
	NOT = {
		description = "``";
	};
	NEG = {
		description = "``";
	};
	INVERT = {
		description = "``";
	};
	ADD_C = {
		description = "``";
		"const_1";
	};
	SMUL_C = {
		description = "``";
		"const_1";
	};
	ZERO_PRI = {
		description = "``";
	};
	ZERO_ALT = {
		description = "``";
	};
	ZERO = {
		description = "``";
		"addr_1";
	};
	ZERO_S = {
		description = "``";
		"stack_1";
	};
	SIGN_PRI = {};
	SIGN_ALT = {};
	EQ = {
		description = "``";
	};
	NEQ = {
		description = "``";
	};
	LESS = {};
	LEQ = {};
	GRTR = {};
	GEQ = {};
	SLESS = {
		description = "``";
	};
	SLEQ = {
		description = "``";
	};
	SGRTR = {
		description = "``";
	};
	SGEQ = {
		description = "``";
	};
	EQ_C_PRI = {
		description = "``";
		"const_1";
	};
	EQ_C_ALT = {
		description = "``";
		"const_1";
	};
	INC_PRI = {
		description = "``";
	};
	INC_ALT = {
		description = "``";
	};
	INC = {
		description = "``";
		"addr_1";
	};
	INC_S = {
		description = "``";
		"stack_1";
	};
	INC_I = {
		description = "``";
	};
	DEC_PRI = {
		description = "``";
	};
	DEC_ALT = {
		description = "``";
	};
	DEC = {
		description = "``";
		"addr_1";
	};
	DEC_S = {
		description = "``";
		"stack_1";
	};
	DEC_I = {
		description = "``";
	};
	MOVS = {
		description = "``";
		"const_1";
	};
	CMPS = {};
	FILL = {
		description = "``";
		"const_1";
	};
	HALT = {
		description = "``";
		"const_1";
	};
	BOUNDS = {
		description = "``";
		"const_1";
	};
	SYSREQ_PRI = {};
	SYSREQ_C = {
		description = "``";
		"native_1";
	};
	FILE = {};
	LINE = {};
	SYMBOL = {};
	SRANGE = {};
	JUMP_PRI = {};
	SWITCH = {
		description = "``";
		"jump_1";
	};
	CASETBL = {
		description = "``";
		"const_1", "jump_1";
	};
	SWAP_PRI = {
		description = "``";
	};
	SWAP_ALT = {
		description = "``";
	};
	PUSH_ADR = {
		description = "``";
		"stack_1";
	};
	NOP = {
		description = "`;`";
	};
	SYSREQ_N = {
		description = "Invoke native `native` with `n_args` arguments.";
		"native", "n_args";
	};
	SYMTAG = {};
	BREAK = {
		description = "Invoke a debug line break.";
	};
	PUSH2_C = {
		description = "``";
		"const_1", "const_2";
	};
	PUSH2 = {
		description = "``";
		"addr_1", "addr_2";
	};
	PUSH2_S = {
		description = "``";
		"stack_1", "stack_2";
	};
	PUSH2_ADR = {
		description = "``";
		"stack_1", "stack_2";
	};
	PUSH3_C = {
		description = "``";
		"const_1", "const_2", "const_3";
	};
	PUSH3 = {
		description = "``";
		"addr_1", "addr_2", "addr_3";
	};
	PUSH3_S = {
		description = "``";
		"stack_1", "stack_2", "stack_3";
	};
	PUSH3_ADR = {
		description = "``";
		"stack_1", "stack_2", "stack_3";
	};
	PUSH4_C = {
		description = "``";
		"const_1", "const_2", "const_3", "const_4";
	};
	PUSH4 = {
		description = "``";
		"addr_1", "addr_2", "addr_3", "addr_4";
	};
	PUSH4_S = {
		description = "``";
		"stack_1", "stack_2", "stack_3", "stack_4";
	};
	PUSH4_ADR = {
		description = "``";
		"stack_1", "stack_2", "stack_3", "stack_4";
	};
	PUSH5_C = {
		description = "``";
		"const_1", "const_2", "const_3", "const_4", "const_5";
	};
	PUSH5 = {
		description = "``";
		"addr_1", "addr_2", "addr_3", "addr_4", "addr_5";
	};
	PUSH5_S = {
		description = "``";
		"stack_1", "stack_2", "stack_3", "stack_4", "stack_5";
	};
	PUSH5_ADR = {
		description = "``";
		"stack_1", "stack_2", "stack_3", "stack_4", "stack_5";
	};
	LOAD_BOTH = {
		description = "``";
		"addr_1", "addr_2";
	};
	LOAD_S_BOTH = {
		description = "``";
		"stack_1", "stack_2";
	};
	CONST = {
		description = "``";
		"addr_1", "const_1";
	};
	CONST_S = {
		description = "``";
		"stack_1", "const_1";
	};
	SYSREQ_D = {};
	SYSREQ_ND = {};
	TRACKER_PUSH_C = {
		description = "``";
		"const_1";
	};
	TRACKER_POP_SETHEAP = {
		description = "``";
	};
	GENARRAY = {
		description = "``";
		"const_1";
	};
	GENARRAY_Z = {
		description = "``";
		"const_1";
	};
	STRADJUST_PRI = {
		description = "``";
	};
	STKADJUST = {};
	ENDPROC = {
		description = "``";
	};
	LDGFN_PRI = {};
	REBASE = {};
	INITARRAY_PRI = {
		description = "``";
		"addr_1", "const_1", "const_2", "const_3", "const_4";
	};
	INITARRAY_ALT = {
		description = "``";
		"addr_1", "const_1", "const_2", "const_3", "const_4";
	};
	HEAP_SAVE = {
		description = "``";
	};
	HEAP_RESTORE = {
		description = "``";
	};

	FIRST_FAKE = {};
	FABS = {
		description = "``";
	};
	FLOAT = {
		description = "``";
	};
	FLOATADD = {
		description = "``";
	};
	FLOATSUB = {
		description = "``";
	};
	FLOATMUL = {
		description = "``";
	};
	FLOATDIV = {
		description = "``";
	};
	RND_TO_NEAREST = {
		description = "``";
	};
	RND_TO_FLOOR = {
		description = "``";
	};
	RND_TO_CEIL = {
		description = "``";
	};
	RND_TO_ZERO = {
		description = "``";
	};
	FLOATCMP = {
		description = "``";
	};
	FLOAT_GT = {
		description = "``";
	};
	FLOAT_GE = {
		description = "``";
	};
	FLOAT_LT = {
		description = "``";
	};
	FLOAT_LE = {
		description = "``";
	};
	FLOAT_NE = {
		description = "``";
	};
	FLOAT_EQ = {
		description = "``";
	};
	FLOAT_NOT = {
		description = "``";
	};
}

local error = error
local io_write = io.write
local string_upper = string.upper
local tonumber = tonumber

local read_line = io.lines()

local found
for line in read_line do
	if line:find("^#define OPCODE_LIST") then
		found = true
		break
	end
end

if not found then
	return error("couldn't find at least `#define OPCODE_LIST` in stdin")
end

local function unquot(what)
	return what:match("^\"(.-)\"$")
end

local instructions = {}
local instructions_i = -2
local instruction_count = 0

for line in read_line do
	--- @type string
	line = line:match("^%s*(.-)%s*$")
	if not line or line == '' then break end

	local which, rest_i = line:match("^([^%(]+)%(()")
	if which == "_G" then
		local identifier, nice_name, n_cells =
			line:match("^([^,]+),%s*(%b\"\"),%s*([^%)]+)", rest_i)
		nice_name = unquot(nice_name)
		instructions_i = instructions_i + 3
		instructions[instructions_i] = identifier
		instructions[instructions_i + 1] = nice_name
		instructions[instructions_i + 2] = n_cells
		instruction_count = instruction_count + 1
	elseif which == "_U" then
		local identifier, nice_name = line:match("^([^,]+),%s*(%b\"\")", rest_i)
		nice_name = unquot(nice_name)
		instructions_i = instructions_i + 3
		instructions[instructions_i] = identifier
		instructions[instructions_i + 1] = nice_name
		instructions[instructions_i + 2] = nil
		instruction_count = instruction_count + 1
	elseif not line:find("^/") then
		return error("unrecognized opcode type: " .. which)
	end
end

local function rustify_opcode(identifier)
	return identifier:lower()
		:gsub("_([a-z])", string_upper)
		:gsub("^.", string_upper)
end

for i = 1, instructions_i, 3 do
	local n_cells = tonumber(instructions[i + 2])
	local opcode = instructions[i]
	if n_cells and n_cells < 0 and opcode ~= "CASETBL" then
		return error(
			"instruction " .. opcode .. " has invalid size " .. n_cells
		)
	end

	local doc = OPCODE_MAP[opcode]
	if not doc then
		return error(
			"instruction " .. opcode .. " is undocumented"
		)
	end

	if n_cells and n_cells > 1 then
		local arg_n = n_cells - 1
		if #doc ~= arg_n then
			return error(
				"instruction " .. opcode .. " has " .. arg_n ..
				" argument(s), however the documented count is " .. #doc
			)
		end
	end
end

io_write([[
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

]])

io_write [[
/// Enumeration of every possible SourcePawn instruction.
/// 
/// This type is generated automatically by a script.
]]
io_write("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
io_write("#[repr(C)]\n")
io_write("pub enum Instruction {\n")
for i = 1, instructions_i, 3 do
	local opcode = instructions[i]
	local rust_opcode = rustify_opcode(opcode)
	-- if opcode == "CASETBL" then
	--	io_write('\t', rust_opcode, ",\n")
	-- else
	if instructions[i + 2] then
		local doc = OPCODE_MAP[opcode]
		if #doc > 0 then
			io_write('\t', rust_opcode, " {\n")
			for i = 1, #doc do
				io_write("\t\t", doc[i], ": Cell,\n")
			end
			io_write("\t},\n")
		else
			io_write('\t', rust_opcode, ",\n")
		end
	end
end
io_write("}\n\n")

io_write("impl Instruction {\n")

io_write("\tpub fn read_from(r: &mut impl ReadBytesExt) -> IoResult<Self> {\n")
io_write("\t\tmatch read_cell(r)? {\n")
do
	local opcode_byte = 0
	for i = 1, instructions_i, 3 do
		if instructions[i + 2] then
			local opcode = instructions[i]
			local doc = OPCODE_MAP[opcode]
			io_write("\t\t\t", opcode_byte, " => ")
			if #doc > 0 then
				io_write("{\n")
				for i = 1, #doc do
					io_write("\t\t\t\tlet ", doc[i], " = read_cell(r)?;\n")
				end
				io_write("\t\t\t\tOk(Self::", rustify_opcode(opcode), " {\n")
				for i = 1, #doc do
					io_write("\t\t\t\t\t", doc[i], ",\n")
				end
				io_write("\t\t\t\t})\n")
				io_write("\t\t\t}\n")
			else
				io_write("Ok(Self::", rustify_opcode(opcode), "),\n")
			end
		end
		opcode_byte = opcode_byte + 1
	end
end
io_write([[
			opcode => Err(IoError::new(
				IoErrorKind::InvalidData, format!("invalid opcode: {opcode:?}")
			))
]])
io_write("\t\t}\n")
io_write("\t}\n\n")

io_write("\tpub fn write_to(&self, w: &mut impl WriteBytesExt) -> IoResult<()> {\n")
io_write("\t\tmatch self {\n")
do
	local opcode_byte = 0
	for i = 1, instructions_i, 3 do
		if instructions[i + 2] then
			local opcode = instructions[i]
			local doc = OPCODE_MAP[opcode]
			io_write("\t\t\tSelf::", (rustify_opcode(opcode)))
			if #doc > 0 then
				io_write(" { ")
				for i = 1, #doc do
					io_write(doc[i], ", ")
				end
				io_write("} => {\n")
				io_write("\t\t\t\twrite_cell(w, ", opcode_byte, ")?;\n")
				for i = 1, #doc do
					io_write("\t\t\t\twrite_cell(w, *", doc[i], ")?;\n")
				end
				io_write("\t\t\t\tOk(())\n")
				io_write("\t\t\t}\n")
			else
				io_write(" => write_cell(w, ", opcode_byte, "),\n")
			end
		end
		opcode_byte = opcode_byte + 1
	end
end
io_write("\t\t}\n")
io_write("\t}\n")

io_write("}\n")
