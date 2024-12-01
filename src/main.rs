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

use clap::Parser;
use popsim::run_rom;

fn main() {
    let args = Args::parse();
    run_rom(&args.rom);
}

#[derive(Parser)]
#[command(version)]
struct Args {
    rom: String,
}
