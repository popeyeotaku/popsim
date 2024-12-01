//! # PopSim
//!
//! A simple 6502 system emulator.
//!
//! The rom file *must* be 16K, and is loaded into the top of memory.
//!
//! Reading from $FF returns the next byte from standard input,
//! waiting for the character if needed. Writing to $FF writes to
//! standard output.
//!
//! Standard input returns $00 from $FF on end of file. Since $00 is also a
//! possible value, reading from $FE returns $FF if we're truly at end of file,
//! $00 otherwise.

pub fn run_rom(rom_file: &str) {
    todo!()
}

pub fn run_captured(rom_file: &str, stdin: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::run_captured;

    #[test]
    fn hello_world() {
        assert_eq!(run_captured("test.bin", "foo"), "\r\n\r\nHELLO, foo!\r\n");
    }
}
