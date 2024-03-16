#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use basic_prime_test_core;
// use crate::basic_prime_test_core;

risc0_zkvm::guest::entry!(main);


fn main() {
    let number_to_test: u32 = env::read();
    assert!(number_to_test % 2 != 0);
    let mut divisor = 3;
    let mut may_be_prime = true;

    while (may_be_prime && (divisor < number_to_test)) {
        may_be_prime = number_to_test % divisor != 0;
        divisor += 2;
    }

    let ouputs: basic_prime_test_core::Outputs = Outputs {
        tested_number: number_to_test,
        is_prime: may_be_prime && (divisor == number_to_test),
    }; 

    env::commit(&ouputs);
}
