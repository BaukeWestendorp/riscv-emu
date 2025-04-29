use std::{fs::File, io::Read, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use cpu::Cpu;

pub mod cpu;
pub mod inst;
pub mod reg;

/// The current width of an x register in bits (either 32 or 64).
#[allow(non_camel_case_types)]
pub type xlen = u64;

/// The amount of memory in the system in
pub const MEMORY_SIZE: xlen = 1024 * 1024 * 128; // (128MiB)

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

    let mut file = File::open(&args.bin).context("Failed to open binary file")?;
    let mut memory = Vec::new();
    file.read_to_end(&mut memory)
        .context("Failed to read file contents into emulator's CPU memory")?;

    Cpu::new(memory).run().context("Error in running CPU")?;

    Ok(())
}
