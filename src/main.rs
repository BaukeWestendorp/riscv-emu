use std::path::PathBuf;

use clap::Parser;
use cpu::Cpu;

pub mod cpu;
pub mod inst;
pub mod reg;

/// The amount of memory in the system in
const MEMORY_SIZE: u64 = 1024 * 1024 * 128; // (128MiB)

/// A RISC-V emulator.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The RISC-V binary file to run.
    #[arg(short, long)]
    bin: PathBuf,
}

fn main() {
    let args = Args::parse();

    Cpu::default().run();
}
