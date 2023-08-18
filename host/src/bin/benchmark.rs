use ecdsa_methods::BENCHMARK_ELF;
use risc0_zkvm::{Executor, ExecutorEnv};

// Simple main to load and run the benchmark binary in the RISC Zero Executor.
fn main() {
    let env = ExecutorEnv::builder().build().unwrap();
    let mut exec = Executor::from_elf(env, BENCHMARK_ELF).unwrap();
    std::hint::black_box(exec.run().unwrap());
}
