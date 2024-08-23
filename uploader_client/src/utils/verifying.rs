use std::{fs, io::{self, Write}, path::Path, process::Command, time::SystemTime};

use crate::{common::{self, user_interaction::get_input_string}, services::program_distributor::{PagedProgramInputGroups, PagedPrograms}};

pub async fn verify_proven_execution(program_id: &str, input_group_id: &str) -> Result<bool, ()> {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    // write_guard.download_program(program_id, Path::new("./src/runner/methods")).await.expect("Error downloading program code");
    let download_program_result = write_guard.download_program(program_id, Path::new("./src/runner/methods")).await;
    if let Err(received_error) = download_program_result {
        println!("Error while downloading program code: {:?}", received_error);
        return Err(());
    }

    let download_path = "./downloads/proof.bin";
    let download_proof_result = write_guard.download_proof(program_id, input_group_id, Path::new(download_path)).await;
    
    if let Err(received_error) = download_proof_result {
        println!("Error while downloading input group proof: {:?}", received_error);
        return Err(());
    }

    
    println!("Starting proof verification of program with id \"{}\" with input group with id \"{}\"", program_id, input_group_id);
    let start_time = SystemTime::now();

    let execution_args = vec![program_id, input_group_id];

    // Command added because rust cannot detect accurately the change in the code's files
    Command::new("touch")
        .arg("./methods/guest/src/main.rs")
        .current_dir("./src/runner")
        .output()
        .expect("Failed to execute child program");


    let output = Command::new("cargo")
        .arg("run")
        .args(execution_args)
        .current_dir("./src/runner")
        .output()
        .expect("Failed to execute child program");

    let execution_was_successful;
    if output.status.success() {
        write_guard.confirm_proof_validity(program_id, input_group_id).await.expect("Error confirming proof validity");
        let after_verification_time = SystemTime::now();
        println!("Proof was verified, total seconds passed: {}", after_verification_time.duration_since(start_time).expect("Time went backwards").as_secs());
        println!();
        println!();
        execution_was_successful = true;
    } else {
        println!("Process failed.");
        println!("Error output: {}", String::from_utf8(output.stderr).unwrap());
        write_guard.mark_proof_as_invalid(program_id, input_group_id).await.expect("Error while marking proof as invalid");
        execution_was_successful = false;
    }

    let _ = fs::remove_file(download_path);
    Ok(execution_was_successful)
}

// Returns error if there was an error in the endpoints called, and ok with a boolean indicating if the user wants to continue
// executing the following programs or not
pub async fn interactive_verify_proven_execution(program_id: &str, input_group_id: &str) -> Result<bool, ()> {
    let proof_verified_successfully = verify_proven_execution(program_id, input_group_id).await?;
    if !proof_verified_successfully {
        let mut verify_pending_proofs = None;
        let mut made_a_choice = false;
        while !made_a_choice {
            print!("A problem was found while verifying the downloaded proof, would you like to continue with the pending verifications? (y/n): ");
            io::stdout().flush().unwrap();
            let choice = get_input_string();
            if choice == "y" {
                verify_pending_proofs = Some(true);
                made_a_choice = true;
            } else if choice == "n" {
                verify_pending_proofs = Some(false);
                made_a_choice = true;
            }
        }
        return Ok(verify_pending_proofs.unwrap());
    } else {
        Ok(true)
    }
}

pub async fn retrieve_proven_inputs(program_id: &str, limit: usize, page: usize) -> PagedProgramInputGroups {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.get_program_proven_inputs(program_id, Some(limit), Some(page)).await.expect("Error while getting uploaded programs")
}


pub async fn verify_all_program_proven_executions(program_id: &str) -> bool {
    let max_page_size = common::config::CONFIG_OBJECT.max_page_size;
    let mut input_groups_page = retrieve_proven_inputs(program_id, max_page_size, 1).await;
    let mut input_groups_array = input_groups_page.program_input_groups;
    let mut keep_verifying = true;
    while keep_verifying && !input_groups_array.is_empty() {
        for input_group_proof in input_groups_array {
            // let _ = verify_proven_execution(program_id, &input_group_proof.input_group_id).await;
            let verification_result = interactive_verify_proven_execution(program_id, &input_group_proof.input_group_id).await;
            if let Ok(keep_verifying_selected) = verification_result {
                keep_verifying = keep_verifying_selected;
                if !keep_verifying {
                    break;
                }
            }
        }
        input_groups_page = retrieve_proven_inputs(program_id, max_page_size, 1).await;
        input_groups_array = input_groups_page.program_input_groups;
    }
    return keep_verifying;
}

pub async fn verify_all_proven_executions() {
    let max_page_size = common::config::CONFIG_OBJECT.max_page_size;
    let mut should_continue_verification = true;

    while should_continue_verification {
        let proven_programs_page = retrieve_my_proven_programs(max_page_size, 1).await;
        let proven_programs = proven_programs_page.programs;
        if !proven_programs.is_empty() {
            let program_id = &proven_programs[0].program_id;
            should_continue_verification = verify_all_program_proven_executions(program_id).await;
        } else {
            should_continue_verification = false;
        }
    }
}

pub async fn verify_some_program_proven_executions(program_id: &str, proofs_amount: usize) -> Option<usize> {
    let max_page_size = common::config::CONFIG_OBJECT.max_page_size;
    let mut input_groups_page = retrieve_proven_inputs(program_id, max_page_size, 1).await;
    let mut input_groups_array = input_groups_page.program_input_groups;
    let mut verified_proofs = 0;
    let mut keep_verifying = true;
    while keep_verifying && !input_groups_array.is_empty() && verified_proofs < proofs_amount {
        let mut current_page_iterator = 0;
        while keep_verifying && current_page_iterator < input_groups_array.len() && verified_proofs < proofs_amount {
            let input_group_proof = &input_groups_array[current_page_iterator];
            // if verify_proven_execution(program_id, &input_group_proof.input_group_id).await.is_ok() {
            let verification_result = interactive_verify_proven_execution(program_id, &input_group_proof.input_group_id).await;
            if let Ok(keep_verifying_selected) = verification_result {
                keep_verifying = keep_verifying_selected;
                verified_proofs += 1;
            }
            current_page_iterator += 1;
        }
        input_groups_page = retrieve_proven_inputs(program_id, max_page_size, 1).await;
        input_groups_array = input_groups_page.program_input_groups;
    }
    if keep_verifying {
        Some(verified_proofs)
    } else {
        None
    }
}

pub async fn retrieve_my_proven_programs(limit: usize, page: usize) -> PagedPrograms {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.get_my_proven_programs(Some(limit), Some(page)).await.expect("Error while getting uploaded programs")
}

pub async fn verify_some_proven_executions(proofs_amount: usize) {
    let max_page_size = common::config::CONFIG_OBJECT.max_page_size;
    let mut executed_proofs = 0;
    let mut should_continue_verification = true;

    while should_continue_verification && executed_proofs < proofs_amount {
        let proven_programs_page = retrieve_my_proven_programs(max_page_size, 1).await;
        let proven_programs = proven_programs_page.programs;
        if !proven_programs.is_empty() {
            let program_id = &proven_programs[0].program_id;
            // let last_executed_verifications_amount = verify_some_program_proven_executions(program_id, proofs_amount - executed_proofs).await;
            let some_verifications_result = verify_some_program_proven_executions(program_id, proofs_amount - executed_proofs).await;
            if let Some(last_executed_verifications_amount) = some_verifications_result {
                executed_proofs += last_executed_verifications_amount;
            } else {
                should_continue_verification = false;
            }
        } else {
            should_continue_verification = false;
        }
    }
}