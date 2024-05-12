#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

use basic_prime_test_core;

// Your code inside main
fn main() {
    // Read your inputs:
    // let read_inputs: u32 = env::read();

    // Validate necessary_properties:
    // assert!(number_to_test % 2 != 0);

    // Commit to outputs:
    // env::commit(&ouputs);
}
