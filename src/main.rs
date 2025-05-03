use std::{fs, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use cpu::Cpu;
use goblin::elf::Sym;
use rom::Rom;

pub mod cpu;
pub mod inst;
pub mod reg;
pub mod rom;

/// The unsigned width of an x register in bits (either u32 or u64).
#[allow(non_camel_case_types)]
pub type uxlen = u32;

/// The signed width of an x register in bits (either i32 or i64).
#[allow(non_camel_case_types)]
pub type ixlen = i32;

/// A RISC-V emulator.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The RISC-V binary file to run.
    #[arg(short, long)]
    bin: PathBuf,
}

fn main() -> anyhow::Result<()> {
    // Get the arguments from the command line.
    let args = Args::parse();

    // Get the binary data from the provided file.
    let path = PathBuf::from(args.bin);
    let mut bytes = fs::read(&path).context("Could not read file.")?;

    // Prepare to read some symbols from the ELF file.
    let elf = goblin::elf::Elf::parse(&bytes).context("Failed to parse ELF file")?;
    let symbols = &elf.syms;
    let strtab = &elf.strtab;
    let get_symbol_value = |name: &str| -> anyhow::Result<Sym> {
        symbols
            .iter()
            .find(|sym| strtab.get_at(sym.st_name).is_some_and(|n| n == name))
            .with_context(|| format!("Could not find symbol '{name}' in ELF file"))
    };

    // The `_start` symbol is the start address of the ELF file.
    let start = get_symbol_value("_start")?.st_value as usize;

    // The `_end` symbol is the end address of the ELF file.
    let end = get_symbol_value("_end")?.st_value as usize;

    // The `_tohost` symbol is the start address of the program that should be run.
    // NOTE: This symbol is used for the 'riscv-tests' suite.
    let tohost = get_symbol_value("tohost")?.st_value as usize;

    // Create a ROM from the data in the ELF file.
    let rom = Rom::new(
        &mut bytes[(tohost - start)..(end - start)],
        start as uxlen,
        end as uxlen,
    );

    // Create and run the CPU cycle loop.
    Cpu::new(&rom).run().context("Error in running CPU")?;

    Ok(())
}
