#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FermatOutputs {
    pub tested_number: u32,
    pub is_probably_prime: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MillerRabinOutputs {
    pub tested_number: u32,
    pub is_probably_prime: bool,
    pub iterations_limit_reached: bool,
}

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn get_base_2_multiplier(tested_number: u32) -> u32 {
    let mut processed_number = tested_number - 1;
    while processed_number % 2 == 0 {
        processed_number /= 2;
    }
    return processed_number;
}

fn modular_exponentiation(base: u32, exponent: u32, modulo: u32) -> u32 {
    let cast_base = base as u64;
    if (exponent == 2) {
        return (base * base) % modulo;
    }
    if (exponent == 1) {
        return base;
    }
    let sqrt = modular_exponentiation(base, exponent/2, modulo);
    if (exponent % 2 == 0) {
        return (sqrt * sqrt) % modulo;
    } else {
        return (((sqrt * sqrt) % modulo) * base) % modulo;
    }
}


fn miller_rabin(input: Vec<u8>) -> MillerRabinOutputs {
    let first_four_bytes = &input[0..4];
    let number_to_test = u32::from_be_bytes(first_four_bytes.try_into().expect("Error transforming into number from bytes"));
    let second_four_bytes = &input[4..8];
    let iterations_limit = u32::from_be_bytes(first_four_bytes.try_into().expect("Error transforming into number from bytes"));
    let mut iteration_counter = 1;
    let mut was_result_obtained = false;
    let mut is_probably_prime = false;

    let initial_value = modular_exponentiation(2, get_base_2_multiplier(number_to_test), number_to_test);

    // Modulo == +-1
    if (initial_value == 1) || (initial_value == (number_to_test - 1)) {
        was_result_obtained = true;
        is_probably_prime = true;
    }
    let mut current_value = initial_value;

    while !was_result_obtained && iteration_counter < iterations_limit {
        current_value = (current_value * current_value) % number_to_test;

        // Modulo == 1
        if current_value == 1 {
            was_result_obtained = true;
            is_probably_prime = false;
        }

        // Modulo == -1
        if current_value == (number_to_test - 1) {
            was_result_obtained = true;
            is_probably_prime = true;
        }
        iteration_counter += 1;
    } 

    let outputs: MillerRabinOutputs = MillerRabinOutputs {
        tested_number: number_to_test,
        is_probably_prime,
        iterations_limit_reached: iteration_counter == iterations_limit,
    };
    return outputs;
}

fn probabilistic_fermat(input: Vec<u8>) -> FermatOutputs {
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
        let divisor = modular_exponentiation(random_input, number_to_test - 1, number_to_test);
        may_be_prime = divisor % number_to_test == 1;
        iteration_counter += 1;
    } 

    let outputs: FermatOutputs = FermatOutputs {
        tested_number: number_to_test,
        is_probably_prime: may_be_prime,
    }; 
    return outputs;
}


fn main() {
    let input1: Vec<u8> = env::read();
    let probabilistic_fermat_result: FermatOutputs = probabilistic_fermat(input1);
    
    if !probabilistic_fermat_result.is_probably_prime {
        let returned_output = MillerRabinOutputs {
            tested_number: probabilistic_fermat_result.tested_number,
            is_probably_prime: probabilistic_fermat_result.is_probably_prime,
            iterations_limit_reached: false,
        };
        let serialized_outputs = to_string(&returned_output).expect("Error in fermat result struct serialization");
        env::commit(&serialized_outputs);
    } else {
        let input2: Vec<u8> = env::read();
        let miller_rabin_result: MillerRabinOutputs = miller_rabin(input2);
        let serialized_outputs = to_string(&miller_rabin_result).expect("Error in miller rabin result struct serialization");
        env::commit(&serialized_outputs);
    }

}