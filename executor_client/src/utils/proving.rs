use std::{fs, path::Path, process::Command, time::SystemTime};

use crate::{common::{self, communication::EndpointError}, models::returned_program::ReturnedProgram, services::program_distributor::{PagedPrograms, UploadedProof}};

pub async fn download_and_run_program(program: &ReturnedProgram) -> Result<(), ()> {
    let read_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.read().expect("Error in rw lock");
    let downloaded_files_names_result = read_guard.get_program_and_input_group(&program.program_id).await;

    match downloaded_files_names_result {
        Ok(downloaded_files_names_value) => {
            let csv_file_name = downloaded_files_names_value.input_file_name;
    
            let execution_args = vec![csv_file_name.clone()];
        
            let start_time = SystemTime::now();
        
            let output = Command::new("cargo")
                .arg("run")
                .args(execution_args)
                .current_dir("./src/runner")
                .output()
                .expect("Failed to execute child program");
        
            // println!("Program output: {:?}", output);
        
            let input_group_id = csv_file_name.split(".").collect::<Vec<&str>>()[0];
        
            let _ = fs::remove_file(format!("./program_with_input/{csv_file_name}"));
            let _ = fs::remove_file(format!("./program_with_input/{}", downloaded_files_names_value.program_file_name));
        
            let uploaded_proof_data = UploadedProof {
                organization_id: program.organization_id.clone(),
                program_id: program.program_id.clone(),
                input_group_id: input_group_id.to_string(),
            };
        
            if output.status.success() {
                let after_proof_time = SystemTime::now();
                println!("Proof generated successfully.");
                read_guard.upload_proof(Path::new("./src/runner/proof.bin"), uploaded_proof_data).await.expect("Error uploading proof");
                println!("Proof was uploaded, total seconds passed: {}", after_proof_time.duration_since(start_time).expect("Time went backwards").as_secs());
                let _ = fs::remove_file(format!("./program_with_input/{}", downloaded_files_names_value.program_file_name));
            } else {
                println!("Process failed.");
                println!("Error output: {}", String::from_utf8(output.stderr).unwrap());
            }
            Ok(())
        },
        Err(received_error) => {
            println!("Error while requesting the program with an input group: {:?}", received_error);
            Err(())
        }
    }
}

pub async fn retrieve_programs(organization_option: Option<&str>, limit: Option<usize>, page: Option<usize>) -> PagedPrograms {
    let read_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.read().expect("Error in rw lock");
    let response: Result<PagedPrograms, EndpointError>;
    if organization_option.is_some() {
        let organization_id = organization_option.unwrap();
        response = read_guard.get_organization_programs(&organization_id.to_string(), limit, page).await;
    } else {
        response = read_guard.get_general_programs(limit, page).await;
    }
    if let Err(received_error) = response {
        panic!("Error in programs retrieval: {:?}", received_error);
    }
    return response.unwrap();
}

// Executes all the possible program inputs of each program that is iterated until the
// received amount of program executions are proven
pub async fn run_some_programs(organization_id: Option<&str>, programs_amount: usize) {
    let page_size: usize = common::config::CONFIG_OBJECT.max_page_size;
    let mut programs_page = retrieve_programs(organization_id, Some(page_size), Some(1)).await;
    let mut programs_list = programs_page.programs;
    let mut programs_counter = 0;
    while programs_list.len() != 0 && programs_counter < programs_amount {
        let mut current_page_iterator = 0;
        while current_page_iterator < programs_list.len() && programs_counter < programs_amount {
            let mut keep_same_program = true;
            let returned_program = &programs_list[current_page_iterator];
            while keep_same_program && programs_counter < programs_amount {
                keep_same_program = download_and_run_program(&returned_program).await.is_ok();
                programs_counter += 1;
            }
            current_page_iterator += 1;
        }
        programs_page = retrieve_programs(organization_id, Some(page_size), Some(1)).await;
        programs_list = programs_page.programs;
    }
}

pub async fn run_some_program_inputs(chosen_program: &ReturnedProgram, programs_amount: usize) {
    let mut keep_running_program = true;
    let mut programs_counter = 0;
    while keep_running_program && programs_counter < programs_amount {
        keep_running_program = download_and_run_program(chosen_program).await.is_ok();
        programs_counter += 1;
    }
}
