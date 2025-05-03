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

/// The current width of an x register in bits (either 32 or 64).
#[allow(non_camel_case_types)]
pub type xlen = u64;

/// A RISC-V emulator.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The RISC-V binary file to run.
    #[arg(short, long)]
    bin: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path = PathBuf::from(args.bin);
    let mut bytes = fs::read(&path).context("Could not read file.")?;

    let elf = goblin::elf::Elf::parse(&bytes).context("Failed to parse ELF")?;
    let symbols = &elf.syms;
    let strtab = &elf.strtab;
    let get_symbol_value = |name: &str| -> anyhow::Result<Sym> {
        symbols
            .iter()
            .find(|sym| strtab.get_at(sym.st_name).is_some_and(|n| n == name))
            .with_context(|| format!("Could not find symbol '{name}' in ELF executable"))
    };

    let start = get_symbol_value("_start")?.st_value as usize;
    let end = get_symbol_value("_end")?.st_value as usize;
    let tohost = get_symbol_value("tohost")?.st_value as usize;

    let rom = Rom::new(
        &mut bytes[(tohost - start)..(end - start)],
        start as xlen,
        end as xlen,
    );

    Cpu::new(&rom).run().context("Error in running CPU")?;

    Ok(())
}
