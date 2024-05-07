use methods::{
    BASIC_PRIME_TEST_GUEST_ELF, BASIC_PRIME_TEST_GUEST_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::env;
use std::fs::File;
use std::io::{self, Read};

// use basic_prime_test_core;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // let tested_number = args[1].parse::<u32>().expect("The received input is not a positive integer");

    // let env = ExecutorEnv::builder()
    // .write(&tested_number)
    // .unwrap()
    // .build()
    // .unwrap();

    let mut env_builder = ExecutorEnv::builder();
    let mut env_bulder_ref = &env_builder;

    let file = File::open("input_file_path").expect("Error while reading file");
    let mut input_reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut current_input = 0;
    for line in input_reader.records() {
        let line_ok = line.expect("Error in line reading");
        let line_iterator = line_ok.into_iter();
        let mut counter = 0;

        for value in line_iterator {
            
            // TODO: decode this string from utf8 before assigning a value to decoded_value
            let decoded_value = value;

            let decoded_bytes = decoded_value.as_bytes();


            // env_bulder_ref = env_bulder_ref.write(decoded_value.as_bytes()).unwrap();
            env_bulder_ref = env_bulder_ref.write(&decoded_bytes).unwrap();
            counter += 1;
        }
        assert!(counter == 1, "There is more than one element per line");
        current_input += 1;
    }


    let prover = default_prover();
    let receipt = prover
        .prove(env, BASIC_PRIME_TEST_GUEST_ELF)
        .unwrap();

    let _output: basic_prime_test_core::Outputs = receipt.journal.decode().unwrap();

    println!("The output of the journal is {:?}", _output);

    receipt
        .verify(BASIC_PRIME_TEST_GUEST_ID)
        .expect("Proof verification failed");
    println!("Successful verification");
}
