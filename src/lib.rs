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
//!
//! Programs end by writing to the port at $FE.

use emulator_6502::MOS6502;
use memory::Memory;

/// Load the rom file and run it.
pub fn run_rom(rom_file: &str) {
    let rom = load_rom(rom_file);
    let mut memory = Memory::new(&rom, None, false);
    run_program(&mut memory);
}

/// Load the run file and run it, with "standard input" coming from the `stdin` string, and returning
/// standard output.
pub fn run_captured(rom_file: &str, stdin: &str) -> String {
    let rom = load_rom(rom_file);
    let mut memory = Memory::new(&rom, Some(stdin), true);
    run_program(&mut memory);
    String::from_utf8(memory.output.unwrap()).unwrap()
}

/// Run a program via the given memory.
fn run_program(memory: &mut Memory<'_, '_>) {
    let mut cpu = MOS6502::new_reset_position(memory);
    while !memory.wrote_eof {
        cpu.cycle(memory);
    }
}

/// Load a rom file.
fn load_rom(path: &str) -> Vec<u8> {
    todo!()
}

mod memory {
    //! The memory interface for the 6502.

    use std::{
        io::{stdin, stdout, Read, Write},
        str::Bytes,
    };

    const ROM_SIZE: u16 = 16 * 1024;
    const RAM_SIZE: u16 = ((u16::MAX as usize) + 1 - (ROM_SIZE as usize)) as u16;
    const ROM_BOT: u16 = RAM_SIZE;

    const IO_PORT: u16 = 0xFF;
    const EOF_PORT: u16 = 0xFE;

    use emulator_6502::Interface6502;

    pub struct Memory<'a, 'b> {
        ram: [u8; RAM_SIZE as usize],
        rom: &'b [u8],
        input: Option<Bytes<'a>>,
        pub output: Option<Vec<u8>>,
        at_eof: bool,
        pub wrote_eof: bool,
    }

    impl<'a, 'b> Memory<'a, 'b> {
        pub fn new(rom: &'b [u8], input: Option<&'a str>, output: bool) -> Self {
            let output: Option<Vec<u8>> = {
                if output {
                    Some(Vec::new())
                } else {
                    None
                }
            };
            let input = input.map(|s| s.bytes());
            assert!(rom.len() == ROM_SIZE as usize);
            Self {
                ram: [0; RAM_SIZE as usize],
                rom,
                input,
                output,
                at_eof: false,
                wrote_eof: false,
            }
        }
    }

    impl Interface6502 for Memory<'_, '_> {
        fn read(&mut self, address: u16) -> u8 {
            match address {
                IO_PORT => {
                    if let Some(bytes) = &mut self.input {
                        if let Some(c) = bytes.next() {
                            self.at_eof = false;
                            c
                        } else {
                            self.at_eof = true;
                            0
                        }
                    } else if let Some(c) = stdin().bytes().next().map(|r| r.unwrap()) {
                        self.at_eof = false;
                        c
                    } else {
                        self.at_eof = true;
                        0
                    }
                }
                EOF_PORT => {
                    if self.at_eof {
                        0xFF
                    } else {
                        0
                    }
                }
                ROM_BOT.. => self.rom[(address - 0xC000) as usize],
                _ => self.ram[address as usize],
            }
        }

        fn write(&mut self, address: u16, data: u8) {
            match address {
                IO_PORT => {
                    if let Some(s) = &mut self.output {
                        s.push(data);
                    } else {
                        stdout().write_all(&[data]).unwrap()
                    }
                }
                EOF_PORT => {
                    self.wrote_eof = true;
                }
                ROM_BOT.. => (),
                _ => self.ram[address as usize] = data,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::run_captured;

    #[test]
    fn hello_world() {
        assert_eq!(run_captured("test.bin", "foo"), "\r\n\r\nHELLO, foo!\r\n");
    }
}
