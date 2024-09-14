#!/usr/bin/env sh
# Run this script from inside the repository's root.

if [ ! -f "tools/smx-v1-opcodes.h" ]; then
	wget \
		-O "tools/smx-v1-opcodes.h" \
		"https://raw.githubusercontent.com/alliedmodders/sourcepawn/master/include/smx/smx-v1-opcodes.h"
fi

cat tools/smx-v1-opcodes.h | lua tools/parse_opcodes.lua
