#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub tested_number: u32,
    pub is_prime: bool,
}

use risc0_zkvm::guest::env;
// use crate::basic_prime_test_core;

risc0_zkvm::guest::entry!(main);

fn main() {
    // let number_to_test: u32 = env::read();

    // let byte_array: [u8; 5] = [1, 2, 3, 4, 5];

    let input: [u8; 1024] = env::read();

    let first_four_bytes = &input[0..4];
    let number_to_test = u32::from_be_bytes(first_four_bytes.try_into().unwrap());

    assert!(number_to_test % 2 != 0);
    let mut divisor = 3;
    let mut may_be_prime = true;

    while (may_be_prime && (divisor < number_to_test)) {
        may_be_prime = number_to_test % divisor != 0;
        divisor += 2;
    }

    let ouputs: Outputs = Outputs {
        tested_number: number_to_test,
        is_prime: may_be_prime && (divisor == number_to_test),
    }; 

    env::commit(&ouputs);
}
