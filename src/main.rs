use cpu::Cpu;

mod cpu;
mod inst;
mod reg;

/// The amount of memory in the system in
const MEMORY_SIZE: u64 = 1024 * 1024 * 128; // (128MiB)

fn main() {
    Cpu::default().run();
}
