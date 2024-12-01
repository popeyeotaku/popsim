# PopSim

A simple 6502 system emulator.

Reading from $FF returns the next character from standard input,
waiting for the character if needed. Writing to $FF writes to
standard output.

The ROM is loaded into the top of memory, so that its last 6 bytes
are the 6502 vectors.
