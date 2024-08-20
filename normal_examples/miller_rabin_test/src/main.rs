use std::fs::{self, File};

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use base64::prelude::*;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub tested_number: u32,
    pub is_probably_prime: bool,
    pub iterations_limit_reached: bool,
}

fn get_base_2_multiplier(tested_number: u32) -> u32 {
    let mut processed_number = tested_number - 1;
    while processed_number % 2 == 0 {
        processed_number /= 2;
    }
    return processed_number;
}

fn modular_exponentiation(base: u32, exponent: u32, modulo: u32) -> u32 {
    if exponent == 2 {
        return (base * base) % modulo;
    }
    if exponent == 1 {
        return base;
    }
    let sqrt = modular_exponentiation(base, exponent/2, modulo);
    if exponent % 2 == 0 {
        return (sqrt * sqrt) % modulo;
    } else {
        return (((sqrt * sqrt) % modulo) * base) % modulo;
    }
}

fn execute_miller_rabin(input: Vec<u8>) {

    let first_four_bytes = &input[0..4];
    let number_to_test = u32::from_be_bytes(first_four_bytes.try_into().expect("Error transforming into number from bytes"));
    let second_four_bytes = &input[4..8];
    let iterations_limit = u32::from_be_bytes(second_four_bytes.try_into().expect("Error transforming into number from bytes"));
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

    let outputs: Outputs = Outputs {
        tested_number: number_to_test,
        is_probably_prime,
        iterations_limit_reached: iteration_counter == iterations_limit,
    }; 
}

fn main() {
    let dir_entries = fs::read_dir("./inputs").expect("Failed reading the inputs folder");
    for entry in dir_entries {
        let dir_entry = entry.expect("Error in entry parsing");
        let current_path = dir_entry.path();
        let file = File::open(current_path).expect("Failed opening input file");
        let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);

        for line in reader.records() {
            let line_ok = line.expect("Error in read line");
            let line_iterator = line_ok.into_iter();
            let mut counter = 0;
            for value in line_iterator {
                counter += 1;
                execute_miller_rabin(BASE64_STANDARD.decode(value).expect("Error in input base 64 decoding"));
            }
            assert!(counter == 1, "There is more than one element per line");
        }

    }
}