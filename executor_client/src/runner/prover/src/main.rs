use methods::{
    DOWNLOADED_GUEST_ELF, DOWNLOADED_GUEST_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::env;
use std::fs::File;
// use std::io::{self, Read};
use bincode;
use csv;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().collect();

    let program_input_file_name = args[1].parse::<String>().expect("The received input is not a positive integer");

    let mut env_builder = ExecutorEnv::builder();
    let mut env_bulder_ref = &mut env_builder;
    let program_input_path = format!("../../program_with_input/{}", program_input_file_name);

    let file = File::open(program_input_path).expect("Error while reading file");
    let mut input_reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);

    for line in input_reader.records() {
        let line_ok = line.expect("Error in line reading");
        let line_iterator = line_ok.into_iter();
        let mut counter = 0;

        for value in line_iterator {
            let bytes_vector = base64::decode(value).expect("Failed to decode base64");
            env_bulder_ref = env_bulder_ref.write(&bytes_vector).unwrap();
            counter += 1;
        }
        assert!(counter == 1, "There is more than one element per line");
    }

    let executor_env = env_bulder_ref.build().unwrap();

    let prover = default_prover();
    let prove_info = prover
        .prove(executor_env, DOWNLOADED_GUEST_ELF)
        .unwrap();

    let receipt = &prove_info.receipt;
    let serialized_proof = bincode::serialize(&receipt).expect("Error in proof serialization");
    std::fs::write("./proof.bin", serialized_proof).expect("Error writing proof to file");
}
