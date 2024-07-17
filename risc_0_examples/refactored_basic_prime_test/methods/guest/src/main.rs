#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub tested_number: u32,
    pub is_prime: bool,
}

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let input: Vec<u8> = env::read();

    let first_four_bytes = &input[0..4];
    let number_to_test = u32::from_be_bytes(first_four_bytes.try_into().expect("Error transforming into number from bytes"));

    assert!(number_to_test % 2 != 0);
    let mut divisor = 3;
    let mut may_be_prime = true;

    while (may_be_prime && (divisor < number_to_test)) {
        may_be_prime = number_to_test % divisor != 0;
        divisor += 2;
    }

    let outputs: Outputs = Outputs {
        tested_number: number_to_test,
        is_prime: may_be_prime && (divisor == number_to_test),
    }; 

    // env::commit(&outputs);
    
    let serialized_outputs = to_string(&outputs).expect("Error in struct serialization");
    env::commit(&serialized_outputs);
}