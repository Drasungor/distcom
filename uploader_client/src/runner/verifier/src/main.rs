use methods::DOWNLOADED_GUEST_ID;
use risc0_zkvm::ExecutorEnv;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use bincode;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().collect();

    let program_id = args[1].parse::<String>().expect("Must receive the program id");
    let input_group_id = args[2].parse::<String>().expect("Must receive the input group id");
    let proof_file_path = format!("../../downloads/proof.bin");
    let mut proof_file = File::open(proof_file_path).expect("Error while reading file");
    let mut proof_data = Vec::new();
    proof_file.read_to_end(&mut proof_data).expect("Error while reading proof file");

    // Deserialize the proof
    let receipt: risc0_zkvm::Receipt = bincode::deserialize(&proof_data).expect("Error in proof deserialization");
    receipt
        .verify(DOWNLOADED_GUEST_ID)
        .expect("Proof verification failed");

    let output: String = receipt.journal.decode().unwrap();
    let output_dir_path = format!("../../programs_data/{program_id}/{input_group_id}");
    fs::create_dir_all(&output_dir_path).expect("Error creating directories");

    // let output_file_path = format!("../../programs_data/{program_id}/{input_group_id}/output.json");
    let output_file_path = format!("{output_dir_path}/output.json");
    
    let mut file = File::create(output_file_path).expect("Error in output file creation");
    file.write_all(output.as_bytes()).expect("Errors in file write");
 
    println!("Proof verification successful");
    println!("output: {output}")
}
