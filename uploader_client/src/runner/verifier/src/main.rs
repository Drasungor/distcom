use methods::{
    // DOWNLOADED_GUEST_ELF, DOWNLOADED_GUEST_ID
    DOWNLOADED_GUEST_ID
};
// use risc0_zkvm::{default_verifier, ExecutorEnv};
use risc0_zkvm::ExecutorEnv;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use bincode;

// use basic_prime_test_core;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // let args: Vec<String> = env::args().collect();

    // let program_input_file_name = args[1].parse::<String>().expect("The received input is not a positive integer");


    // let mut env_builder = ExecutorEnv::builder();
    // let mut env_bulder_ref = &mut env_builder;
    // let program_input_path = format!("../../program_with_input/{}", program_input_file_name);
    let proof_file_path = format!("../../downloads/proof.bin");

    let mut proof_file = File::open(proof_file_path).expect("Error while reading file");

    // let executor_env = env_bulder_ref.build().unwrap();

    // let prover = default_prover();
    // let receipt = prover
    //     .prove(executor_env, DOWNLOADED_GUEST_ELF)
    //     .unwrap();


    let mut proof_data = Vec::new();
    proof_file.read_to_end(&mut proof_data).expect("Error while reading proof file");

    // Deserialize the proof
    let receipt: risc0_zkvm::Receipt = bincode::deserialize(&proof_data).expect("Error in proof deserialization");
    // let serialized_proof = bincode::serialize(&receipt.receipt).expect("Error in proof serialization");

    // let verifier = default_verifier();
    receipt
        .verify(DOWNLOADED_GUEST_ID)
        .expect("Proof verification failed");

    println!("Proof verification successful");

    // let receipt = &prove_info.receipt;
    // receipt
    //     .verify(DOWNLOADED_GUEST_ID)
    //     .expect("Proof verification failed");
    // println!("Successful verification");

}
