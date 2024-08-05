use std::{fs, path::Path, process::Command};

use crate::{common, services::program_distributor::{PagedProgramInputGroups, PagedPrograms}};

pub async fn verify_proven_execution(program_id: &str, input_group_id: &str) -> Result<(), ()> {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.download_program(program_id, Path::new("./src/runner/methods")).await;

    // let download_path = "../downloads/proof.bin";
    let download_path = "./downloads/proof.bin";
    let download_proof_result = write_guard.download_proof(program_id, input_group_id, Path::new(download_path)).await;
    
    if let Err(received_error) = download_proof_result {
        println!("Error while downloading input group proof");
        return Err(());
    }

    println!("Starting proof verification");

    let execution_args = vec![program_id, input_group_id];

    let output = Command::new("cargo")
        .arg("run")
        .args(execution_args)
        .current_dir("./src/runner")
        .output()
        .expect("Failed to execute child program");

    // println!("Program output: {:?}", output);

    if output.status.success() {
        println!("Proof verified successfully.");
        // println!("Output: {}", String::from_utf8(output.stdout).unwrap());
        // println!("Stderr: {}", String::from_utf8(output.stderr).unwrap());
        write_guard.confirm_proof_validity(program_id, input_group_id).await.expect("Error confirming proof validity");
    } else {
        println!("Process failed.");
        println!("Error output: {}", String::from_utf8(output.stderr).unwrap());
        write_guard.mark_proof_as_invalid(program_id, input_group_id).await.expect("Error while marking proof as invalid");
    }

    let _ = fs::remove_file(download_path);
    return Ok(());
}

pub async fn retrieve_proven_inputs(program_id: &str, limit: usize, page: usize) -> PagedProgramInputGroups {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.get_program_proven_inputs(program_id, Some(limit), Some(page)).await.expect("Error while getting uploaded programs");
}


pub async fn verify_all_program_proven_executions(program_id: &str) {
    let max_page_size = common::config::CONFIG_OBJECT.max_page_size;
    let mut input_groups_page = retrieve_proven_inputs(program_id, max_page_size, 1).await;
    let mut input_groups_array = input_groups_page.program_input_groups;
    while input_groups_array.len() != 0 {
        for input_group_proof in input_groups_array {
            let _ = verify_proven_execution(program_id, &input_group_proof.input_group_id).await;
        }
        input_groups_page = retrieve_proven_inputs(program_id, max_page_size, 1).await;
        input_groups_array = input_groups_page.program_input_groups;
    }
}

pub async fn verify_some_program_proven_executions(program_id: &str, proofs_amount: usize) -> usize {
    let max_page_size = common::config::CONFIG_OBJECT.max_page_size;
    let mut input_groups_page = retrieve_proven_inputs(program_id, max_page_size, 1).await;
    let mut input_groups_array = input_groups_page.program_input_groups;
    let mut verified_proofs = 0;
    while input_groups_array.len() != 0 && verified_proofs < proofs_amount {
        let mut current_page_iterator = 0;
        while current_page_iterator < input_groups_array.len() && verified_proofs < proofs_amount {
            let input_group_proof = &input_groups_array[current_page_iterator];
            if verify_proven_execution(program_id, &input_group_proof.input_group_id).await.is_ok() {
                verified_proofs += 1;
            }
            current_page_iterator += 1;
        }
        input_groups_page = retrieve_proven_inputs(program_id, max_page_size, 1).await;
        input_groups_array = input_groups_page.program_input_groups;
    }
    return verified_proofs;
}

async fn retrieve_my_proven_programs(limit: usize, page: usize) -> PagedPrograms {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.get_my_proven_programs(Some(limit), Some(page)).await.expect("Error while getting uploaded programs");
}

pub async fn verify_some_proven_executions(proofs_amount: usize) {
    let max_page_size = common::config::CONFIG_OBJECT.max_page_size;
    let mut pending_proofs = proofs_amount;
    let proven_programs = retrieve_my_proven_programs(max_page_size, 1).await;
    // verify_some_program_proven_executions(program_id: &str, pending_proofs);
}