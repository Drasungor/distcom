use methods::{
    BASIC_PRIME_TEST_GUEST_ELF, BASIC_PRIME_TEST_GUEST_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::env;
use basic_prime_test_core;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().collect();

    println!("The args list is: {:?}", args);

    if (args.len() != 2) {
        panic!("The program needs a number for primality check");
    }

    let tested_number = args[1].parse::<u32>().expect("The received input is not a positive integer");

    let env = ExecutorEnv::builder()
        .write(&tested_number)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let receipt = prover
        .prove(env, BASIC_PRIME_TEST_GUEST_ELF)
        .unwrap();

    let serialized_proof = bincode::serialize(&receipt).expect("Error in proof serialization");

    std::fs::write("./proof.bin", serialized_proof);


    let _output: basic_prime_test_core::Outputs = receipt.journal.decode().unwrap();

    println!("The output of the journal is {:?}", _output);

    receipt
        .verify(BASIC_PRIME_TEST_GUEST_ID)
        .expect("Proof verification failed");
    println!("Successful verification");
}
