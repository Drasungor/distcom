pub fn proof_verification() {
    // let serialized_proof = bincode::serialize(&receipt).expect("Error in proof serialization");
    let serialized_proof = bincode::serialize(&receipt).expect("Error in proof serialization");

    std::fs::write("./proof.bin", serialized_proof);


    let _output: basic_prime_test_core::Outputs = receipt.journal.decode().unwrap();

    println!("The output of the journal is {:?}", _output);

    receipt
        .verify(BASIC_PRIME_TEST_GUEST_ID)
        .expect("Proof verification failed");
    println!("Successful verification");
}