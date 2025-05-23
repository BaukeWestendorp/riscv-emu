use std::{
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Parser;
use emu::{cpu::Cpu, rom::Rom, uxlen};
use goblin::elf::Sym;

/// A RISC-V emulator.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The riscv-tests test name.
    #[arg(short, long)]
    test_name: Option<String>,

    /// Prints information about the current instruction for each cycle.
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    // Get the arguments from the command line.
    let args = Args::parse();

    let riscv_tests_path = Path::new("riscv-tests").join("isa");

    match args.test_name {
        Some(test_name) => {
            let file_name = format!("rv32ui-p-{test_name}");
            let path = &riscv_tests_path.join(file_name);
            run_test(path, args.verbose)
                .with_context(|| format!("Failed to run test at '{}'", path.display()))?;
        }
        None => {
            let test_paths = riscv_tests_path
                .read_dir()
                .context("Failed to read riscv-tests folder")?
                .filter_map(|entry| {
                    entry.ok().filter(|e| {
                        e.file_name().as_bytes().starts_with(b"rv32ui-p-")
                            && !e.file_name().as_bytes().ends_with(b".dump")
                    })
                })
                .map(|entry| entry.path());

            for path in test_paths {
                run_test(&path, args.verbose)
                    .with_context(|| format!("Failed to run test at '{}'", path.display()))?;
            }
        }
    }

    Ok(())
}

fn run_test(path: &PathBuf, verbose: bool) -> anyhow::Result<()> {
    eprintln!("Running test at '{}'...", path.display());
    // Get the binary data from the provided file.
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
    let tohost = get_symbol_value("tohost")?.st_value as usize;

    // Create a ROM from the data in the ELF file.
    let rom = Rom::new(&mut bytes[(tohost - start)..(end - start)], start as uxlen, end as uxlen);

    // Create and run the CPU cycle loop.
    Cpu::new(&rom, verbose)
        .on_ecall(Box::new(|cpu| {
            // a7 is the syscall register used, 0x5D indicates test status syscall.
            if cpu.registers().a7() == 0x5D {
                // a0 indicates the test status.
                let status = cpu.registers().a0();
                if status == 0 {
                    eprintln!("Test Passed!");
                } else {
                    let failed_test_num = (status - 1) / 2;
                    eprintln!("Test {} Failed!", failed_test_num);
                    cpu.abort();
                }
            }
        }))
        .run()
        .context("Error in running CPU")?;

    Ok(())
}
