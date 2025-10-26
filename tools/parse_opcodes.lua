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
		description = "`vm.pri = vm.smx_data.get_cell(offset);`";
		"offset";
	};
	LOAD_ALT = {
		description = "`vm.alt = vm.smx_data.get_cell(offset);`";
		"offset";
	};
	LOAD_S_PRI = {
		description = "`vm.pri = vm.stack.get_cell(offset);`";
		"offset";
	};
	LOAD_S_ALT = {
		description = "`vm.alt = vm.stack.get_cell(offset);`";
		"offset";
	};
	LREF_PRI = {};
	LREF_ALT = {};
	LREF_S_PRI = {
		description = "`vm.pri = vm.stack.get_cell(vm.stack.get_cell(offset));`";
		"offset";
	};
	LREF_S_ALT = {
		description = "`vm.alt = vm.stack.get_cell(vm.stack.get_cell(offset));`";
		"offset";
	};
	LOAD_I = {
		description = "`vm.pri = vm.stack.get_cell(vm.pri);`";
	};
	LODB_I = {
		description = "`let value = vm.smx_data[vm.pri]; vm.pri = match width { 1 => value & 0xff, 2 => value & 0xffff, 4 => value, _ => panic!() };`";
		"width";
	};
	CONST_PRI = {
		description = "`vm.pri = value;`";
		"value";
	};
	CONST_ALT = {
		description = "`vm.alt = value;`";
		"value";
	};
	ADDR_PRI = {
		description = "`vm.pri = &vm.stack[offset];`";
		"offset";
	};
	ADDR_ALT = {
		description = "`vm.alt = &vm.stack[offset];`";
		"offset";
	};
	STOR_PRI = {
		description = "`vm.smx_data.set_cell(offset, vm.pri);`";
		"offset";
	};
	STOR_ALT = {
		description = "`vm.smx_data.set_cell(offset, vm.alt);`";
		"offset";
	};
	STOR_S_PRI = {
		description = "`vm.stack.set_cell(offset, vm.pri);`";
		"offset";
	};
	STOR_S_ALT = {
		description = "`vm.stack.set_cell(offset, vm.alt);`";
		"offset";
	};
	SREF_PRI = {};
	SREF_ALT = {};
	SREF_S_PRI = {
		description = "`vm.stack.set_cell(vm.stack.get_cell(offset), vm.pri);`";
		"offset";
	};
	SREF_S_ALT = {
		description = "`vm.stack.set_cell(vm.stack.get_cell(offset), vm.alt);`";
		"offset";
	};
	STOR_I = {
		description = "`vm.heap.set_cell(vm.alt, vm.pri);`";
	};
	STRB_I = {
		description = "`vm.smx_data[vm.alt] = match width { 1 => vm.pri & 0xff, 2 => vm.pri & 0xffff, 4 => vm.pri, _ => panic!() };`";
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
		description = "`vm.stack.push_cell(pri); vm.stack.push_cell(alt);`";
	};
	PUSH_PRI = {
		description = "`vm.stack.push_cell(vm.pri);`";
	};
	PUSH_ALT = {
		description = "`vm.stack.push_cell(vm.alt);`";
	};
	PUSH_R = {};
	PUSH_C = {
		description = "`vm.stack.push_cell(value);`";
		"value";
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
		description = "`vm.pri = vm.stack.pop_cell();`";
	};
	POP_ALT = {
		description = "`vm.alt = vm.stack.pop_cell();`";
	};
	STACK = {
		description = "`vm.stack.pop_bytes(count);`";
		"count";
	};
	HEAP = {
		description = "`vm.alt = vm.heap.alloc_bytes(size);`";
		"size";
	};
	PROC = {
		description = "Indicates the start of a procedure.";
	};
	RET = {};
	RETN = {
		description = "`vm.do_return(vm.pri);`";
	};
	CALL = {
		description = [[
Call procedure at `code_offset`, given that the number of arguments is the top-most [`Cell`] in `vm.stack`.
The return value of the procedure is placed into `vm.pri`.
]];
		"code_offset";
	};
	CALL_PRI = {};
	JUMP = {
		description = "`vm.pc = code_offset;`";
		"code_offset";
	};
	JREL = {};
	JZER = {
		description = "`if vm.pri == 0 { vm.pc = code_offset; }`";
		"code_offset";
	};
	JNZ = {
		description = "`if vm.pri != 0 { vm.pc = code_offset; }`";
		"code_offset";
	};
	-- FIXME: This is a guess.
	JEQ = {
		description = "`if vm.pri == vm.alt { vm.pc = code_offset; }`";
		"code_offset";
	};
	-- FIXME: This is a guess.
	JNEQ = {
		description = "`if vm.pri != vm.alt { vm.pc = code_offset; }`";
		"code_offset";
	};
	JLESS = {};
	JLEQ = {};
	JGRTR = {};
	JGEQ = {};
	JSLESS = {
		description = [[
```no_compile
let a = vm.stack.pop_cell();
let b = vm.stack.pop_cell();
if a < b {
	vm.pc = code_offset;
}
```]];
		"code_offset";
	};
	JSLEQ = {
		description = [[
```no_compile
let a = vm.stack.pop_cell();
let b = vm.stack.pop_cell();
if a <= b {
	vm.pc = code_offset;
}
```]];
		"code_offset";
	};
	JSGRTR = {
		description = [[
```no_compile
let a = vm.stack.pop_cell();
let b = vm.stack.pop_cell();
if a > b {
	vm.pc = code_offset;
}
```]];
		"code_offset";
	};
	JSGEQ = {
		description = [[
```no_compile
let a = vm.stack.pop_cell();
let b = vm.stack.pop_cell();
if a >= b {
	vm.pc = code_offset;
}
```]];
		"code_offset";
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
		description = "`vm.pri += value;`";
		"value";
	};
	SMUL_C = {
		description = "``";
		"const_1";
	};
	ZERO_PRI = {
		description = "`vm.pri = 0;`";
	};
	ZERO_ALT = {
		description = "`vm.alt = 0;`";
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
		description = "`if vm.pri == vm.alt { vm.pri = 1; } else { vm.pri = 0; }`";
	};
	NEQ = {
		description = "`if vm.pri != vm.alt { vm.pri = 1; } else { vm.pri = 0; }`";
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
		description = [[
```no_compile
for offset in 0..count {
	vm.heap.set_byte(
		vm.alt, offset,
		vm.smx_data.get_byte(vm.pri + offset)
	);
}
```]];
		"count";
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
		description = [[
Allocate an array of cells, with `dimensions` specifying the number of sizes for each dimension to be popped from the
stack, with the top-most element being the last dimension.]];
		"dimensions";
	};
	GENARRAY_Z = {
		description = "Same as [`Self::Genarray`], but the array is filled with `0`s.";
		"dimensions";
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
		"data_offset", "const_1", "size", "const_3", "const_4";
	};
	INITARRAY_ALT = {
		description = "``";
		"data_offset", "const_1", "size", "const_3", "const_4";
	};
	HEAP_SAVE = {
		description = "``";
	};
	HEAP_RESTORE = {
		description = "vm.heap.free_all();";
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
