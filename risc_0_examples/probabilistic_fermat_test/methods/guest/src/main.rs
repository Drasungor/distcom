#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub tested_number: u32,
    pub is_probably_prime: bool,
}

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let input: Vec<u8> = env::read();

    let first_four_bytes = &input[0..4];
    let number_to_test = u32::from_be_bytes(first_four_bytes.try_into().expect("Error transforming into number from bytes"));
    let mut randomly_generated_numbers: Vec<u32> = Vec::new();
    let mut may_be_prime = true;
    let mut iteration_counter: u32 = 0;
    while may_be_prime && iteration_counter < 20 { // At 20 iterations, if not all tests are fools, then the chance of not being prime is really low
        let current_byte_index = (4*(iteration_counter + 1)) as usize;
        let last_number_byte_index_bound = (current_byte_index + 4) as usize;
        let currently_analyzed_bytes: &[u8] = &input[current_byte_index..last_number_byte_index_bound];
        let random_input = u32::from_be_bytes(currently_analyzed_bytes.try_into().expect("Error transforming into number from bytes"));
        let cast_random_input = random_input as u128;
        let divisor: u128 = cast_random_input.checked_pow(number_to_test).expect(&format!("Exponent overflow base {} exponent {}", cast_random_input, number_to_test)) - cast_random_input;
        may_be_prime = divisor % (number_to_test as u128) == 0;
        iteration_counter += 1;
    } 

    let outputs: Outputs = Outputs {
        tested_number: number_to_test,
        is_probably_prime: may_be_prime,
    }; 
    let serialized_outputs = to_string(&outputs).expect("Error in struct serialization");
    env::commit(&serialized_outputs);
}