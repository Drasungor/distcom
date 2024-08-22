use std::fs::{self, File};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub tested_number: u32,
    pub is_probably_prime: bool,
}

pub fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn create_folder(path: &str) {
    if !folder_exists(path) {
        fs::create_dir_all(path).expect("Error in uploads folder creation")
    }
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

fn execute_fermat_test(input: Vec<u8>) -> Outputs {
    let first_four_bytes = &input[0..4];
    let number_to_test = u32::from_be_bytes(first_four_bytes.try_into().expect("Error transforming into number from bytes"));
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

    let outputs: Outputs = Outputs {
        tested_number: number_to_test,
        is_probably_prime: may_be_prime,
    };
    outputs
}

fn main() {
    let dir_entries = fs::read_dir("./inputs").expect("Failed reading the inputs folder");
    let _ = fs::remove_dir_all("./outputs");
    create_folder("./outputs");
    for entry in dir_entries {
        let dir_entry = entry.expect("Error in entry parsing");
        let current_path = dir_entry.path();
        let file = File::open(current_path.clone()).expect("Failed opening input file");
        let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);

        for line in reader.records() {
            let line_ok = line.expect("Error in read line");
            let line_iterator = line_ok.into_iter();
            let mut counter = 0;
            for value in line_iterator {
                counter += 1;
                let output = execute_fermat_test(BASE64_STANDARD.decode(value).expect("Error in input base 64 decoding"));
                let serialized_outputs = to_string(&output).expect("Error in struct serialization");
                let input_file_name = current_path.file_name().expect("There is no file in this path");
                let file_name_string = input_file_name.to_str().unwrap().split(".").collect::<Vec<&str>>()[0];
                std::fs::write(format!("./outputs/{file_name_string}.json"), serialized_outputs).expect("Error writing proof to file");
            }
            assert!(counter == 1, "There is more than one element per line");
        }
    }
}